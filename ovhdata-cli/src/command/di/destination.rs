use crossterm::style::Stylize;
use std::io::stdout;

use ovhdata_common::model::di::destination::DestinationSpec;
use ovhdata_common::model::di::common::parameters_as_string;
use ovhdata_common::ovhapi::{OVHapiV6Client, DiApi};

use crate::config::Context;
use crate::opts::{DestGet, DiSubDestCommands, DestCreate, DestUpdate, DestDelete};
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
            DiSubDestCommands::Status(destination_get) => { self.get_last_connection_status(&destination_get, destination_get.output.unwrap_or_default().into()).await }
            DiSubDestCommands::List(destination_list) => { self.list(destination_list.output.unwrap_or_default().into()).await }
            DiSubDestCommands::Get(destination_get) => { self.get(&destination_get, destination_get.output.unwrap_or_default().into()).await }
            DiSubDestCommands::Create(destination_create) => { self.create(&destination_create, destination_create.output.unwrap_or_default().into()).await }
            DiSubDestCommands::Update(destination_update) => { self.update(&destination_update, destination_update.output.unwrap_or_default().into()).await }
            DiSubDestCommands::Delete(destination_delete) => { self.delete(&destination_delete).await }
        }
    }

    async fn list(&self, output: Output) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();

        let destinations = self.rcp_client.clone().di_destinations(&service_name).await?;
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

    async fn get_last_connection_status(&self, input: &DestGet, output: Output) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();
        let id = self.get_destination_id(&service_name, &input.id).await?;

        if input.id.is_none() {
            Printer::print_command(&format!("di destination status {} --service-name {} ", &id, &service_name));
        }

        let destination_status =self.rcp_client.clone().di_destination_status(&service_name, &id).await?;
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
            let message  = format!("Do you want to create the destination {} ?", input.name.clone());
            let confirm = Printer::confirm(message.as_str());

            let cmd:String = format!("di destnation create {} --service-name {} --connector-id {} {}", &spec.name, &service_name, &connector_id, parameters_as_string(&spec.parameters));
            Printer::print_command(cmd.as_str());

            if confirm.is_err() {
                return Err(Error::Custom(format!("Create destination canceled")));
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
        let connector = self.rcp_client.clone().di_destination_connector(&service_name, &destination.connector_id).await?;

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
            let message  = format!("Do you want to update the destination {} ?", id);
            let confirm = Printer::confirm(message.as_str());

            let cmd:String = format!("di destnation update {} --service-name {} {}", &spec.name, &service_name, parameters_as_string(&spec.parameters));

            Printer::print_command(cmd.as_str());

            if confirm.is_err() {
                return Err(Error::Custom(format!("Update destination canceled")));
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
            let message  = format!("Are you sure you want to delete the destination {} ?", destination_id.clone().green());
            let confirm = Printer::confirm(message.as_str());

            if confirm.is_err() {
                return Err(Error::Custom(format!("Delete destination canceled")));
            }
        }

        if input.id.is_none() {
            Printer::print_command(&format!("di destination delete {} --service-name {} ", &destination_id, service_name));
        }

        self.rcp_client.di_destination_delete(&service_name, &destination_id).await?;
        Printer::println_success(&mut stdout(), &format!("Destination {} successfully deleted", destination_id.clone().green()));
        Ok(())
    }

    async fn get_connector_id(&self, service_name: &String, input_id: &Option<String>) -> Result<String> {
        let interactive = input_id.is_none();

        let connector_id = if interactive {
            let connectors = self.rcp_client.clone().di_destination_connectors(&service_name).await?;

            let connector = Printer::ask_select_table(&connectors, None)?;
            println!("Destination connector {} selected. id={}", connector.name.clone().green(), connector.id.clone().green());
            connector.id.clone()
        } else {
            input_id.clone().unwrap()
        };

        Ok(connector_id.clone())
    }

    async fn get_destination_id(&self, service_name: &String, input_id: &Option<String>) -> Result<String> {
        let interactive = input_id.is_none();

        let id = if interactive {
            let destinations = self.rcp_client.clone().di_destinations(&service_name).await?;
            Printer::ask_select_table(&destinations, None)?.id.clone()
        } else {
            input_id.clone().unwrap()
        };
        Ok(id.clone())
    }
}