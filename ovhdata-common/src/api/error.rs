use reqwest::StatusCode;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("request error: {0}")]
    Request(reqwest::Error),
    #[error("response error: {}: {1}", .0.as_u16())]
    Response(StatusCode, String),
    #[error("deserialize error: {0}, string is {1}")]
    DeserializeContent(serde_json::Error, String),
}

pub type Result<T> = std::result::Result<T, Error>;

