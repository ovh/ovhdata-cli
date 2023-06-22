use std::fmt::{Debug, Display};

use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("{0}")]
    Custom(String),
    #[error("Invalid user input")]
    UserInput,
    #[error("Config error: {0}")]
    Config(#[from] crate::config::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Request error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Config error: {0}")]
    OvhcloudConfig(#[from] ovhdata_common::config::Error),
    #[error("Data API error: {0}")]
    DataApi(#[from] ovhdata_common::api::Error),
}

impl Error {
    pub fn custom(err: impl Display) -> Self {
        Self::Custom(err.to_string())
    }
}

