use crossterm::style::Stylize;
use std::io::stdout;

use ovhdata_common::model::di::source_metadata::{TableMeta};
use ovhdata_common::ovhapi::{OVHapiV6Client, DiApi};

use crate::config::Context;
use crate::opts::{SourceGet, SourceSubMetaCommands};
use crate::utils::ui::printer::{Output, Printer};
use crate::utils::{Error, Result};

pub struct SourceMetadataCommand {
    rcp_client: OVHapiV6Client,
}

impl SourceMetadataCommand {
    pub fn new(rcp_client: OVHapiV6Client) -> Self {
        Self { rcp_client }
    }

    pub async fn execute_command(&self, commands: SourceSubMetaCommands) -> Result<()> {
        match commands {
            SourceSubMetaCommands::Get(source_get) => { self.get(&source_get, source_get.output.unwrap_or_default().into()).await }
            SourceSubMetaCommands::Extract(source_get) => { self.extract(&source_get, source_get.output.unwrap_or_default().into()).await }
        }
    }

    async fn get(&self, input: &SourceGet, output: Output) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();

        let id = self.get_source_id(&service_name, &input.id).await?;

        let spinner = Printer::start_spinner("Retrieving source metadata");
        let tables = self.rcp_client.clone().di_source_metadata(&service_name, &id).await?;
        Printer::stop_spinner(spinner);

        Printer::print_list::<TableMeta>(tables.as_slice(), &output)?;
        Ok(())
    }

    async fn extract(&self, input: &SourceGet, output: Output) -> Result<()> {
        let service_name = Context::get().get_current_service_name().unwrap();

        let id = self.get_source_id(&service_name, &input.id).await?;

        let spinner = Printer::start_spinner("Extracting metadata from source");
        let tables = self.rcp_client.clone().di_source_metadata_post(&service_name, &id).await?;
        Printer::stop_spinner(spinner);

        Printer::print_list::<TableMeta>(tables.as_slice(), &output)?;
        Ok(())
    }

    
    async fn get_source_id(&self, service_name: &String, input_id: &Option<String>) -> Result<String> {
        let interactive = input_id.is_none();

        let id = if interactive {
            let sources = self.rcp_client.clone().di_sources(&service_name).await?;

            Printer::ask_select_table(&sources, None)?.id.clone()
        } else {
            input_id.clone().unwrap()
        };

        Ok(id)
    }
}