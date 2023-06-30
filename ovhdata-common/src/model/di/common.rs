use chrono::{DateTime, Utc};
use descriptor::Descriptor;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::model::utils::{DescribedDateTime};

#[derive(Debug, Clone, Deserialize, Serialize, Descriptor)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub name: String,
    pub value: String
}

pub struct ParametersWrapper(pub Vec<Parameter>);

impl fmt::Display for ParametersWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.iter()
            .map(|param| format!("--parameter {}={}", param.name, param.value))
            .collect::<Vec<_>>()
            .join(" "))
    }
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