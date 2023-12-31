use crossterm::style::Stylize;
use std::io::stdout;

use ovhdata_common::model::di::workflow::{WorkflowPatch, WorkflowSpec};
use ovhdata_common::model::utils::sort_workflow;
use ovhdata_common::ovhapi::{DiApi, OVHapiV6Client};

use crate::config::Context;
use crate::options::{DiSubWorkflowCommands, WorkflowCreate, WorkflowDelete, WorkflowGet, WorkflowList, WorkflowRun, WorkflowUpdate};
use crate::utils::ui::printer::{Output, Printer};
use crate::utils::{Error, Result};

pub struct WorkflowCommand {
    rcp_client: OVHapiV6Client,
}

impl WorkflowCommand {
    pub fn new(rcp_client: OVHapiV6Client) -> Self {
        Self { rcp_client }
    }

    pub async fn execute_command(&self, commands: DiSubWorkflowCommands) -> Result<()> {
        match commands {
            DiSubWorkflowCommands::List(workflow_list) => self.list(&workflow_list, workflow_list.output.unwrap_or_default().into()).await,
            DiSubWorkflowCommands::Get(workflow_get) => self.get(&workflow_get, workflow_get.output.unwrap_or_default().into()).await,
            DiSubWorkflowCommands::Create(workflow_create) => self.create(&workflow_create, workflow_create.output.unwrap_or_default().into()).await,
            DiSubWorkflowCommands::Run(workflow_run) => self.run(&workflow_run, workflow_run.output.unwrap_or_default().into()).await,
            DiSubWorkflowCommands::Delete(workflow_delete) => self.delete(&workflow_delete).await,
            DiSubWorkflowCommands::Update(workflow_update) => self.update(&workflow_update, workflow_update.output.unwrap_or_default().into()).await,
            DiSubWorkflowCommands::Enable(workflow_get) => self.enable(&workflow_get, workflow_get.output.unwrap_or_default().into()).await,
            DiSubWorkflowCommands::Disable(workflow_get) => self.disable(&workflow_get, workflow_get.output.unwrap_or_default().into()).await,
        }
    }

    async fn list(&self, input: &WorkflowList, output: Output) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();

        let mut workflows = self.rcp_client.clone().di_workflows(&service_name, input.filter.clone()).await?;

        if output == Output::default_table() && !workflows.is_empty() {
            workflows = sort_workflow(workflows, input.sort.clone().unwrap_or_default().as_str(), input.desc);

            if !input.script {
                Printer::print_interactive_list(&workflows, None)?;
                return Ok(());
            }
        }

        Printer::print_list(&workflows, &output)?;
        Ok(())
    }

    async fn get(&self, input: &WorkflowGet, output: Output) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();

        let workflow_id = self.get_workflow_id(&service_name, &input.id).await?;

        if input.id.is_none() {
            Printer::print_command(&format!("di workflow get {} --service-name {} ", &workflow_id, &service_name));
        }

        let workflow = self.rcp_client.clone().di_workflow(&service_name, &workflow_id).await?;
        Printer::print_object(&workflow, &output)?;
        Ok(())
    }

    async fn create(&self, input: &WorkflowCreate, output: Output) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();

        let missing_source = input.source_id.is_none();
        let source_id = if missing_source {
            let sources = self.rcp_client.clone().di_sources(&service_name, None).await?;
            Printer::ask_select_table(&sources, None)?.id.clone()
        } else {
            input.source_id.clone().unwrap()
        };

        let missing_destination = input.destination_id.is_none();
        let destination_id = if missing_destination {
            let destinations = self.rcp_client.clone().di_destinations(&service_name, None).await?;
            Printer::ask_select_table(&destinations, None)?.id.clone()
        } else {
            input.destination_id.clone().unwrap()
        };

        let interactive = missing_destination || missing_source;

        let description = if interactive {
            Printer::ask_input_string("Enter a description (press enter to skip)", input.description.clone(), true, None)
        } else {
            input.description.clone()
        };

        let schedule = if interactive {
            Printer::ask_input_string("Enter a schedule (press enter to skip)", input.schedule.clone(), true, None)
        } else {
            input.schedule.clone()
        };

        let spec = WorkflowSpec {
            name: input.name.clone(),
            description,
            source_id,
            destination_id,
            schedule,
            region: input.region.clone(),
            enabled: !input.disabled,
        };

        // if there was an interaction, ask for confirmation
        if missing_destination || missing_source {
            Printer::print_object(&spec, &output)?;
            let message = format!("Do you want to create the workflow {} ?", input.name.clone());
            let confirm = Printer::confirm(&message);

            let mut cmd: String = format!(
                "di workflow create {} --service-name {} --source-id {} --destination-id {} --region {}",
                &spec.name, &service_name, &spec.source_id, &spec.destination_id, &spec.region
            );
            if spec.description.clone().is_some() {
                cmd.push_str(&format!(" --description {}", spec.description.clone().unwrap()));
            }
            if spec.schedule.clone().is_some() {
                cmd.push_str(&format!(" --schedule {}", spec.schedule.clone().unwrap()));
            }
            Printer::print_command(&cmd);

            if confirm.is_err() {
                return Err(Error::custom("Create workflow canceled"));
            }
        }

        let spinner = Printer::start_spinner("Creating workflow");
        let workflow = self.rcp_client.di_workflow_post(&service_name, &spec).await?;
        Printer::stop_spinner(spinner);

        Printer::print_object(&workflow, &output)?;
        Ok(())
    }

    async fn run(&self, input: &WorkflowRun, output: Output) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();
        let interactive = input.id.is_none();

        let id = if interactive {
            let workflows = self.rcp_client.clone().di_workflows(&service_name, None).await?;
            Printer::ask_select_table(&workflows, None)?.id.clone()
        } else {
            input.id.clone().unwrap()
        };

        if interactive {
            Printer::print_command(&format!("di destination run {} --service-name {} ", &id, &service_name));
        }

        let spinner = Printer::start_spinner("Running workflow");
        let workflow = self.rcp_client.clone().di_job_post(&service_name, &id).await?;
        Printer::stop_spinner(spinner);

        Printer::print_object(&workflow, &output)?;
        Ok(())
    }

    async fn delete(&self, input: &WorkflowDelete) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();

        let workflow_id = self.get_workflow_id(&service_name, &input.id).await?;

        if !input.script {
            let message = format!("Are you sure you want to delete the workflow {} ?", workflow_id.clone().green());
            let confirm = Printer::confirm(&message);

            if confirm.is_err() {
                return Err(Error::custom("Delete workflow canceled"));
            }
        }

        if input.id.is_none() {
            Printer::print_command(&format!("di workflow delete {} --service-name {} ", &workflow_id, &service_name));
        }

        let spinner = Printer::start_spinner("Deleting workflow");
        self.rcp_client.clone().di_workflow_delete(&service_name, &workflow_id).await?;
        Printer::stop_spinner(spinner);

        Printer::println_success(&mut stdout(), &format!("Workflow {} successfully deleted", workflow_id.clone().green()));
        Ok(())
    }

    async fn update(&self, input: &WorkflowUpdate, output: Output) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();

        let workflow_id = self.get_workflow_id(&service_name, &input.id).await?;

        let interactive_update = input.name.is_none() && input.description.is_none() && input.schedule.is_none() && input.enabled.is_none();

        let spec = if interactive_update {
            let workflow = self.rcp_client.clone().di_workflow(&service_name, &workflow_id).await?;

            let name = Printer::ask_input_string("Enter the new name", Some(workflow.name), true, None);
            let description = Printer::ask_input_string("Enter the new definition", workflow.description, true, None);
            let schedule = Printer::ask_input_string("Enter the new schedule", workflow.schedule, true, None);
            let enabled = Printer::ask_input_boolean("Is the workflow enabled", workflow.enabled).unwrap();

            WorkflowPatch {
                name,
                description,
                schedule,
                enabled: Some(enabled),
            }
        } else {
            WorkflowPatch {
                name: input.name.clone(),
                description: input.description.clone(),
                schedule: input.schedule.clone(),
                enabled: input.enabled,
            }
        };

        if interactive_update {
            Printer::print_object(&spec, &output)?;
            let confirm = Printer::confirm(&format!("Do you want to update the workflow {} ?", &workflow_id));

            let mut cmd = format!("di workflow update {} --service-name {} ", &workflow_id, &service_name);
            if spec.name.clone().is_some() {
                cmd.push_str(&format!(" --name {}", spec.name.clone().unwrap()));
            }
            if spec.enabled.is_some() {
                cmd.push_str(&format!(" --enabled {}", spec.enabled.unwrap()));
            }
            if spec.description.clone().is_some() {
                cmd.push_str(&format!(" --description {}", spec.description.clone().unwrap()));
            }
            if spec.schedule.clone().is_some() {
                cmd.push_str(&format!(" --schedule {}", spec.schedule.clone().unwrap()));
            }
            Printer::print_command(&cmd);

            if confirm.is_err() {
                return Err(Error::custom("Update workflow canceled"));
            }
        }

        let spinner = Printer::start_spinner("Updating workflow");
        let workflow = self.rcp_client.di_workflow_put(&service_name, &workflow_id, &spec).await?;
        Printer::stop_spinner(spinner);

        Printer::print_object(&workflow, &output)?;

        Ok(())
    }

    async fn enable(&self, input: &WorkflowGet, output: Output) -> Result<()> {
        self.toggle_enabled(input, output, true).await
    }

    async fn disable(&self, input: &WorkflowGet, output: Output) -> Result<()> {
        self.toggle_enabled(input, output, false).await
    }

    async fn toggle_enabled(&self, input: &WorkflowGet, _output: Output, enabled: bool) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();

        let workflow_id = self.get_workflow_id(&service_name, &input.id).await?;

        let spec = WorkflowPatch {
            name: None,
            description: None,
            schedule: None,
            enabled: Some(enabled),
        };

        let verb = if enabled { "enabl" } else { "disabl" };

        if input.id.is_none() {
            Printer::print_command(&format!("di workflow {}e {} --service-name {} ", &verb, &workflow_id, &service_name));
        }

        let spinner = Printer::start_spinner(&format!("Workflow {}ing", &verb));
        self.rcp_client.di_workflow_put(&service_name, &workflow_id, &spec).await?;
        Printer::stop_spinner(spinner);

        Printer::println_success(&mut stdout(), &format!("\nWorkflow {} {}ed", workflow_id.clone().green(), &verb));
        Ok(())
    }

    async fn get_workflow_id(&self, service_name: &str, input_id: &Option<String>) -> Result<String> {
        let interactive = input_id.is_none();

        let id = if interactive {
            let workflows = self.rcp_client.clone().di_workflows(service_name, None).await?;
            Printer::ask_select_table(&workflows, None)?.id.clone()
        } else {
            input_id.clone().unwrap()
        };

        Ok(id)
    }
}
