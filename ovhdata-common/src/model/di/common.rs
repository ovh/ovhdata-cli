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

pub fn parameters_as_string(parameters: &Vec<Parameter>) -> String {
    let mut cmd = String::new();
    for parameter in parameters.iter() {
        cmd.push_str(&format!(" --parameter {}={}", parameter.name, parameter.value));
    }
    cmd
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