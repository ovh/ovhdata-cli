use jsonpath_rust::{JsonPathFinder, JsonPathInst};
use serde_json::Value;
use std::str::FromStr;

use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum JsonPathError {
    #[error("json_parse didnt find any values")]
    NullValue(),
    #[error("jsonpath error: {0}")]
    JsonPath(String),
}

pub fn json_parse_task(json: Value, path: Option<String>) -> Result<Value, JsonPathError> {
    if path.is_some() {
        let json_path = JsonPathInst::from_str(path.unwrap().as_str()).map_err(JsonPathError::JsonPath)?;
        let finder = JsonPathFinder::new(Box::new(json), Box::new(json_path));

        let result = finder.find();
        if result.is_null() {
            return Err(JsonPathError::NullValue());
        }
        Ok(result)
    } else {
        Ok(json)
    }
}
