use ovhdata_common::ovhapi::{DiApi, OVHapiV6Client};

use crate::config::Context;
use crate::options::{DiSubSourceConnectorCommands, SourceConnectorGet};
use crate::utils::ui::printer::{Output, Printer};
use crate::utils::Result;

pub struct SourceConnectorCommand {
    rcp_client: OVHapiV6Client,
}

impl SourceConnectorCommand {
    pub fn new(rcp_client: OVHapiV6Client) -> Self {
        Self { rcp_client }
    }

    pub async fn execute_command(
        &self,
        connector_commands: DiSubSourceConnectorCommands,
    ) -> Result<()> {
        match connector_commands {
            DiSubSourceConnectorCommands::List(connector_list) => {
                self.list_src_connectors(connector_list.output.unwrap_or_default().into())
                    .await
            }
            DiSubSourceConnectorCommands::Get(connector_get) => {
                self.get_src_connector(
                    &connector_get,
                    connector_get.output.unwrap_or_default().into(),
                )
                .await
            }
        }
    }

    async fn list_src_connectors(&self, output: Output) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();

        let connectors = self
            .rcp_client
            .clone()
            .di_source_connectors(&service_name)
            .await?;
        Printer::print_list(&connectors, &output)?;
        Ok(())
    }

    async fn get_src_connector(&self, input: &SourceConnectorGet, output: Output) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();
        let interactive = input.id.is_none();

        let id = if interactive {
            let connectors = self
                .rcp_client
                .clone()
                .di_source_connectors(&service_name)
                .await?;
            Printer::ask_select_table(&connectors, None)?.id.clone()
        } else {
            input.id.clone().unwrap()
        };

        if interactive {
            Printer::print_command(&format!(
                "di source-connector get {} --service-name {} ",
                &id, &service_name
            ));
        }

        let connector = self
            .rcp_client
            .clone()
            .di_source_connector(&service_name, &id)
            .await?;
        Printer::print_object(&connector, &output)?;
        Ok(())
    }
}
