use crate::options::utils::NameValue;
use crate::options::{OutputList, OutputObject};
use clap::builder::{PossibleValuesParser, TypedValueParser};
use clap::Parser;

#[derive(Parser)]
pub struct DiShim {
    #[clap(subcommand)]
    pub subcmd: DiSubCommands,
}

#[derive(Parser)]
pub enum DiSubCommands {
    /// Sources information
    #[clap(subcommand)]
    Source(DiSubSourceCommands),
    /// Destinations information
    #[clap(subcommand)]
    Destination(DiSubDestCommands),
    /// Source connector information
    #[clap(subcommand)]
    SourceConnector(DiSubSourceConnectorCommands),
    /// Destination connector information
    #[clap(subcommand)]
    DestinationConnector(DiSubDestConnectorCommands),
    /// Workflows information
    #[clap(subcommand)]
    Workflow(DiSubWorkflowCommands),
    /// Jobs information for a specific workflow
    #[clap(subcommand)]
    Job(DiSubJobCommands),
}

#[derive(Parser)]
pub enum DiSubSourceCommands {
    /// List sources
    #[clap(visible_alias = "ls")]
    List(SourceList),
    /// Get a source
    Get(SourceGet),
    /// Get the last connection status
    Status(SourceGet),
    /// Get metadata information
    #[clap(subcommand)]
    Metadata(SourceSubMetaCommands),
    /// Create a new source
    Create(SourceCreate),
    /// Update a new source
    Update(SourceUpdate),
    /// Delete a source
    #[clap(visible_alias = "rm")]
    Delete(SourceDelete),
}

#[derive(Parser)]
pub enum SourceSubMetaCommands {
    /// Get the source's metadata
    Get(SourceGet),
    /// Trigger the source's metadata extraction
    Extract(SourceGet),
}

#[derive(Parser)]
pub struct SourceList {
    /// Filters to apply to the sources list (jsonpath filter)
    #[clap(long = "filter")]
    pub filter: Option<String>,
    /// Field by witch the list will be ordered (default: by name)
    #[clap(long, value_parser = PossibleValuesParser::new(["age", "update", "status", "connector", "name"]))]
    pub sort: Option<String>,
    /// Return list in descending order (if not present the default behaviour is ascending)
    #[clap(long, action)]
    pub desc: bool,
    /// Command output format
    #[clap(short, long, value_parser = PossibleValuesParser::new(&["json", "yaml", "list"]).map(|s| s.parse::<OutputList>().unwrap()))]
    pub output: Option<OutputList>,
    /// To prevent interactive display of the list
    #[clap(long, short, action)]
    pub script: bool,
}

#[derive(Parser)]
pub struct SourceGet {
    /// Source ID (interactive input if not set)
    pub id: Option<String>,
    /// Command output format
    #[clap(short, long, value_parser = PossibleValuesParser::new(&["json", "yaml", "description"]).map(|s| s.parse::<OutputObject>().unwrap()))]
    pub output: Option<OutputObject>,
}

#[derive(Parser)]
pub struct SourceDelete {
    /// Source ID (interactive input if not set)
    pub id: Option<String>,
    /// Never prompt confirmation
    #[clap(long, short, action)]
    pub script: bool,
}

#[derive(Parser)]
pub struct SourceCreate {
    /// Source name
    pub name: String,
    /// Connector ID (interactive input if not set)
    #[clap(long)]
    pub connector_id: Option<String>,
    /// Connector parameters
    #[clap(short, long = "parameter", num_args = 1, value_name = "name=value")]
    pub parameters: Vec<NameValue>,
    /// Command output format
    #[clap(short, long, value_parser = PossibleValuesParser::new(&["json", "yaml", "description"]).map(|s| s.parse::<OutputObject>().unwrap()))]
    pub output: Option<OutputObject>,
}

#[derive(Parser)]
pub struct SourceUpdate {
    /// Source ID (interactive input if not set)
    pub id: Option<String>,
    /// Source name
    pub name: Option<String>,
    /// Connector parameters
    #[clap(short, long = "parameter", num_args = 1, value_name = "name=value")]
    pub parameters: Vec<NameValue>,
    /// Command output format
    #[clap(short, long, value_parser = PossibleValuesParser::new(&["json", "yaml", "description"]).map(|s| s.parse::<OutputObject>().unwrap()))]
    pub output: Option<OutputObject>,
}

#[derive(Parser)]
pub enum DiSubDestCommands {
    /// List all destinations
    #[clap(visible_alias = "ls")]
    List(DestList),
    /// Get destination
    Get(DestGet),
    /// Return the last connection status
    Status(DestGet),
    /// Create a new destination and check connectivity.
    Create(DestCreate),
    /// Update a destination
    Update(DestUpdate),
    /// Delete a destination
    #[clap(visible_alias = "rm")]
    Delete(DestDelete),
}

#[derive(Parser)]
pub struct DestList {
    /// Filters to apply to the destinations list (jsonpath filter)
    #[clap(long = "filter")]
    pub filter: Option<String>,
    /// Field by witch the list will be ordered (default: by name)
    #[clap(long, value_parser = PossibleValuesParser::new(["age", "update", "status", "connector", "name"]))]
    pub sort: Option<String>,
    /// Return list in descending order (if not present the default behaviour is ascending)
    #[clap(long, action)]
    pub desc: bool,
    /// Command output format
    #[clap(short, long, value_parser = PossibleValuesParser::new(&["json", "yaml", "list"]).map(|s| s.parse::<OutputList>().unwrap()))]
    pub output: Option<OutputList>,
    /// To prevent interactive display of the list
    #[clap(long, short, action)]
    pub script: bool,
}

#[derive(Parser)]
pub struct DestGet {
    /// Destination ID
    pub id: Option<String>,
    /// Command output format
    #[clap(short, long, value_parser = PossibleValuesParser::new(&["json", "yaml", "description"]).map(|s| s.parse::<OutputObject>().unwrap()))]
    pub output: Option<OutputObject>,
}

#[derive(Parser)]
pub struct DestDelete {
    /// Destination ID
    pub id: Option<String>,
    /// Never prompt confirmation
    #[clap(long, short, action)]
    pub script: bool,
}

#[derive(Parser)]
pub struct DestCreate {
    /// Destination name
    pub name: String,
    /// Connector ID (interactive input if not set)
    #[clap(long)]
    pub connector_id: Option<String>,
    /// Connector parameters
    #[clap(short, long = "parameter", num_args = 1, value_name = "name=value")]
    pub parameters: Vec<NameValue>,
    /// Command output format
    #[clap(short, long, value_parser = PossibleValuesParser::new(&["json", "yaml", "description"]).map(|s| s.parse::<OutputObject>().unwrap()))]
    pub output: Option<OutputObject>,
}

#[derive(Parser)]
pub struct DestUpdate {
    /// Destination ID (interactive input if not set)
    pub id: Option<String>,
    /// Destination name
    pub name: Option<String>,
    /// Connector parameters
    #[clap(short, long = "parameter", num_args = 1, value_name = "name=value")]
    pub parameters: Vec<NameValue>,
    /// Command output format
    #[clap(short, long, value_parser = PossibleValuesParser::new(&["json", "yaml", "description"]).map(|s| s.parse::<OutputObject>().unwrap()))]
    pub output: Option<OutputObject>,
}

#[derive(Parser)]
pub enum DiSubWorkflowCommands {
    /// List workflows in a service name
    #[clap(visible_alias = "ls")]
    List(WorkflowList),
    /// Get workflow information
    Get(WorkflowGet),
    /// Create a new workflow
    Create(WorkflowCreate),
    /// Delete a workflow
    #[clap(visible_alias = "rm")]
    Delete(WorkflowDelete),
    /// Run a job of the worflow
    Run(WorkflowRun),
    /// Edit a worflow
    Update(WorkflowUpdate),
    /// Enable workflow
    Enable(WorkflowGet),
    /// Disable workflow
    Disable(WorkflowGet),
}

#[derive(Parser)]
pub struct WorkflowList {
    /// Filters to apply to the workflows list (jsonpath filter)
    #[clap(long = "filter")]
    pub filter: Option<String>,
    /// Field by witch the list will be ordered (default: by name)
    #[clap(long, value_parser = PossibleValuesParser::new(["last-execution", "status", "enabled", "source-name", "destination-name"]))]
    pub sort: Option<String>,
    /// Return list in descending order (if not present the default behaviour is ascending)
    #[clap(long, action)]
    pub desc: bool,
    /// Command output format
    #[clap(short, long, value_parser = PossibleValuesParser::new(&["json", "yaml", "list"]).map(|s| s.parse::<OutputList>().unwrap()))]
    pub output: Option<OutputList>,
    /// To prevent interactive display of the list
    #[clap(long, short, action)]
    pub script: bool,
}

#[derive(Parser)]
pub struct WorkflowGet {
    /// Workflow ID (interactive input if not set)
    pub id: Option<String>,
    /// Command output format
    #[clap(short, long, value_parser = PossibleValuesParser::new(&["json", "yaml", "description"]).map(|s| s.parse::<OutputObject>().unwrap()))]
    pub output: Option<OutputObject>,
}

#[derive(Parser)]
pub struct WorkflowRun {
    /// Workflow ID (interactive input if not set)
    pub id: Option<String>,
    /// Command output format
    #[clap(short, long, value_parser = PossibleValuesParser::new(&["json", "yaml", "description"]).map(|s| s.parse::<OutputObject>().unwrap()))]
    pub output: Option<OutputObject>,
}

#[derive(Parser)]
pub struct WorkflowCreate {
    /// Workflow name
    pub name: String,
    /// ID of the source to use (interactive input if not set)
    #[clap(long)]
    pub source_id: Option<String>,
    /// ID of the destination to use (interactive input if not set)
    #[clap(long)]
    pub destination_id: Option<String>,
    /// Workflow description
    #[clap(short, long)]
    pub description: Option<String>,
    /// Schedule of the workflow in cron format
    #[clap(short, long)]
    pub schedule: Option<String>,
    /// Default region where jobs will run
    #[clap(short, long)]
    pub region: String,
    /// Whether workflow is disabled
    #[clap(long, action)]
    pub disabled: bool,
    /// Command output format
    #[clap(short, long, value_parser = PossibleValuesParser::new(&["json", "yaml", "description"]).map(|s| s.parse::<OutputObject>().unwrap()))]
    pub output: Option<OutputObject>,
}

#[derive(Parser)]
pub struct WorkflowDelete {
    /// Workflow ID (interactive input if not set)
    pub id: Option<String>,
    /// Never prompt confirmation
    #[clap(long, short, action)]
    pub script: bool,
}

#[derive(Parser)]
pub struct WorkflowUpdate {
    /// Workflow ID (interactive input if not set)
    pub id: Option<String>,
    #[clap(short, long)]
    /// Workflow name
    pub name: Option<String>,
    /// Workflow description
    #[clap(short, long)]
    pub description: Option<String>,
    /// Schedule of the workflow in cron format
    #[clap(short, long)]
    pub schedule: Option<String>,
    /// Whether workflow is enabled
    #[clap(long, short)]
    pub enabled: Option<bool>,
    /// Command output format
    #[clap(short, long, value_parser = PossibleValuesParser::new(&["json", "yaml", "description"]).map(|s| s.parse::<OutputObject>().unwrap()))]
    pub output: Option<OutputObject>,
}

#[derive(Parser)]
pub enum DiSubJobCommands {
    /// List worflow jobs
    #[clap(visible_alias = "ls")]
    List(JobList),
    /// Get information about a job
    Get(JobGet),
    /// Stop a running job
    Stop(JobStop),
}

#[derive(Parser)]
pub struct JobList {
    /// Workflow ID
    #[clap(long)]
    pub workflow_id: Option<String>,
    /// Filters to apply to the jobs list (jsonpath filter)
    #[clap(long = "filter")]
    pub filter: Option<String>,
    /// Field by witch the list will be ordered (default: by age)
    #[clap(long, value_parser = PossibleValuesParser::new(["age", "status"]))]
    pub sort: Option<String>,
    /// Return list in descending order (if not present the default behaviour is ascending)
    #[clap(long, action)]
    pub desc: bool,
    /// Command output format
    #[clap(short, long, value_parser = PossibleValuesParser::new(&["json", "yaml", "list"]).map(|s| s.parse::<OutputList>().unwrap()))]
    pub output: Option<OutputList>,
    /// To prevent interactive display of the list
    #[clap(long, short, action)]
    pub script: bool,
}

#[derive(Parser)]
pub struct JobGet {
    /// Job ID
    pub id: Option<String>,
    /// Workflow ID
    #[clap(long)]
    pub workflow_id: Option<String>,
    /// Command output format
    #[clap(short, long, value_parser = PossibleValuesParser::new(&["json", "yaml", "description"]).map(|s| s.parse::<OutputObject>().unwrap()))]
    pub output: Option<OutputObject>,
}

#[derive(Parser)]
pub struct JobStop {
    /// Job ID
    pub id: Option<String>,
    /// Workflow ID
    #[clap(long)]
    pub workflow_id: Option<String>,
}

#[derive(Parser)]
pub enum DiSubSourceConnectorCommands {
    /// List source connectors
    #[clap(visible_alias = "ls")]
    List(SourceConnectorList),
    /// Get source connector
    Get(SourceConnectorGet),
}

#[derive(Parser)]
pub struct SourceConnectorList {
    /// Command output format
    #[clap(short, long, value_parser = PossibleValuesParser::new(&["json", "yaml", "list"]).map(|s| s.parse::<OutputList>().unwrap()))]
    pub output: Option<OutputList>,
}

#[derive(Parser)]
pub struct SourceConnectorGet {
    /// Source connector id
    pub id: Option<String>,
    /// Command output format
    #[clap(short, long, value_parser = PossibleValuesParser::new(&["json", "yaml", "description"]).map(|s| s.parse::<OutputObject>().unwrap()))]
    pub output: Option<OutputObject>,
}

#[derive(Parser)]
pub enum DiSubDestConnectorCommands {
    /// List destination connectors
    #[clap(visible_alias = "ls")]
    List(DestConnectorList),
    /// Get destination connector
    Get(DestConnectorGet),
}

#[derive(Parser)]
pub struct DestConnectorList {
    /// Command output format
    #[clap(short, long, value_parser = PossibleValuesParser::new(&["json", "yaml", "list"]).map(|s| s.parse::<OutputList>().unwrap()))]
    pub output: Option<OutputList>,
}

#[derive(Parser)]
pub struct DestConnectorGet {
    /// Destination connector ident
    pub id: Option<String>,
    /// Command output format
    #[clap(short, long, value_parser = PossibleValuesParser::new(&["json", "yaml", "description"]).map(|s| s.parse::<OutputObject>().unwrap()))]
    pub output: Option<OutputObject>,
}
