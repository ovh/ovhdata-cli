use chrono::{DateTime, Utc};
use descriptor::Descriptor;
use serde::{Deserialize, Serialize};

use crate::model::utils::{DescribedDateTime};

#[derive(Debug, Clone, Deserialize, Serialize, Descriptor)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub name: String,
    pub value: String
}

#[derive(Debug, Clone, Deserialize, Serialize, Descriptor)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub status: String,
    #[descriptor(into = DescribedDateTime)]
    pub date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Descriptor)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDetails {
    pub code: String,
    pub description: String,
}