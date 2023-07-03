use async_trait::async_trait;
use hyper::HeaderMap;
use reqwest::Method;

use crate::api::{Result, EMPTY_BODY};
use crate::model::di::common::Status;
use crate::model::di::connector::{DestinationConnector, SourceConnector};
use crate::model::di::destination::{Destination, DestinationSpec};
use crate::model::di::job::Job;
use crate::model::di::source::{Source, SourceSpec};
use crate::model::di::source_metadata::TablesMeta;
use crate::model::di::workflow::{JobPost, Workflow, WorkflowPatch, WorkflowSpec};
use crate::ovhapi::OVHapiV6Client;

#[async_trait]
pub trait DiApi {
    /// List all available source connectors
    async fn di_source_connectors(&self, service_name: &str) -> Result<Vec<SourceConnector>>;
    /// Get a source connector by ID
    async fn di_source_connector(&self, service_name: &str, id: &str) -> Result<SourceConnector>;

    /// List all available destination connectors
    async fn di_destination_connectors(
        &self,
        service_name: &str,
    ) -> Result<Vec<DestinationConnector>>;
    /// Get a destination connector by ID
    async fn di_destination_connector(
        &self,
        service_name: &str,
        id: &str,
    ) -> Result<DestinationConnector>;

    /// List all the sources for a service name
    async fn di_sources(&self, service_name: &str) -> Result<Vec<Source>>;
    /// Get a source by ID
    async fn di_source(&self, service_name: &str, id: &str) -> Result<Source>;
    // Get a source status by ID
    async fn di_source_status(&self, service_name: &str, id: &str) -> Result<Status>;
    /// Get a source metadata by ID
    async fn di_source_metadata(&self, service_name: &str, id: &str) -> Result<TablesMeta>;
    /// Trigger a source metadata extract by ID
    async fn di_source_metadata_post(&self, service_name: &str, id: &str) -> Result<TablesMeta>;
    /// Delete a source by ID
    async fn di_source_delete(&self, service_name: &str, id: &str) -> Result<()>;
    /// Create a new source for a service name
    async fn di_source_post(&self, service_name: &str, spec: &SourceSpec) -> Result<Source>;
    /// Updates a source
    async fn di_source_update(
        &self,
        service_name: &str,
        id: &str,
        spec: &SourceSpec,
    ) -> Result<Source>;

    /// List all the destinations for a service name
    async fn di_destinations(&self, service_name: &str) -> Result<Vec<Destination>>;
    /// Get a destination by ID
    async fn di_destination(&self, service_name: &str, id: &str) -> Result<Destination>;
    /// Get a destination status by ID
    async fn di_destination_status(&self, service_name: &str, id: &str) -> Result<Status>;
    /// Create a new destination for a service name
    async fn di_destination_post(
        &self,
        service_name: &str,
        spec: &DestinationSpec,
    ) -> Result<Destination>;
    /// Delete a destination by ID
    async fn di_destination_delete(&self, service_name: &str, id: &str) -> Result<()>;
    /// Updates a destination
    async fn di_destination_update(
        &self,
        service_name: &str,
        id: &str,
        spec: &DestinationSpec,
    ) -> Result<Destination>;

    /// List all the workflows for a service name
    async fn di_workflows(&self, service_name: &str) -> Result<Vec<Workflow>>;
    /// Get a workflow by ID
    async fn di_workflow(&self, service_name: &str, id: &str) -> Result<Workflow>;
    /// Create a new workflow for a service name
    async fn di_workflow_post(&self, service_name: &str, spec: &WorkflowSpec) -> Result<Workflow>;
    /// Delete a workflow by ID
    async fn di_workflow_delete(&self, service_name: &str, id: &str) -> Result<()>;
    /// Update a workflow
    async fn di_workflow_put(
        &self,
        service_name: &str,
        id: &str,
        spec: &WorkflowPatch,
    ) -> Result<Workflow>;

    /// List all the jobs for a service name
    async fn di_jobs(&self, service_name: &str, workflow_id: &str) -> Result<Vec<Job>>;
    /// Get a job by ID
    async fn di_job(&self, service_name: &str, id: &str, workflow_id: &str) -> Result<Job>;
    /// Start a new job for a workflow
    async fn di_job_post(&self, service_name: &str, workflow_id: &str) -> Result<Job>;
    /// Stop a job for a workflow
    async fn di_job_delete(&self, service_name: &str, workflow_id: &str, id: &str) -> Result<()>;
}

#[async_trait]
impl DiApi for OVHapiV6Client {
    async fn di_source_connectors(&self, service_name: &str) -> Result<Vec<SourceConnector>> {
        let request = self
            .build_request(
                Method::GET,
                &[
                    "cloud",
                    "project",
                    service_name,
                    "dataIntegration",
                    "sourceConnectors",
                ],
                &[],
                &HeaderMap::new(),
                EMPTY_BODY,
            )
            .await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_source_connector(&self, service_name: &str, id: &str) -> Result<SourceConnector> {
        let request = self
            .build_request(
                Method::GET,
                &[
                    "cloud",
                    "project",
                    service_name,
                    "dataIntegration",
                    "sourceConnectors",
                    id,
                ],
                &[],
                &HeaderMap::new(),
                EMPTY_BODY,
            )
            .await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_destination_connectors(
        &self,
        service_name: &str,
    ) -> Result<Vec<DestinationConnector>> {
        let request = self
            .build_request(
                Method::GET,
                &[
                    "cloud",
                    "project",
                    service_name,
                    "dataIntegration",
                    "destinationConnectors",
                ],
                &[],
                &HeaderMap::new(),
                EMPTY_BODY,
            )
            .await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_destination_connector(
        &self,
        service_name: &str,
        id: &str,
    ) -> Result<DestinationConnector> {
        let request = self
            .build_request(
                Method::GET,
                &[
                    "cloud",
                    "project",
                    service_name,
                    "dataIntegration",
                    "destinationConnectors",
                    id,
                ],
                &[],
                &HeaderMap::new(),
                EMPTY_BODY,
            )
            .await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_sources(&self, service_name: &str) -> Result<Vec<Source>> {
        let request = self
            .build_request(
                Method::GET,
                &[
                    "cloud",
                    "project",
                    service_name,
                    "dataIntegration",
                    "sources",
                ],
                &[],
                &HeaderMap::new(),
                EMPTY_BODY,
            )
            .await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_source(&self, service_name: &str, id: &str) -> Result<Source> {
        let request = self
            .build_request(
                Method::GET,
                &[
                    "cloud",
                    "project",
                    service_name,
                    "dataIntegration",
                    "sources",
                    id,
                ],
                &[],
                &HeaderMap::new(),
                EMPTY_BODY,
            )
            .await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_source_status(&self, service_name: &str, id: &str) -> Result<Status> {
        let request = self
            .build_request(
                Method::GET,
                &[
                    "cloud",
                    "project",
                    service_name,
                    "dataIntegration",
                    "sources",
                    id,
                    "connection",
                ],
                &[],
                &HeaderMap::new(),
                EMPTY_BODY,
            )
            .await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_source_metadata(&self, service_name: &str, id: &str) -> Result<TablesMeta> {
        let request = self
            .build_request(
                Method::GET,
                &[
                    "cloud",
                    "project",
                    service_name,
                    "dataIntegration",
                    "sources",
                    id,
                    "metadata",
                ],
                &[],
                &HeaderMap::new(),
                EMPTY_BODY,
            )
            .await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_source_metadata_post(&self, service_name: &str, id: &str) -> Result<TablesMeta> {
        let request = self
            .build_request(
                Method::POST,
                &[
                    "cloud",
                    "project",
                    service_name,
                    "dataIntegration",
                    "sources",
                    id,
                    "metadata",
                ],
                &[],
                &HeaderMap::new(),
                EMPTY_BODY,
            )
            .await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_source_delete(&self, service_name: &str, id: &str) -> Result<()> {
        let request = self
            .build_request(
                Method::DELETE,
                &[
                    "cloud",
                    "project",
                    service_name,
                    "dataIntegration",
                    "sources",
                    id,
                ],
                &[],
                &HeaderMap::new(),
                EMPTY_BODY,
            )
            .await?;
        request.send(&self.client, &[]).await?;
        Ok(())
    }

    async fn di_source_post(&self, service_name: &str, spec: &SourceSpec) -> Result<Source> {
        let request = self
            .build_request(
                Method::POST,
                &[
                    "cloud",
                    "project",
                    service_name,
                    "dataIntegration",
                    "sources",
                ],
                &[],
                &HeaderMap::new(),
                Some(spec),
            )
            .await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_source_update(
        &self,
        service_name: &str,
        id: &str,
        spec: &SourceSpec,
    ) -> Result<Source> {
        let request = self
            .build_request(
                Method::PUT,
                &[
                    "cloud",
                    "project",
                    service_name,
                    "dataIntegration",
                    "sources",
                    id,
                ],
                &[],
                &HeaderMap::new(),
                Some(spec),
            )
            .await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_destinations(&self, service_name: &str) -> Result<Vec<Destination>> {
        let request = self
            .build_request(
                Method::GET,
                &[
                    "cloud",
                    "project",
                    service_name,
                    "dataIntegration",
                    "destinations",
                ],
                &[],
                &HeaderMap::new(),
                EMPTY_BODY,
            )
            .await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_destination(&self, service_name: &str, id: &str) -> Result<Destination> {
        let request = self
            .build_request(
                Method::GET,
                &[
                    "cloud",
                    "project",
                    service_name,
                    "dataIntegration",
                    "destinations",
                    id,
                ],
                &[],
                &HeaderMap::new(),
                EMPTY_BODY,
            )
            .await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_destination_status(&self, service_name: &str, id: &str) -> Result<Status> {
        let request = self
            .build_request(
                Method::GET,
                &[
                    "cloud",
                    "project",
                    service_name,
                    "dataIntegration",
                    "destinations",
                    id,
                    "connection",
                ],
                &[],
                &HeaderMap::new(),
                EMPTY_BODY,
            )
            .await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_destination_post(
        &self,
        service_name: &str,
        spec: &DestinationSpec,
    ) -> Result<Destination> {
        let request = self
            .build_request(
                Method::POST,
                &[
                    "cloud",
                    "project",
                    service_name,
                    "dataIntegration",
                    "destinations",
                ],
                &[],
                &HeaderMap::new(),
                Some(spec),
            )
            .await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_destination_delete(&self, service_name: &str, id: &str) -> Result<()> {
        let request = self
            .build_request(
                Method::DELETE,
                &[
                    "cloud",
                    "project",
                    service_name,
                    "dataIntegration",
                    "destinations",
                    id,
                ],
                &[],
                &HeaderMap::new(),
                EMPTY_BODY,
            )
            .await?;
        request.send(&self.client, &[]).await?;
        Ok(())
    }

    async fn di_destination_update(
        &self,
        service_name: &str,
        id: &str,
        spec: &DestinationSpec,
    ) -> Result<Destination> {
        let request = self
            .build_request(
                Method::PUT,
                &[
                    "cloud",
                    "project",
                    service_name,
                    "dataIntegration",
                    "destinations",
                    id,
                ],
                &[],
                &HeaderMap::new(),
                Some(spec),
            )
            .await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_workflows(&self, service_name: &str) -> Result<Vec<Workflow>> {
        let request = self
            .build_request(
                Method::GET,
                &[
                    "cloud",
                    "project",
                    service_name,
                    "dataIntegration",
                    "workflows",
                ],
                &[],
                &HeaderMap::new(),
                EMPTY_BODY,
            )
            .await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_workflow(&self, service_name: &str, id: &str) -> Result<Workflow> {
        let request = self
            .build_request(
                Method::GET,
                &[
                    "cloud",
                    "project",
                    service_name,
                    "dataIntegration",
                    "workflows",
                    id,
                ],
                &[],
                &HeaderMap::new(),
                EMPTY_BODY,
            )
            .await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_workflow_post(&self, service_name: &str, spec: &WorkflowSpec) -> Result<Workflow> {
        let request = self
            .build_request(
                Method::POST,
                &[
                    "cloud",
                    "project",
                    service_name,
                    "dataIntegration",
                    "workflows",
                ],
                &[],
                &HeaderMap::new(),
                Some(spec),
            )
            .await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_workflow_delete(&self, service_name: &str, id: &str) -> Result<()> {
        let request = self
            .build_request(
                Method::DELETE,
                &[
                    "cloud",
                    "project",
                    service_name,
                    "dataIntegration",
                    "workflows",
                    id,
                ],
                &[],
                &HeaderMap::new(),
                EMPTY_BODY,
            )
            .await?;
        request.send(&self.client, &[]).await?;
        Ok(())
    }

    async fn di_workflow_put(
        &self,
        service_name: &str,
        id: &str,
        spec: &WorkflowPatch,
    ) -> Result<Workflow> {
        let request = self
            .build_request(
                Method::PUT,
                &[
                    "cloud",
                    "project",
                    service_name,
                    "dataIntegration",
                    "workflows",
                    id,
                ],
                &[],
                &HeaderMap::new(),
                Some(spec),
            )
            .await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_jobs(&self, service_name: &str, workflow_id: &str) -> Result<Vec<Job>> {
        let request = self
            .build_request(
                Method::GET,
                &[
                    "cloud",
                    "project",
                    service_name,
                    "dataIntegration",
                    "workflows",
                    workflow_id,
                    "jobs",
                ],
                &[],
                &HeaderMap::new(),
                EMPTY_BODY,
            )
            .await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_job(&self, service_name: &str, workflow_id: &str, id: &str) -> Result<Job> {
        let request = self
            .build_request(
                Method::GET,
                &[
                    "cloud",
                    "project",
                    service_name,
                    "dataIntegration",
                    "workflows",
                    workflow_id,
                    "jobs",
                    id,
                ],
                &[],
                &HeaderMap::new(),
                EMPTY_BODY,
            )
            .await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_job_post(&self, service_name: &str, workflow_id: &str) -> Result<Job> {
        let job = JobPost { parameters: vec![] };
        let request = self
            .build_request(
                Method::POST,
                &[
                    "cloud",
                    "project",
                    service_name,
                    "dataIntegration",
                    "workflows",
                    workflow_id,
                    "jobs",
                ],
                &[],
                &HeaderMap::new(),
                Some(&job),
            )
            .await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_job_delete(&self, service_name: &str, workflow_id: &str, id: &str) -> Result<()> {
        let request = self
            .build_request(
                Method::DELETE,
                &[
                    "cloud",
                    "project",
                    service_name,
                    "dataIntegration",
                    "workflows",
                    workflow_id,
                    "jobs",
                    id,
                ],
                &[],
                &HeaderMap::new(),
                EMPTY_BODY,
            )
            .await?;
        request.send(&self.client, &[]).await?;
        Ok(())
    }
}
