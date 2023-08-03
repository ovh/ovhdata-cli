use reqwest::StatusCode;
use thiserror::Error as ThisError;
use crate::utils::jsonpath::JsonPathError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("request error: {0}")]
    Request(reqwest::Error),
    #[error("response error: {}: {1}", .0.as_u16())]
    Response(StatusCode, String),
    #[error("deserialize error: {0}, string is {1}")]
    DeserializeContent(serde_json::Error, String),
    #[error("filtering error: {0}")]
    FilterContent(JsonPathError),
}

pub type Result<T> = std::result::Result<T, Error>;
