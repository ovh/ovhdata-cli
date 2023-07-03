use descriptor::Descriptor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Descriptor)]
#[serde(rename_all = "camelCase")]
#[descriptor(default_headers = ["name", "id", "version"])]
pub struct SourceConnector {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    #[descriptor(map = crate::utils::style::underlined, resolve_option)]
    pub documentation_url: Option<String>,
    #[descriptor(output_table)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ConnectorParameter>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Descriptor)]
#[serde(rename_all = "camelCase")]
#[descriptor(default_headers = ["name", "id", "version"])]
pub struct DestinationConnector {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    #[descriptor(map = crate::utils::style::underlined, resolve_option)]
    pub documentation_url: Option<String>,
    #[descriptor(output_table)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ConnectorParameter>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Descriptor)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorParameter {
    pub name: String,
    #[serde(with = "crate::utils::serde::custom_serde_string")]
    pub default: Option<String>,
    pub mandatory: bool,
    #[serde(rename = "type")]
    #[descriptor(rename_header = "TYPE")]
    pub type_name: String,
    pub validator: Option<ConnectorValidator>,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Descriptor)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorValidator {
    pub min: i64,
    pub max: i64,
    pub regex: Option<String>,
}
