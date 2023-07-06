use chrono::{DateTime, Utc};
use descriptor::Descriptor;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::model::utils::DescribedDateTime;

#[derive(Debug, Clone, Deserialize, Serialize, Descriptor)]
#[serde(rename_all = "camelCase")]
// Internal use only (skip write for description output)
#[descriptor(default_headers = ["name", "value"])]
pub struct Parameter {
    pub name: String,
    pub value: String,

    // Internal use only (skip serialization for api, json & yaml output)
    #[serde(default, skip_serializing)]
    pub secret: bool,
}

pub struct ParametersWrapper(pub Vec<Parameter>);

impl fmt::Display for ParametersWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|param| {
                    let value = if param.secret { "[secret_hidden]" } else { &param.value };

                    format!("--parameter {}={}", param.name, value)
                })
                .collect::<Vec<_>>()
                .join(" ")
        )
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
