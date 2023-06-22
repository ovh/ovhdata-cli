use clap::builder::{PossibleValuesParser, TypedValueParser};
use clap::{crate_version, ArgAction, Parser};
use clap_complete::Shell;
use lazy_static::lazy_static;
use std::str::FromStr;

use crate::opts::{ConfigShim, DiShim, ParseError, ParseResult};
use crate::utils::ui::printer::{Output, Printer, HELP_LOGIN_HOW_TO, HELP_MAIN, HELP_COMPLETION_HOW_TO};

lazy_static! {
    static ref BEFORE_HELP_MAIN: String = Printer::gen_help(HELP_MAIN);
    static ref BEFORE_HELP_LOGIN: String = Printer::gen_help(HELP_LOGIN_HOW_TO);
    static ref BEFORE_HELP_COMPLETION: String = Printer::gen_help(HELP_COMPLETION_HOW_TO);
}

/// Client managing OVHcloud data products
#[derive(Parser)]
#[clap(version = crate_version!(), before_help = BEFORE_HELP_MAIN.as_str())]
pub struct Opts {
    /// OVHcloud service name to use
    #[clap(global = true, long)]
    pub service_name: Option<String>,
    #[clap(subcommand)]
    pub subcmd: SubCommand,
    /// Level of verbosity, can be used multiple times
    #[clap(global = true, short, long, action = ArgAction::Count)]
    pub verbose: u8,
    /// Log in json format rather than in plain text
    #[clap(long)]
    pub json_log: bool,
    /// Remove colors from output
    #[clap(global = true, long)]
    pub no_color: bool,
    /// Remove spinner from output
    #[clap(global = true, long)]
    pub no_spinner: bool,
}

#[derive(Parser)]
pub enum SubCommand {
    #[clap(before_help = BEFORE_HELP_COMPLETION.as_str())]
    /// Produces shell completion code for the specified shell
    Completion(Completion),
    /// Controls configuration of ovhdata-cli
    Config(ConfigShim),
    /// Upgrade the CLI
    Upgrade(Upgrade),    
    /// Displays logs of a command executed by the cli
    Debug(Debug),
    /// Di (Data integration) product Subcommand
    Di(DiShim),
    /// Login into OVHcloud API on the current region
    #[clap(before_help = BEFORE_HELP_LOGIN.as_str())]
    Login(Login),
    /// Removes OVHcloud API tokens on the current region
    Logout(Logout),
    /// Me from OVHcloud API
    Me(Me),
}

#[derive(Parser)]
pub struct Logout {

}

#[derive(Parser)]
pub struct Login {
    /// OVH API Application key
    #[clap(short, long)]
    pub application_key: Option<String>,
    /// OVH API Consumer key
    #[clap(short, long)]
    pub consumer_key: Option<String>,
    /// OVH API Secret
    #[clap(short, long)]
    pub secret: Option<String>,
    /// Command output format
    #[clap(short, long, value_parser = PossibleValuesParser::new(&["json", "yaml", "description"]).map(|s| s.parse::<OutputObject>().unwrap()))]
    pub output: Option<OutputObject>,
}

#[derive(Parser)]
pub struct Upgrade {
    /// Force upgrade
    #[clap(short, long)]
    pub force: bool,
}

#[derive(Parser)]
pub struct Debug {
    /// Command session id
    pub session_id: String,
}

#[derive(Parser)]
pub struct Me {
    /// Command output format
    #[clap(short, long, value_parser = PossibleValuesParser::new(&["json", "yaml", "description"]).map(|s| s.parse::<OutputObject>().unwrap()))]
    pub output: Option<OutputObject>,
}

#[derive(Parser)]
pub struct Completion {
    /// Shell name (bash, fish, zsh, powershell)
    pub shell: Shell,
}

#[derive(Parser)]
pub struct SingleOutputObject {
    /// Command output format
    #[clap(short, long, value_parser = PossibleValuesParser::new(&["json", "yaml", "description"]).map(|s| s.parse::<OutputObject>().unwrap()))]
    pub output: Option<OutputObject>,
}

#[derive(Parser)]
pub struct SingleOutputList {
    /// Command output format
    #[clap(short, long, value_parser = PossibleValuesParser::new(&["json", "yaml", "table"]).map(|s| s.parse::<OutputList>().unwrap()))]
    pub output: Option<OutputList>,
}

#[derive(Parser, Clone, Copy)]
pub enum OutputObject {
    Json,
    Yaml,
    Description,
}

impl Default for OutputObject {
    fn default() -> Self {
        Self::Description
    }
}

impl From<OutputObject> for Output {
    fn from(output: OutputObject) -> Self {
        match output {
            OutputObject::Json => Output::Json,
            OutputObject::Yaml => Output::Yaml,
            OutputObject::Description => Output::Description,
        }
    }
}

impl FromStr for OutputObject {
    type Err = ParseError;

    fn from_str(s: &str) -> ParseResult<Self> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputObject::Json),
            "yaml" => Ok(OutputObject::Yaml),
            "description" => Ok(OutputObject::Description),
            _ => Err(ParseError::OutputParse),
        }
    }
}

#[derive(Parser, Clone)]
pub enum OutputList {
    Table,
    Json,
    Yaml,
}

impl Default for OutputList {
    fn default() -> Self {
        Self::Table
    }
}

impl From<OutputList> for Output {
    fn from(output: OutputList) -> Self {
        match output {
            OutputList::Table => Output::default_table(),
            OutputList::Json => Output::Json,
            OutputList::Yaml => Output::Yaml,
        }
    }
}

impl FromStr for OutputList {
    type Err = ParseError;

    fn from_str(s: &str) -> ParseResult<Self> {
        match s.to_lowercase().as_str() {
            "table" => Ok(OutputList::Table),
            "json" => Ok(OutputList::Json),
            "yaml" => Ok(OutputList::Yaml),
            _ => Err(ParseError::OutputParse),
        }
    }
}
