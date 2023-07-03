extern crate serde;
extern crate serde_json;

use hyper::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, USER_AGENT};
use reqwest::{Client, Method};
use serde::Serialize;
use sha1::{Digest, Sha1};
use std::str::FromStr;

use uuid::Uuid;

use crate::api::{Error, RequestWrapper, Result, EMPTY_BODY};
use crate::utils::http::Url;

// Required headers for auth
pub const HEADER_OVH_APPLICATION: &str = "X-Ovh-Application";
pub const HEADER_OVH_TIMESTAMP: &str = "X-Ovh-Timestamp";
pub const HEADER_OVH_SIGNATURE: &str = "X-Ovh-Signature";
pub const HEADER_OVH_CONSUMER: &str = "X-Ovh-Consumer";

#[derive(Debug, Clone)]
pub struct OVHapiV6Client {
    endpoint_url: String,
    application_key: String,
    application_secret: String,
    consumer_key: String,
    pub client: Client,
}

impl OVHapiV6Client {
    /// Initialize a new `ovh api client ` from default path a App Key, App secret, Consumer key.
    pub fn new(
        endpoint_url: String,
        application_key: String,
        application_secret: String,
        consumer_key: String,
    ) -> Self {
        Self {
            endpoint_url,
            application_key,
            application_secret,
            consumer_key,
            client: Client::new(),
        }
    }

    /// Compute signature for OVH.
    fn build_sig(
        method: &str,
        query: &str,
        body: &str,
        timestamp: &str,
        aas: &str,
        ck: &str,
    ) -> String {
        let sep = "+";
        let prefix = "$1$".to_string();

        let capacity = 1
            + aas.len()
            + sep.len()
            + ck.len()
            + method.len()
            + sep.len()
            + query.len()
            + sep.len()
            + body.len()
            + sep.len()
            + timestamp.len();
        let mut signature = String::with_capacity(capacity);
        signature.push_str(aas);
        signature.push_str(sep);
        signature.push_str(ck);
        signature.push_str(sep);
        signature.push_str(method);
        signature.push_str(sep);
        signature.push_str(query);
        signature.push_str(sep);
        signature.push_str(body);
        signature.push_str(sep);
        signature.push_str(timestamp);

        // debug!("Signature: {}", &signature);
        let mut hasher = Sha1::new();
        hasher.update(signature);
        let hash_result = hasher.finalize();
        let hash_hexstr = format!("{:x}", hash_result);

        prefix + &hash_hexstr
    }

    /// Ask time to OVH API server to compute delta time
    async fn remote_time(&self) -> Result<u64> {
        let request = self
            .build_request_without_authent(
                Method::GET,
                &["auth", "time"],
                &[],
                &HeaderMap::new(),
                EMPTY_BODY,
            )
            .await?;
        let response = request.send(&self.client, &[]);
        let time = (response.await?.parse::<u64>().await).unwrap_or(1);
        Ok(time)
    }

    /// Start a client request with given method
    /// Use Hyper client
    pub async fn build_request<T>(
        &self,
        method: Method,
        path: &[&str],
        query: &[(String, String)],
        headers: &HeaderMap,
        body: Option<&T>,
    ) -> Result<RequestWrapper>
    where
        T: Serialize,
    {
        // compute time delta
        let computed_time = self.remote_time().await?;
        let timestamp = computed_time.to_string();

        // build the request
        let url = Url::from_str(self.endpoint_url.as_str())
            .unwrap()
            .with_segment(path);
        let result_request_wrapper = self
            .build_request_with_full_url(method.to_owned(), &url, query, headers, body)
            .await;
        let mut request = result_request_wrapper.unwrap();

        // build the ovhapi signature
        let sign = OVHapiV6Client::build_sig(
            method.to_owned().as_str(),
            request.url().as_str(),
            &request.body_str(),
            &timestamp,
            self.application_secret.as_str(),
            self.consumer_key.as_str(),
        );

        // Adds ovhapiv6 authentication headers
        let headers = request.headers_mut();
        headers.insert(
            HEADER_OVH_APPLICATION,
            HeaderValue::from_str(self.application_key.as_str()).unwrap(),
        );
        headers.insert(
            HEADER_OVH_CONSUMER,
            HeaderValue::from_str(self.consumer_key.as_str()).unwrap(),
        );
        headers.insert(
            HEADER_OVH_TIMESTAMP,
            HeaderValue::from_str(timestamp.as_str()).unwrap(),
        );
        headers.insert(
            HEADER_OVH_SIGNATURE,
            HeaderValue::from_str(sign.as_str()).unwrap(),
        );
        headers.insert(
            ACCEPT,
            HeaderValue::from_str("application/json; charset=utf-8").unwrap(),
        );
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_str("application/json; charset=utf-8").unwrap(),
        );

        Ok(request)
    }

    async fn build_request_without_authent<T>(
        &self,
        method: Method,
        path: &[&str],
        query: &[(String, String)],
        headers: &HeaderMap,
        body: Option<&T>,
    ) -> Result<RequestWrapper>
    where
        T: Serialize,
    {
        let url = Url::from_str(self.endpoint_url.as_str())
            .unwrap()
            .with_segment(path);
        self.build_request_with_full_url(method, &url, query, headers, body)
            .await
    }

    async fn build_request_with_full_url<T>(
        &self,
        method: Method,
        url: &Url,
        query: &[(String, String)],
        headers: &HeaderMap,
        body: Option<&T>,
    ) -> Result<RequestWrapper>
    where
        T: Serialize,
    {
        let request_id = Uuid::new_v4();

        let mut request_builder = self
            .client
            .request(method, url.to_string())
            .headers(HeaderMap::to_owned(headers))
            .header(USER_AGENT, "OVH-DATA-CLI/hyper/0.14");

        request_builder = request_builder.query(&query);

        if let Some(body) = body {
            request_builder = request_builder.json(body);
        }

        let request = request_builder.build().map_err(Error::Request)?;
        let request_wrapper = RequestWrapper::from(request_id, request);

        Ok(request_wrapper)
    }
}
