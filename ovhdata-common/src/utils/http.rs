use std::fmt::{Display, Formatter, Result};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use thiserror::Error as ThisError;

#[derive(Debug, Clone)]
pub struct UserAgent {
    pub name: String,
    pub version: String,
    pub info: String,
}

impl UserAgent {
    pub fn new(name: &str, version: &str) -> Self {
        UserAgent {
            name: String::from(name),
            version: String::from(version),
            info: String::from(std::env::consts::OS),
        }
    }
}

impl Display for UserAgent {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(format!("{}/{} ({})", &self.name, &self.version, &self.info).as_str())
    }
}

#[derive(ThisError, Debug)]
pub enum UrlError {
    #[error("url {0} cannot be a base")]
    CannotBeABase(url::Url),
    #[error("url {0} cannot be parsed {1}")]
    ParseError(String, url::ParseError),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Url(pub url::Url);

impl Url {
    pub fn with_segment(&self, path: &[&str]) -> Url {
        let mut cloned_url = self.clone();
        for segment in path {
            // We can unwrap because '!cannot_be_a_base()' is cover by the constructor of Url
            cloned_url.0.path_segments_mut().unwrap().push(segment);
        }
        cloned_url
    }
}

impl Display for Url {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.0.fmt(f)
    }
}

impl FromStr for Url {
    type Err = UrlError;
    fn from_str(input: &str) -> std::result::Result<Url, UrlError> {
        match url::Url::parse(input) {
            Ok(url) if !url.cannot_be_a_base() => Ok(Url(url)),
            Ok(url) => Err(UrlError::CannotBeABase(url)),
            Err(error) => Err(UrlError::ParseError(input.to_string(), error.into())),
        }
    }
}

impl descriptor::Describe for Url {
    fn to_field(&self, _field_name: &str) -> String {
        self.to_string()
    }
}

impl From<url::Url> for Url {
    fn from(url: url::Url) -> Self {
        Url(url)
    }
}

impl Into<url::Url> for Url {
    fn into(self) -> url::Url {
        self.0
    }
}

impl Deref for Url {
    type Target = url::Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Url {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
