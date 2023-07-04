use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

use reqwest::header::HeaderMap;
use reqwest::{Client, Request, Response, StatusCode};
use serde::de::DeserializeOwned;

use tracing::{debug, error, info};
use uuid::Uuid;

use crate::api::{Error, Result};
use crate::model::utils::ResponseError;
use crate::REQUEST_ID;

pub const EMPTY_BODY: Option<&()> = None;

/// Deserialize response body as JSON
pub async fn parse_response<T>(response: ResponseWrapper) -> Result<T>
where
    T: DeserializeOwned,
{
    let body_string = response.body_text().await;
    serde_json::from_str(body_string.as_str()).map_err(|e| Error::DeserializeContent(e, body_string))
}

/// Send HTTP request with optional body and return a response
#[tracing::instrument(
    skip(client, request, allowed_statuses),
    fields(
        request_id = %request.request_id(),
        method = %request.method(),
        url = %request.url(),
    )
)]
pub async fn send_request(client: &Client, request: RequestWrapper, allowed_statuses: &[StatusCode]) -> Result<ResponseWrapper> {
    let request_id = *request.request_id();
    let request_method = request.method().clone();
    info!(
        http = "request",
        headers = %request.headers(),
        body = %request.body_str(),
        "SEND {} {} {}={}", request.method(), request.url(), REQUEST_ID, request.request_id()
    );
    let response = match client.execute(request.into()).await {
        Ok(response) => {
            let status = response.status();
            let response = ResponseWrapper::from(request_id, response);
            info!(
                http = "response",
                status_int = %response.status().as_u16(),
                headers = %response.headers_map_wrapper(),
                "[{:6}] {} {} {}={}", response.status(), request_method, response.url(), REQUEST_ID, response.request_id
            );
            // Handle response status
            if status.is_success() || allowed_statuses.contains(&status) {
                Ok(response)
            } else {
                let body_string = response.body_text().await;

                let body_string = match serde_json::from_str::<ResponseError>(body_string.as_str()) {
                    Ok(response_error) => response_error.message,
                    Err(_) => body_string,
                };
                Err(Error::Response(status, body_string))
            }
        }
        Err(err) => {
            error!(error = %err, "[KO] Was unable to execute request !");
            Err(Error::Request(err))
        }
    }?;

    Ok(response)
}

pub struct RequestWrapper(Uuid, Request);

impl RequestWrapper {
    pub fn from(uuid: Uuid, request: Request) -> Self {
        RequestWrapper(uuid, request)
    }

    pub fn headers(&self) -> HeaderMapWrapper {
        HeaderMapWrapper::from(self.1.headers().clone())
    }

    pub fn body_str(&self) -> String {
        if let Some(body) = self.body() {
            if let Some(bytes) = body.as_bytes() {
                let utf8 = std::str::from_utf8(bytes).unwrap_or("<error decoding body as utf8>");
                String::from(utf8)
            } else {
                String::from("<stream data>")
            }
        } else {
            String::default()
        }
    }

    pub fn request_id(&self) -> &Uuid {
        &self.0
    }

    pub async fn send(self, client: &Client, allowed_status: &[StatusCode]) -> Result<ResponseWrapper> {
        send_request(client, self, allowed_status).await
    }
}

impl Deref for RequestWrapper {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl DerefMut for RequestWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.1
    }
}

impl From<RequestWrapper> for Request {
    fn from(val: RequestWrapper) -> Self {
        val.1
    }
}

pub struct ResponseWrapper {
    request_id: Uuid,
    response: Response,
}

impl ResponseWrapper {
    pub fn headers_map_wrapper(&self) -> HeaderMapWrapper {
        self.headers().clone().into()
    }

    #[tracing::instrument(skip(self), fields(request_id = %self.request_id, http = "response"))]
    pub async fn body_text(self) -> String {
        let response_string = self.response.text().await.unwrap_or("".to_string());
        debug!(body = %response_string ,"Response body read as text !");
        response_string
    }

    pub fn response(self) -> Response {
        self.response
    }

    pub fn from(request_id: Uuid, response: Response) -> Self {
        Self { request_id, response }
    }
    pub async fn parse<T: DeserializeOwned>(self) -> Result<T> {
        parse_response::<T>(self).await
    }
}

impl Deref for ResponseWrapper {
    type Target = Response;
    fn deref(&self) -> &Self::Target {
        &self.response
    }
}

pub struct HeaderMapWrapper(HeaderMap);

impl Deref for HeaderMapWrapper {
    type Target = HeaderMap;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<HeaderMap> for HeaderMapWrapper {
    fn from(header_map: HeaderMap) -> Self {
        HeaderMapWrapper(header_map)
    }
}
impl Display for HeaderMapWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = self
            .0
            .iter()
            .map(|(name, value)| format!("{:?}: {:?}", name, value))
            .collect::<Vec<String>>()
            .join("; ");
        f.write_str(string.as_str())
    }
}
