use chrono::{DateTime, Utc};
use descriptor::Descriptor;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::model::utils::DescribedDateTime;
use ovhdata_macros::PrintObjectCompletely;

// Important, read this before implement this trait.
//
// The printer use the library descriptor for print structures to the console
// Some structs may have secret, so the Printer expect to have the trait EnsureSecret implemented
// You have 2 options:
// 1. Your struct does not contain secret, your struct must derive from PrintObjectCompletely who
// returns a clone of the reference given in input.
//
// 2. Your struct contains secret, you have to implement this trait (likes Parameter) in order to
// hide them with stars or the keyword [secret_hidden]
//
// Keep in mind, secrets are secrets, keep them secret ;-)
pub trait EnsureSecret<T> {
    fn hide_secrets(&self) -> T;
}

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

impl EnsureSecret<Parameter> for Parameter {
    fn hide_secrets(&self) -> Parameter {
        let mut out = self.clone();
        if self.secret {
            out.value = "[secret_hidden]".to_string();
        }
        out
    }
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

#[derive(Debug, Clone, Deserialize, Serialize, Descriptor, PrintObjectCompletely)]
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
