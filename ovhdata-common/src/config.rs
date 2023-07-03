use std::collections::hash_map::HashMap;
use std::convert::TryFrom;
use std::path::PathBuf;

use descriptor::{Describe, Descriptor};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug, PartialEq)]
pub enum Error {
    #[error("Unable to find any config with name: {}", .0.as_str())]
    ConfigNameNotFound(ConfigName),
    #[error("{}", .0.as_str())]
    DeserializationError(String),
    #[error("{}", .0.as_str())]
    ReadFileError(String),
    #[error("Unable to save configuration into file : {0:?}, message was : {1}")]
    SaveConfigError(PathBuf, String),
}

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct ConfigName(String);

impl Describe for ConfigName {
    fn to_field(&self, _: &str) -> String {
        self.0.to_string()
    }
}

impl<S> From<S> for ConfigName
where
    S: Into<String>,
{
    fn from(name: S) -> Self {
        ConfigName(name.into())
    }
}

impl ConfigName {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct AllConfig {
    pub current_config_name: ConfigName,
    pub configs: HashMap<ConfigName, Config>,
}

// Convert a single config into a group of config with the single one as 'default'
impl From<Config> for AllConfig {
    fn from(config: Config) -> Self {
        AllConfig {
            current_config_name: ConfigName::from("default"),
            configs: maplit::hashmap!(ConfigName::from("default") => config),
        }
    }
}

impl TryFrom<PathBuf> for AllConfig {
    type Error = Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let config_as_string = std::fs::read_to_string(path)
            .map_err(|error| Error::ReadFileError(error.to_string()))?;
        let maybe_all_config = serde_json::from_str::<AllConfig>(config_as_string.as_str())
            .map_err(|error| Error::DeserializationError(error.to_string()));
        match maybe_all_config {
            Err(_) => {
                // If we couldn't deserialize a group of config maybe it's only a single one
                let single_config = serde_json::from_str::<Config>(config_as_string.as_str())
                    .map_err(|error| Error::DeserializationError(error.to_string()))?;
                Ok(single_config.into())
            }
            other => other,
        }
    }
}

impl TryFrom<Value> for AllConfig {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
            .map_err(|error| Error::DeserializationError(error.to_string()))
    }
}

impl AllConfig {
    pub fn save(&self, path: PathBuf) -> Result<(), Error> {
        let bytes = match serde_json::to_vec_pretty(self) {
            Ok(bytes) => bytes,
            Err(err) => Err(Error::SaveConfigError(path.clone(), err.to_string()))?,
        };

        std::fs::write(path.clone(), bytes)
            .map_err(|err| Error::SaveConfigError(path.clone(), err.to_string()))
    }

    pub fn get_current_config_name(&self) -> &ConfigName {
        &self.current_config_name
    }

    pub fn get_current_config(&self) -> Result<&Config, Error> {
        let current_config = &self.current_config_name;
        self.configs
            .get(current_config)
            .ok_or(Error::ConfigNameNotFound(current_config.clone()))
    }

    pub fn get_config<N>(&self, config_name: N) -> Option<&Config>
    where
        N: Into<ConfigName>,
    {
        let config_name: ConfigName = config_name.into();
        self.configs.get(&config_name)
    }

    pub fn set_current_config<N>(&mut self, config_name: N) -> Result<&Config, Error>
    where
        N: Into<ConfigName>,
    {
        let config_name: ConfigName = config_name.into();
        if self.configs.contains_key(&config_name) {
            self.current_config_name = config_name;
            self.get_current_config()
        } else {
            Err(Error::ConfigNameNotFound(config_name))
        }
    }

    pub fn add_config(&mut self, config_name: impl Into<ConfigName>, config: impl Into<Config>) {
        self.configs.insert(config_name.into(), config.into());
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Descriptor)]
pub struct Config {
    #[descriptor(skip)]
    pub cli_release_url: String,
    #[descriptor(skip)]
    pub auth_method: String,
    #[descriptor(rename_header = "ENDPOINT")]
    pub ovhapiv6: ConfigOVHapiV6,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Descriptor)]
pub struct ConfigOVHapiV6 {
    #[descriptor(rename_header = "URL")]
    pub endpoint_url: String,
    #[descriptor(skip)]
    pub create_token_url: String,
}
