use crossterm::style::Stylize;
use ovhdata_common::ovhapi::{DiApi, OVHapiV6Client};
use ovhdata_common::model::utils::sort_job;
use std::io::stdout;

use crate::config::Context;
use crate::options::{DiSubJobCommands, JobGet, JobList, JobStop};
use crate::utils::ui::printer::{Output, Printer};
use crate::utils::Result;

pub struct JobCommand {
    rcp_client: OVHapiV6Client,
}

impl JobCommand {
    pub fn new(rcp_client: OVHapiV6Client) -> Self {
        Self { rcp_client }
    }

    pub async fn execute_command(&self, commands: DiSubJobCommands) -> Result<()> {
        match commands {
            DiSubJobCommands::List(job_list) => self.list(&job_list, job_list.output.unwrap_or_default().into()).await,
            DiSubJobCommands::Get(job_get) => self.get(&job_get, job_get.output.unwrap_or_default().into()).await,
            DiSubJobCommands::Stop(job_stop) => self.stop(&job_stop).await,
        }
    }

    async fn list(&self, input: &JobList, output: Output) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();
        let interactive = input.workflow_id.is_none();

        let workflow_id = if interactive {
            let workflows = self.rcp_client.clone().di_workflows(&service_name).await?;
            Printer::ask_select_table(&workflows, None)?.id.clone()
        } else {
            input.workflow_id.clone().unwrap()
        };

        if interactive {
            Printer::print_command(&format!("di job list --service-name {} --workflow-id {}", &service_name, &workflow_id));
        }

        let mut jobs = self.rcp_client.clone().di_jobs_filtered(&service_name, &workflow_id,input.filter.clone()).await?;
        
        if output == Output::default_table() {
            jobs = sort_job(jobs, input.order.clone().unwrap_or_default().as_str(), input.desc);

            if !input.force {
                Printer::print_interactive_list(&jobs, None)?;
                return Ok(());
            }
        } 

        Printer::print_list(&jobs, &output)?;
        Ok(())
    }

    async fn get(&self, input: &JobGet, output: Output) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();

        let result = self.get_ids(&service_name, &input.workflow_id, &input.id).await?;
        let workflow_id = result.0;
        let id = result.1;

        if input.id.is_none() {
            Printer::print_command(&format!(
                "di job get {} --service-name {} --workflow-id {}",
                &id, &service_name, &workflow_id
            ));
        }

        let job = self.rcp_client.clone().di_job(&service_name, &workflow_id, &id).await?;
        Printer::print_object(&job, &output)?;
        Ok(())
    }

    async fn stop(&self, input: &JobStop) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();

        let result = self.get_ids(&service_name, &input.workflow_id, &input.id).await?;
        let workflow_id = result.0;
        let id = result.1;

        if input.id.is_none() {
            Printer::print_command(&format!(
                "di job stop {} --service-name {} --workflow-id {}",
                &id, &service_name, &workflow_id
            ));
        }

        self.rcp_client.clone().di_job_delete(&service_name, &workflow_id, &id).await?;
        Printer::println_success(&mut stdout(), &format!("Job {} stopped", id.clone().green()));
        Ok(())
    }

    async fn get_ids(&self, service_name: &str, input_workflow_id: &Option<String>, input_id: &Option<String>) -> Result<(String, String)> {
        let missing_workflow = input_workflow_id.is_none();
        let mut missing_job = input_id.is_none();

        let workflow_id = if missing_workflow {
            let workflows = self.rcp_client.clone().di_workflows(service_name).await?;
            missing_job = true;

            Printer::ask_select_table(&workflows, None)?.id.clone()
        } else {
            input_workflow_id.clone().unwrap()
        };

        let id = if missing_job {
            let jobs = self.rcp_client.clone().di_jobs(service_name, &workflow_id).await?;

            Printer::ask_select_table(&jobs, input_id.clone())?.id.clone()
        } else {
            input_id.clone().unwrap()
        };

        Ok((workflow_id, id))
    }
}
