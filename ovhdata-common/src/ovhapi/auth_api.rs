use async_trait::async_trait;
use hyper::HeaderMap;
use reqwest::Method;

use crate::api::Result;
use crate::api::EMPTY_BODY;
use crate::model::auth::CredentialDetails;
use crate::model::me::Me;
use crate::ovhapi::OVHapiV6Client;

#[async_trait]
pub trait AuthApi {
    /// Information about the logged user
    async fn me(&self) -> Result<Me>;

    async fn current_credential(&self) -> Result<CredentialDetails>;
}

#[async_trait]
impl AuthApi for OVHapiV6Client {
    async fn me(&self) -> Result<Me> {
        let request = self
            .build_request(
                Method::GET,
                &["auth", "details"],
                &[],
                &HeaderMap::new(),
                EMPTY_BODY,
            )
            .await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn current_credential(&self) -> Result<CredentialDetails> {
        let request = self
            .build_request(
                Method::GET,
                &["auth", "currentCredential"],
                &[],
                &HeaderMap::new(),
                EMPTY_BODY,
            )
            .await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }
}
