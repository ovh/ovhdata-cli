use ovhdata_common::config::ConfigName;
use ovhdata_common::ovhapi::{OVHapiV6Client, ProjectApi};

use crate::config::{custom_config_path, Config, SelectableItem, Context};
use crate::opts::ConfigSubCommand;
use crate::utils::ui::printer::{Output, Printer};
use crate::utils::{Error, Result};



pub struct ConfigCommand {
    rcp_client: OVHapiV6Client,
}

impl ConfigCommand {
    pub fn new(rcp: OVHapiV6Client) -> Self {
        ConfigCommand { rcp_client: rcp }
    }


    pub async fn execute_command(&self, config_command: ConfigSubCommand) -> Result<()> {
        match config_command {
            ConfigSubCommand::List(config_list) => self.list(config_list.output.unwrap_or_default().into()),
            ConfigSubCommand::Get(config_get) => self.get(
                config_get.config_name,
                config_get.output.unwrap_or_default().into(),
            ),
            ConfigSubCommand::Set(config_set) => self.set(
                config_set.config_name,
                config_set.output.unwrap_or_default().into(),
            ),
            ConfigSubCommand::SetServiceName(config_set_service_name) => self.set_service_name(
                &config_set_service_name.service_name,
            ).await,
        }
    }

    /// List all available configurations with their names
    fn list(&self, output: Output) -> Result<()> {
        let all_config = Config::get_all();
        let all_items = all_config
            .configs
            .iter()
            .map(|(config_name, config)| {
                let is_selected = &all_config.current_config_name == config_name;
                SelectableItem::new(
                    Config::new(
                        config_name.clone(),
                        config.clone(),
                        None,
                    ),
                    is_selected,
                )
            })
            .collect::<Vec<SelectableItem>>();
        Printer::print_list(&all_items, &output)?;
        Ok(())
    }

    /// Get a single configuration info by its name
    fn get(&self, name: Option<String>, output: Output) -> Result<()> {
        let config = self.get_config(name)?;

        let runtime_context = Context::get().get_runtime_context(&config.0);

        Printer::print_object(
            &Config::new(config.0, config.1, Some(runtime_context)),
            &output,
        )?;
        Ok(())
    }

    /// Switch to the configuration identify by the given name
    fn set(&self, name: Option<String>, output: Output) -> Result<()> {
        let config = self.get_config(name)?;
        let runtime_context = Context::get().get_runtime_context(&config.0);

        let mut all_config = Config::get_all();
        all_config.set_current_config(config.clone().0)?;
        all_config.save(custom_config_path())?;

        Printer::print_object(
            &Config::new(config.0, config.1, Some(runtime_context)),
            &output,
        )?;
        Ok(())
    }

    // Get config with interactive mode if necessary
    fn get_config(&self, name: Option<String>) -> Result<(ConfigName, ovhdata_common::config::Config)> {
        let all_config = Config::get_all();
        let all_items = all_config
            .configs
            .iter()
            .map(|(config_name, config)| {
                Config::new(
                    config_name.clone(),
                    config.clone(),
                    None,
                )
            })
            .collect::<Vec<Config>>();

        let config_name = if name.is_some() {
            ConfigName::from(name.unwrap())
        } else {
            let selected_config = Printer::ask_select_table(&all_items, None)?;
            selected_config.name.clone()
        };

        let config = all_config.get_config(config_name.clone()).ok_or(Error::Custom(format!("Unknown configuration")))?.clone();
        Ok((config_name.clone(), config.clone()))
    }

    /// Set the default service name to use for the current config
    async fn set_service_name(&self, input_service_name: &Option<String>) -> Result<()> {
        let mut context = Context::get();
        let interactive = input_service_name.is_none();

        let service_name = if interactive {
            let projects = self.rcp_client.projects().await?;
            Printer::ask_select_table(&projects, None)?.project_id.clone()
        } else {
            input_service_name.clone().unwrap()
        };

        context.set_service_name(service_name);
        context.save()?;

        Ok(())
    }
}