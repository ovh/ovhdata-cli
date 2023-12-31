use crossterm::style::Stylize;
use std::io::stdout;

use ovhdata_common::model::di::common::ParametersWrapper;
use ovhdata_common::model::di::source::SourceSpec;
use ovhdata_common::model::utils::sort_source;
use ovhdata_common::ovhapi::{DiApi, OVHapiV6Client};

use crate::command::di::source_metadata::SourceMetadataCommand;
use crate::config::Context;
use crate::options::{DiSubSourceCommands, SourceCreate, SourceDelete, SourceGet, SourceList, SourceUpdate};
use crate::utils::ui::printer::{Output, Printer};
use crate::utils::{Error, Result};

pub struct SourceCommand {
    rcp_client: OVHapiV6Client,
}

impl SourceCommand {
    pub fn new(rcp_client: OVHapiV6Client) -> Self {
        Self { rcp_client }
    }

    pub async fn execute_command(&self, commands: DiSubSourceCommands) -> Result<()> {
        match commands {
            DiSubSourceCommands::Status(src_get) => self.get_last_connection_status(&src_get, src_get.output.unwrap_or_default().into()).await,
            DiSubSourceCommands::List(source_list) => self.list(&source_list, source_list.output.unwrap_or_default().into()).await,
            DiSubSourceCommands::Get(source_get) => self.get(&source_get, source_get.output.unwrap_or_default().into()).await,
            DiSubSourceCommands::Metadata(subcmd) => SourceMetadataCommand::new(self.rcp_client.clone()).execute_command(subcmd).await,
            DiSubSourceCommands::Create(source_create) => self.create(&source_create, source_create.output.unwrap_or_default().into()).await,
            DiSubSourceCommands::Update(source_update) => self.update(&source_update, source_update.output.unwrap_or_default().into()).await,
            DiSubSourceCommands::Delete(source_delete) => self.delete(&source_delete).await,
            DiSubSourceCommands::TestConnection(source_test) => self.test_connection(&source_test, source_test.output.unwrap_or_default().into()).await,
        }
    }

    async fn list(&self, input: &SourceList, output: Output) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();

        let mut sources = self.rcp_client.clone().di_sources(&service_name, input.filter.clone()).await?;

        if output == Output::default_table() && !sources.is_empty() {
            sources = sort_source(sources, input.sort.clone().unwrap_or_default().as_str(), input.desc);

            if !input.script {
                Printer::print_interactive_list(&sources, None)?;
                return Ok(());
            }
        }

        Printer::print_list(&sources, &output)?;
        Ok(())
    }

    async fn get(&self, input: &SourceGet, output: Output) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();

        let id = self.get_source_id(&service_name, &input.id).await?;

        if input.id.is_none() {
            Printer::print_command(&format!("di source get {} --service-name {} ", &id, &service_name));
        }

        let source = self.rcp_client.clone().di_source(&service_name, &id).await?;
        Printer::print_object(&source, &output)?;
        Ok(())
    }

    async fn test_connection(&self, input: &SourceGet, output: Output) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();

        let id = self.get_source_id(&service_name, &input.id).await?;

        if input.id.is_none() {
            Printer::print_command(&format!("di source test-connection {} --service-name {} ", &id, &service_name));
        }

        let spinner = Printer::start_spinner("Testing source connection");
        let source = self.rcp_client.clone().di_source_test(&service_name, &id).await?;
        Printer::stop_spinner(spinner);

        Printer::print_object(&source, &output)?;
        Ok(())
    }

    async fn get_last_connection_status(&self, input: &SourceGet, output: Output) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();
        let id = self.get_source_id(&service_name, &input.id).await?;

        if input.id.is_none() {
            Printer::print_command(&format!("di source status {} --service-name {} ", &id, &service_name));
        }

        let source_status = self.rcp_client.clone().di_source_status(&service_name, &id).await?;
        Printer::print_object(&source_status, &output)?;
        Ok(())
    }

    async fn update(&self, input: &SourceUpdate, output: Output) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();
        let id = self.get_source_id(&service_name, &input.id).await?;

        // Get the existing source
        let source = self.rcp_client.di_source(&service_name, &id).await?;

        // Get connector specs
        let connector = self.rcp_client.clone().di_source_connector(&service_name, &source.connector_id).await?;

        let interactive = input.name.is_none();
        let name = if interactive {
            Printer::ask_input("Enter the new source name", Some(&source.name)).unwrap()
        } else {
            input.name.clone().unwrap()
        };

        // Update connector parameters with the

        let parameters = Printer::ask_connector_parameters(&input.parameters, Some(&source.parameters), &connector.parameters).unwrap();
        let parameters_len = parameters.len();

        // Default values will be overridden
        let spec = SourceSpec {
            name,
            parameters,
            connector_id: None,
        };

        // new parameters we are in interactive mode
        if interactive || parameters_len > input.parameters.len() {
            Printer::print_object(&spec, &output)?;
            let confirm = Printer::confirm(&format!("Do you want to update the source {} ?", id));

            let cmd = format!(
                "di source update {} {} --service-name {} {}",
                &id,
                &spec.name,
                &service_name,
                ParametersWrapper(spec.parameters.clone())
            );
            Printer::print_command(&cmd);

            if confirm.is_err() {
                return Err(Error::custom("Update source canceled"));
            }
        }

        let spinner = Printer::start_spinner("Source updating");
        let source = self.rcp_client.di_source_update(&service_name, &id, &spec).await?;
        Printer::stop_spinner(spinner);

        Printer::print_object(&source, &output)?;
        Ok(())
    }

    async fn create(&self, input: &SourceCreate, output: Output) -> Result<()> {
        let interactive = input.connector_id.is_none();
        let service_name = Context::get().get_current_service_name().unwrap();

        let connector_id = if interactive {
            let connectors = self.rcp_client.clone().di_source_connectors(&service_name).await?;

            let connector = Printer::ask_select_table(&connectors, None)?;
            println!(
                "Source connector {} selected. id={}",
                connector.name.clone().green(),
                connector.id.clone().green()
            );
            connector.id.clone()
        } else {
            input.connector_id.clone().unwrap()
        };

        let connector = self.rcp_client.clone().di_source_connector(&service_name, &connector_id).await?;

        let parameters = Printer::ask_connector_parameters(&input.parameters, None, &connector.parameters).unwrap();
        let parameters_len = parameters.len();

        // Default values will be overridden
        let spec = SourceSpec {
            name: input.name.clone(),
            parameters,
            connector_id: Some(connector_id.clone()),
        };

        // new parameters we are in interactive mode
        if interactive || parameters_len > input.parameters.len() {
            Printer::print_object(&spec, &output)?;
            let confirm = Printer::confirm(&format!("Do you want to create the source {} ?", &input.name));

            Printer::print_command(&format!(
                "di source create {} --service-name {} --connector-id {} {}",
                &spec.name,
                &service_name,
                &connector_id,
                ParametersWrapper(spec.parameters.clone())
            ));

            if confirm.is_err() {
                return Err(Error::custom("Create source canceled"));
            }
        }

        let spinner = Printer::start_spinner("Source creating");
        let source = self.rcp_client.di_source_post(&service_name, &spec).await?;
        Printer::stop_spinner(spinner);

        Printer::print_object(&source, &output)?;
        Ok(())
    }

    async fn delete(&self, input: &SourceDelete) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();

        let source_id = self.get_source_id(&service_name, &input.id).await?;

        if !input.script {
            let message = format!("Are you sure you want to delete the source {} ?", source_id.clone().green());
            let confirm = Printer::confirm(&message);

            if confirm.is_err() {
                return Err(Error::custom("Delete source canceled"));
            }
        }

        if input.id.is_none() {
            Printer::print_command(&format!("di source delete {} --service-name {} ", &source_id, &service_name));
        }

        let spinner = Printer::start_spinner("Deleting source");
        self.rcp_client.di_source_delete(&service_name, &source_id).await?;
        Printer::stop_spinner(spinner);

        Printer::println_success(&mut stdout(), &format!("Source {} successfully deleted", source_id.clone().green()));
        Ok(())
    }

    async fn get_source_id(&self, service_name: &str, input_id: &Option<String>) -> Result<String> {
        let interactive = input_id.is_none();

        let id = if interactive {
            let sources = self.rcp_client.clone().di_sources(service_name, None).await?;

            Printer::ask_select_table(&sources, None)?.id.clone()
        } else {
            input_id.clone().unwrap()
        };

        Ok(id)
    }
}
