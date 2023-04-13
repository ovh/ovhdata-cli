use descriptor::Descriptor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Descriptor)]
#[serde(rename_all = "camelCase")]
pub struct CredentialDetails {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_ips: Vec<String>,
    pub application_id: u64,
    pub creation: String,
    pub credential_id: u64,
    pub expiration: Option<String>,
    pub last_use: Option<String>,
    pub ovh_support: bool,
    pub status: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<AccessRule>
}

#[derive(Debug, Clone, Deserialize, Serialize, Descriptor)]
#[serde(rename_all = "camelCase")]
pub struct AccessRule {
    pub method: String,
    pub path: String,
}