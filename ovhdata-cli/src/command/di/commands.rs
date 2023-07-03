use crate::command::di::destination::DestinationCommand;
use crate::command::di::destination_connector::DestinationConnectorCommand;
use crate::command::di::job::JobCommand;
use crate::command::di::source::SourceCommand;
use crate::command::di::source_connector::SourceConnectorCommand;
use crate::command::di::workflow::WorkflowCommand;
use crate::options::DiSubCommands;
use crate::utils::Result;
use ovhdata_common::ovhapi::OVHapiV6Client;
use tracing::info;

pub struct DiCommand {
    rcp_client: OVHapiV6Client,
}

impl DiCommand {
    pub fn new(rcp: OVHapiV6Client) -> Self {
        DiCommand { rcp_client: rcp }
    }

    pub async fn execute_command(&self, di_commands: DiSubCommands) -> Result<()> {
        info!("data integration command (di)");
        match di_commands {
            DiSubCommands::Source(subcmd) => {
                SourceCommand::new(self.rcp_client.clone())
                    .execute_command(subcmd)
                    .await
            }

            DiSubCommands::Destination(subcmd) => {
                DestinationCommand::new(self.rcp_client.clone())
                    .execute_command(subcmd)
                    .await
            }

            DiSubCommands::SourceConnector(subcmd) => {
                SourceConnectorCommand::new(self.rcp_client.clone())
                    .execute_command(subcmd)
                    .await
            }

            DiSubCommands::DestinationConnector(subcmd) => {
                DestinationConnectorCommand::new(self.rcp_client.clone())
                    .execute_command(subcmd)
                    .await
            }

            DiSubCommands::Workflow(subcmd) => {
                WorkflowCommand::new(self.rcp_client.clone())
                    .execute_command(subcmd)
                    .await
            }

            DiSubCommands::Job(subcmd) => {
                JobCommand::new(self.rcp_client.clone())
                    .execute_command(subcmd)
                    .await
            }
        }
    }
}
