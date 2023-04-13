use crate::opts::{OutputList, OutputObject};
use clap::builder::{PossibleValuesParser, TypedValueParser};
use clap::Parser;

#[derive(Parser)]
pub struct ConfigShim {
    #[clap(subcommand)]
    pub subcmd: ConfigSubCommand,
}

#[derive(Parser)]
pub enum ConfigSubCommand {
    /// List of available regions
    #[clap(visible_alias = "ls")]
    List(ConfigList),
    /// Display the area config
    Get(ConfigGet),
    /// Set the current area in the config file
    Set(ConfigSet),
    /// Set cloud service name as default
    SetServiceName(ConfigServiceName),
}

#[derive(Parser)]
pub struct ConfigList {
    /// Command output format
    #[clap(short, long, value_parser = PossibleValuesParser::new(&["json", "yaml", "table"]).map(|s| s.parse::<OutputList>().unwrap()))]
    pub output: Option<OutputList>,
}

#[derive(Parser)]
pub struct ConfigSet {
    pub config_name: Option<String>,
    /// Command output format
    #[clap(short, long, value_parser = PossibleValuesParser::new(&["json", "yaml", "description"]).map(|s| s.parse::<OutputObject>().unwrap()))]
    pub output: Option<OutputObject>,
}

#[derive(Parser)]
pub struct ConfigGet {
    pub config_name: Option<String>,
    /// Command output format
    #[clap(short, long, value_parser = PossibleValuesParser::new(&["json", "yaml", "description"]).map(|s| s.parse::<OutputObject>().unwrap()))]
    pub output: Option<OutputObject>,
}

#[derive(Parser)]
pub struct ConfigServiceName {
    /// Service name to set (interactive input if not set)
    pub service_name: Option<String>,
}