use ovhdata_common::ovhapi::{OVHapiV6Client, DiApi};

use crate::config::Context;
use crate::opts::{DestConnectorGet, DiSubDestConnectorCommands};
use crate::utils::ui::printer::{Output, Printer};
use crate::utils::Result;

pub struct DestinationConnectorCommand {
    rcp_client: OVHapiV6Client,
}

impl DestinationConnectorCommand {
    pub fn new(rcp_client: OVHapiV6Client) -> Self {
        Self { rcp_client }
    }

    pub async fn execute_command(&self, connector_commands: DiSubDestConnectorCommands) -> Result<()> {
        match connector_commands {
            DiSubDestConnectorCommands::List(connector_list) => { self.list_destination_connectors(connector_list.output.unwrap_or_default().into()).await }
            DiSubDestConnectorCommands::Get(connector_get) => { self.get_destination_connector(&connector_get, connector_get.output.unwrap_or_default().into()).await }
        }
    }

    async fn list_destination_connectors(&self, output: Output) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();

        let connectors = self.rcp_client.clone().di_destination_connectors(&service_name).await?;
        Printer::print_list(&connectors, &output)?;
        Ok(())
    }

    async fn get_destination_connector(&self, input: &DestConnectorGet, output: Output) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();
        let interactive = input.id.is_none();

        let id = if interactive {
            let connectors = self.rcp_client.clone().di_destination_connectors(&service_name).await?;
            Printer::ask_select_table(&connectors, None)?.id.clone()
        } else {
            input.id.clone().unwrap()
        };

        if interactive {
            Printer::print_command(&format!("di destination-connector get {} --service-name {} ", &id, &service_name));
        }

        let connector = self.rcp_client.clone().di_destination_connector(&service_name, &id).await?;
        Printer::print_object(&connector, &output)?;
        Ok(())
    }
}