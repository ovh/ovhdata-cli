use crossterm::style::Stylize;
use std::io::stdout;

use ovhdata_common::model::di::common::ParametersWrapper;
use ovhdata_common::model::di::destination::DestinationSpec;
use ovhdata_common::model::utils::sort_dest;
use ovhdata_common::ovhapi::{DiApi, OVHapiV6Client};

use crate::config::Context;
use crate::options::{DestCreate, DestDelete, DestList, DestGet, DestUpdate, DiSubDestCommands};
use crate::utils::ui::printer::{Output, Printer};
use crate::utils::{Error, Result};

pub struct DestinationCommand {
    rcp_client: OVHapiV6Client,
}

impl DestinationCommand {
    pub fn new(rcp_client: OVHapiV6Client) -> Self {
        Self { rcp_client }
    }

    pub async fn execute_command(&self, commands: DiSubDestCommands) -> Result<()> {
        match commands {
            DiSubDestCommands::Status(dest_get) => {
                self.get_last_connection_status(&dest_get, dest_get.output.unwrap_or_default().into())
                    .await
            }
            DiSubDestCommands::List(destination_list) => self.list(&destination_list, destination_list.output.unwrap_or_default().into()).await,
            DiSubDestCommands::Get(destination_get) => self.get(&destination_get, destination_get.output.unwrap_or_default().into()).await,
            DiSubDestCommands::Create(dest_create) => self.create(&dest_create, dest_create.output.unwrap_or_default().into()).await,
            DiSubDestCommands::Update(dest_update) => self.update(&dest_update, dest_update.output.unwrap_or_default().into()).await,
            DiSubDestCommands::Delete(destination_delete) => self.delete(&destination_delete).await,
            DiSubDestCommands::TestConnection(dest_test) => self.test_connection(&dest_test, dest_test.output.unwrap_or_default().into()).await,
        }
    }

    async fn list(&self, input: &DestList, output: Output) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();

        let mut destinations = self.rcp_client.clone().di_destinations_filtered(&service_name, input.filter.clone()).await?;

        if output == Output::default_table() {
            destinations = sort_dest(destinations, input.order.clone().unwrap_or_default().as_str(), input.desc);

            if !input.force {
                Printer::print_interactive_list(&destinations, None)?;
                return Ok(());
            }
        } 
        
        Printer::print_list(&destinations, &output)?;
        Ok(())
    }

    async fn get(&self, input: &DestGet, output: Output) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();

        let id = self.get_destination_id(&service_name, &input.id).await?;

        if input.id.is_none() {
            Printer::print_command(&format!("di destination get {} --service-name {} ", &id, &service_name));
        }

        let destination = self.rcp_client.clone().di_destination(&service_name, &id).await?;
        Printer::print_object(&destination, &output)?;
        Ok(())
    }

    async fn test_connection(&self, input: &DestGet, output: Output) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();
        let id = self.get_destination_id(&service_name, &input.id).await?;

        if input.id.is_none() {
            Printer::print_command(&format!("di destination test-connection {} --service-name {} ", &id, &service_name));
        }

        let spinner = Printer::start_spinner("Testing destination connection");
        let source = self.rcp_client.clone().di_destination_test(&service_name, &id).await?;
        Printer::stop_spinner(spinner);

        Printer::print_object(&source, &output)?;
        Ok(())
    }

    async fn get_last_connection_status(&self, input: &DestGet, output: Output) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();
        let id = self.get_destination_id(&service_name, &input.id).await?;

        if input.id.is_none() {
            Printer::print_command(&format!("di destination status {} --service-name {} ", &id, &service_name));
        }

        let destination_status = self.rcp_client.clone().di_destination_status(&service_name, &id).await?;
        Printer::print_object(&destination_status, &output)?;
        Ok(())
    }

    async fn create(&self, input: &DestCreate, output: Output) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();

        let connector_id = self.get_connector_id(&service_name, &input.connector_id).await?;

        let connector = self.rcp_client.clone().di_destination_connector(&service_name, &connector_id).await?;

        let parameters = Printer::ask_connector_parameters(&input.parameters, None, &connector.parameters).unwrap();
        let parameters_len = parameters.len();

        // Default values will be overridden
        let spec = DestinationSpec {
            name: input.name.clone(),
            parameters,
            connector_id: Some(connector_id.clone()),
        };

        // new parameters we are in interactive mode
        if input.connector_id.is_none() || parameters_len > input.parameters.len() {
            Printer::print_object(&spec, &output)?;
            let message = format!("Do you want to create the destination {} ?", input.name.clone());
            let confirm = Printer::confirm(&message);

            let cmd = format!(
                "di destination create {} --service-name {} --connector-id {} {}",
                &spec.name,
                &service_name,
                &connector_id,
                ParametersWrapper(spec.parameters.clone())
            );
            Printer::print_command(&cmd);

            if confirm.is_err() {
                return Err(Error::custom("Create destination canceled"));
            }
        }

        let destination = self.rcp_client.di_destination_post(&service_name, &spec).await?;
        Printer::print_object(&destination, &output)?;
        Ok(())
    }

    async fn update(&self, input: &DestUpdate, output: Output) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();
        let id = self.get_destination_id(&service_name, &input.id).await?;

        // Get the existing destination
        let destination = self.rcp_client.di_destination(&service_name, &id).await?;

        // Get connector specs
        let connector = self
            .rcp_client
            .clone()
            .di_destination_connector(&service_name, &destination.connector_id)
            .await?;

        let interactive = input.name.is_none();
        let name = if interactive {
            Printer::ask_input("Enter the new destination name", Some(&destination.name)).unwrap()
        } else {
            input.name.clone().unwrap()
        };

        // Update connector parameters with the

        let parameters = Printer::ask_connector_parameters(&input.parameters, Some(&destination.parameters), &connector.parameters).unwrap();
        let parameters_len = parameters.len();

        // Default values will be overridden
        let spec = DestinationSpec {
            name,
            parameters,
            connector_id: None,
        };

        // new parameters we are in interactive mode
        if interactive || parameters_len > input.parameters.len() {
            Printer::print_object(&spec, &output)?;
            let confirm = Printer::confirm(&format!("Do you want to update the destination {} ?", id));

            Printer::print_command(&format!(
                "di destination update {} {} --service-name {} {}",
                &id,
                &spec.name,
                &service_name,
                ParametersWrapper(spec.parameters.clone())
            ));

            if confirm.is_err() {
                return Err(Error::custom("Update destination canceled"));
            }
        }

        let spinner = Printer::start_spinner("Destination updating");
        let destination = self.rcp_client.di_destination_update(&service_name, &id, &spec).await?;
        Printer::stop_spinner(spinner);

        Printer::print_object(&destination, &output)?;
        Ok(())
    }

    async fn delete(&self, input: &DestDelete) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();

        let destination_id = self.get_destination_id(&service_name, &input.id).await?;

        if !input.force {
            let message = format!("Are you sure you want to delete the destination {} ?", destination_id.clone().green());
            let confirm = Printer::confirm(&message);

            if confirm.is_err() {
                return Err(Error::custom("Delete destination canceled"));
            }
        }

        if input.id.is_none() {
            Printer::print_command(&format!("di destination delete {} --service-name {} ", &destination_id, service_name));
        }

        self.rcp_client.di_destination_delete(&service_name, &destination_id).await?;
        Printer::println_success(
            &mut stdout(),
            &format!("Destination {} successfully deleted", destination_id.clone().green()),
        );
        Ok(())
    }

    async fn get_connector_id(&self, service_name: &str, input_id: &Option<String>) -> Result<String> {
        let interactive = input_id.is_none();

        let connector_id = if interactive {
            let connectors = self.rcp_client.clone().di_destination_connectors(service_name).await?;

            let connector = Printer::ask_select_table(&connectors, None)?;
            println!(
                "Destination connector {} selected. id={}",
                connector.name.clone().green(),
                connector.id.clone().green()
            );
            connector.id.clone()
        } else {
            input_id.clone().unwrap()
        };

        Ok(connector_id)
    }

    async fn get_destination_id(&self, service_name: &str, input_id: &Option<String>) -> Result<String> {
        let interactive = input_id.is_none();

        let id = if interactive {
            let destinations = self.rcp_client.clone().di_destinations(service_name).await?;
            Printer::ask_select_table(&destinations, None)?.id.clone()
        } else {
            input_id.clone().unwrap()
        };
        Ok(id)
    }
}
