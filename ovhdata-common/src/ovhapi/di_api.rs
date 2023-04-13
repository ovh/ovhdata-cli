use async_trait::async_trait;
use hyper::HeaderMap;
use reqwest::{Method};

use crate::api::{EMPTY_BODY, Result};
use crate::model::di::common::Status;
use crate::model::di::connector::{DestinationConnector, SourceConnector};
use crate::model::di::source::{Source, SourceSpec};
use crate::model::di::source_metadata::{TablesMeta};
use crate::model::di::destination::{Destination, DestinationSpec};
use crate::model::di::job::Job;
use crate::model::di::workflow::{WorkflowSpec, Workflow, WorkflowPatch, JobPost};
use crate::ovhapi::OVHapiV6Client;



#[async_trait]
pub trait DiApi {
    /// List all available source connectors
    async fn di_source_connectors(&self, service_name: &String) -> Result<Vec<SourceConnector>>;
    /// Get a source connector by ID
    async fn di_source_connector(&self, service_name: &String, id: &String) -> Result<SourceConnector>;

    /// List all available destination connectors
    async fn di_destination_connectors(&self, service_name: &String) -> Result<Vec<DestinationConnector>>;
    /// Get a destination connector by ID
    async fn di_destination_connector(&self, service_name: &String, id: &String) -> Result<DestinationConnector>;

    /// List all the sources for a service name
    async fn di_sources(&self, service_name: &String) -> Result<Vec<Source>>;
    /// Get a source by ID
    async fn di_source(&self, service_name: &String, id: &String) -> Result<Source>;
    // Get a source status by ID
    async fn di_source_status(&self, service_name: &String, id: &String) -> Result<Status>;
    /// Get a source metadata by ID
    async fn di_source_metadata(&self, service_name: &String, id: &String) -> Result<TablesMeta>;
    /// Trigger a source metadata extract by ID
    async fn di_source_metadata_post(&self, service_name: &String, id: &String) -> Result<TablesMeta>;
    /// Delete a source by ID
    async fn di_source_delete(&self, service_name: &String, id: &String) -> Result<()>;
    /// Create a new source for a service name
    async fn di_source_post(&self, service_name: &String, spec: &SourceSpec) -> Result<Source>;
    /// Updates a source
    async fn di_source_update(&self, service_name: &String, id: &String, spec: &SourceSpec) -> Result<Source>;

    /// List all the destinations for a service name
    async fn di_destinations(&self, service_name: &String) -> Result<Vec<Destination>>;
    /// Get a destination by ID
    async fn di_destination(&self, service_name: &String, id: &String) -> Result<Destination>;
    /// Get a destination status by ID
    async fn di_destination_status(&self, service_name: &String, id: &String) -> Result<Status>;
    /// Create a new destination for a service name
    async fn di_destination_post(&self, service_name: &String, spec: &DestinationSpec) -> Result<Destination>;
    /// Delete a destination by ID
    async fn di_destination_delete(&self, service_name: &String, id: &String) -> Result<()>;
    /// Updates a destination
    async fn di_destination_update(&self, service_name: &String, id: &String, spec: &DestinationSpec) -> Result<Destination>;

    /// List all the workflows for a service name
    async fn di_workflows(&self, service_name: &String) -> Result<Vec<Workflow>>;
    /// Get a workflow by ID
    async fn di_workflow(&self, service_name: &String, id: &String) -> Result<Workflow>;
    /// Create a new workflow for a service name
    async fn di_workflow_post(&self, service_name: &String, spec: &WorkflowSpec) -> Result<Workflow>;
    /// Delete a workflow by ID
    async fn di_workflow_delete(&self, service_name: &String, id: &String) -> Result<()>;
    /// Update a workflow
    async fn di_workflow_put(&self, service_name: &String, id: &String, spec: &WorkflowPatch) -> Result<Workflow>;

    /// List all the jobs for a service name
    async fn di_jobs(&self, service_name: &String, workflow_id: &String) -> Result<Vec<Job>>;
    /// Get a job by ID
    async fn di_job(&self, service_name: &String, id: &String, workflow_id: &String) -> Result<Job>;
    /// Start a new job for a workflow
    async fn di_job_post(&self, service_name: &String, workflow_id: &String) -> Result<Job>;
    /// Stop a job for a workflow
    async fn di_job_delete(&self, service_name: &String, workflow_id: &String, id: &String) -> Result<()>;
}

#[async_trait]
impl DiApi for OVHapiV6Client {
    async fn di_source_connectors(&self, service_name: &String) -> Result<Vec<SourceConnector>> {
        let request = self.build_request(Method::GET, &["cloud", "project", service_name.as_str(), "dataIntegration", "sourceConnectors"], &[], &HeaderMap::new(), EMPTY_BODY).await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_source_connector(&self, service_name: &String, id: &String) -> Result<SourceConnector> {
        let request = self.build_request(Method::GET, &["cloud", "project", service_name.as_str(), "dataIntegration", "sourceConnectors", id.as_str()], &[], &HeaderMap::new(), EMPTY_BODY).await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_destination_connectors(&self, service_name: &String) -> Result<Vec<DestinationConnector>> {
        let request = self.build_request(Method::GET, &["cloud", "project", service_name.as_str(), "dataIntegration", "destinationConnectors"], &[], &HeaderMap::new(), EMPTY_BODY).await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_destination_connector(&self, service_name: &String, id: &String) -> Result<DestinationConnector> {
        let request = self.build_request(Method::GET, &["cloud", "project", service_name.as_str(), "dataIntegration", "destinationConnectors", id.as_str()], &[], &HeaderMap::new(), EMPTY_BODY).await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_sources(&self, service_name: &String) -> Result<Vec<Source>> {
        let request = self.build_request(Method::GET, &["cloud", "project", service_name.as_str(), "dataIntegration", "sources"], &[], &HeaderMap::new(), EMPTY_BODY).await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_source(&self, service_name: &String, id: &String) -> Result<Source> {
        let request = self.build_request(Method::GET, &["cloud", "project", service_name.as_str(), "dataIntegration", "sources", id.as_str()], &[], &HeaderMap::new(), EMPTY_BODY).await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_source_status(&self, service_name: &String, id: &String) -> Result<Status> {
        let request = self.build_request(Method::GET, &["cloud", "project", service_name.as_str(), "dataIntegration", "sources", id.as_str(), "connection"], &[], &HeaderMap::new(), EMPTY_BODY).await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_source_metadata(&self, service_name: &String, id: &String) -> Result<TablesMeta> {
        let request = self.build_request(Method::GET, &["cloud", "project", service_name.as_str(), "dataIntegration", "sources", id.as_str(), "metadata"], &[], &HeaderMap::new(), EMPTY_BODY).await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_source_metadata_post(&self, service_name: &String, id: &String) -> Result<TablesMeta> {
        let request = self.build_request(Method::POST, &["cloud", "project", service_name.as_str(), "dataIntegration", "sources", id.as_str(), "metadata"], &[], &HeaderMap::new(), EMPTY_BODY).await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_source_delete(&self, service_name: &String, id: &String) -> Result<()> {
        let request = self.build_request(Method::DELETE, &["cloud", "project", service_name.as_str(), "dataIntegration", "sources", id.as_str()], &[], &HeaderMap::new(), EMPTY_BODY).await?;
        request.send(&self.client, &[]).await?;
        Ok(())
    }

    async fn di_source_post(&self, service_name: &String, spec: &SourceSpec) -> Result<Source> {
        let request = self.build_request(Method::POST, &["cloud", "project", service_name.as_str(), "dataIntegration", "sources"], &[], &HeaderMap::new(), Some(spec)).await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_source_update(&self, service_name: &String, id: &String, spec: &SourceSpec) -> Result<Source> {
        let request = self.build_request(Method::PUT, &["cloud", "project", service_name.as_str(), "dataIntegration", "sources", id.as_str()], &[], &HeaderMap::new(), Some(spec)).await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_destinations(&self, service_name: &String) -> Result<Vec<Destination>> {
        let request = self.build_request(Method::GET, &["cloud", "project", service_name.as_str(), "dataIntegration", "destinations"], &[], &HeaderMap::new(), EMPTY_BODY).await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_destination(&self, service_name: &String, id: &String) -> Result<Destination> {
        let request = self.build_request(Method::GET, &["cloud", "project", service_name.as_str(), "dataIntegration", "destinations", id.as_str()], &[], &HeaderMap::new(), EMPTY_BODY).await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_destination_status(&self, service_name: &String, id: &String) -> Result<Status> {
        let request = self.build_request(Method::GET, &["cloud", "project", service_name.as_str(), "dataIntegration", "destinations", id.as_str(), "connection"], &[], &HeaderMap::new(), EMPTY_BODY).await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_destination_post(&self, service_name: &String, spec: &DestinationSpec) -> Result<Destination> {
        let request = self.build_request(Method::POST, &["cloud", "project", service_name.as_str(), "dataIntegration", "destinations"], &[], &HeaderMap::new(), Some(spec)).await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_destination_delete(&self, service_name: &String, id: &String) -> Result<()> {
        let request = self.build_request(Method::DELETE, &["cloud", "project", service_name.as_str(), "dataIntegration", "destinations", id.as_str()], &[], &HeaderMap::new(), EMPTY_BODY).await?;
        request.send(&self.client, &[]).await?;
        Ok(())
    }

    async fn di_destination_update(&self, service_name: &String, id: &String, spec: &DestinationSpec) -> Result<Destination> {
        let request = self.build_request(Method::PUT, &["cloud", "project", service_name.as_str(), "dataIntegration", "destinations", id.as_str()], &[], &HeaderMap::new(), Some(spec)).await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_workflows(&self, service_name: &String) -> Result<Vec<Workflow>> {
        let request = self.build_request(Method::GET, &["cloud", "project", service_name.as_str(), "dataIntegration", "workflows"], &[], &HeaderMap::new(), EMPTY_BODY).await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_workflow(&self, service_name: &String, id: &String) -> Result<Workflow> {
        let request = self.build_request(Method::GET, &["cloud", "project", service_name.as_str(), "dataIntegration", "workflows", id.as_str()], &[], &HeaderMap::new(), EMPTY_BODY).await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_workflow_post(&self, service_name: &String, spec: &WorkflowSpec) -> Result<Workflow> {
        let request = self.build_request(Method::POST, &["cloud", "project", service_name.as_str(), "dataIntegration", "workflows"], &[], &HeaderMap::new(), Some(spec)).await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_workflow_delete(&self, service_name: &String, id: &String) -> Result<()> {
        let request = self.build_request(Method::DELETE, &["cloud", "project", service_name.as_str(), "dataIntegration", "workflows", id.as_str()], &[], &HeaderMap::new(), EMPTY_BODY).await?;
        request.send(&self.client, &[]).await?;
        Ok(())
    }

    async fn di_workflow_put(&self, service_name: &String, id: &String, spec: &WorkflowPatch) -> Result<Workflow> {
        let request = self.build_request(Method::PUT, &["cloud", "project", service_name.as_str(), "dataIntegration", "workflows", id.as_str()], &[], &HeaderMap::new(), Some(spec)).await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_jobs(&self, service_name: &String, workflow_id: &String) -> Result<Vec<Job>> {
        let request = self.build_request(Method::GET, &["cloud", "project", service_name.as_str(), "dataIntegration", "workflows", workflow_id.as_str(), "jobs"], &[], &HeaderMap::new(), EMPTY_BODY).await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_job(&self, service_name: &String, workflow_id: &String, id: &String) -> Result<Job> {
        let request = self.build_request(Method::GET, &["cloud", "project", service_name.as_str(), "dataIntegration", "workflows", workflow_id.as_str(), "jobs", id.as_str()], &[], &HeaderMap::new(), EMPTY_BODY).await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_job_post(&self, service_name: &String, workflow_id: &String) -> Result<Job> {
        let job = JobPost { parameters: vec![] };
        let request = self.build_request(Method::POST, &["cloud", "project", service_name.as_str(), "dataIntegration", "workflows", workflow_id.as_str(), "jobs"], &[], &HeaderMap::new(), Some(&job)).await?;
        let response = request.send(&self.client, &[]).await?;
        response.parse().await
    }

    async fn di_job_delete(&self, service_name: &String, workflow_id: &String, id: &String) -> Result<()> {
        let request = self.build_request(Method::DELETE, &["cloud", "project", service_name.as_str(), "dataIntegration", "workflows", workflow_id.as_str(), "jobs", id.as_str()], &[], &HeaderMap::new(), EMPTY_BODY).await?;
        request.send(&self.client, &[]).await?;
        Ok(())
    }
}