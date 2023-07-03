use async_trait::async_trait;
use futures::future::try_join_all;
use hyper::HeaderMap;
use reqwest::Method;

use crate::api::Result;
use crate::api::EMPTY_BODY;
use crate::model::project::Project;
use crate::ovhapi::OVHapiV6Client;

#[async_trait]
pub trait ProjectApi {
    /// Information about the cloud projects
    async fn projects(&self) -> Result<Vec<Project>>;
    async fn project(&self, service_name: &str) -> Result<Project>;
    async fn project_list(&self) -> Result<Vec<String>>;
}

#[async_trait]
impl ProjectApi for OVHapiV6Client {
    async fn projects(&self) -> Result<Vec<Project>> {
        let projects = self.project_list().await?;
        let result = try_join_all(projects.iter().map(|s: &String| self.project(s))).await?;
        Ok(result)
    }

    async fn project(&self, service_name: &str) -> Result<Project> {
        let request = self
            .build_request(
                Method::GET,
                &["cloud", "project", service_name],
                &[],
                &HeaderMap::new(),
                EMPTY_BODY,
            )
            .await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn project_list(&self) -> Result<Vec<String>> {
        let request = self
            .build_request(
                Method::GET,
                &["cloud", "project"],
                &[],
                &HeaderMap::new(),
                EMPTY_BODY,
            )
            .await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }
}
