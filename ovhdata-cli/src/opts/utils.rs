use std::str::FromStr;
use std::fmt;
use std::fmt::{Display, Formatter};
use super::{ParseError, ParseResult};
use ovhdata_common::model::di::common::Parameter;

#[derive(Clone)]
pub struct NameValue {
     pub name: String,
      pub value: String,
}

/// Parse string like `name=value
impl FromStr for NameValue {
    type Err = ParseError;

    fn from_str(s: &str) -> ParseResult<Self> {
        let mut splits = s.splitn(2, '=');
        let name = splits.next().ok_or(ParseError::NameValueParse)?.to_string();
        let value = splits.next().ok_or(ParseError::NameValueParse)?.to_string();
        Ok(Self { name, value })
    }
}

impl Display for NameValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}={}", &self.name, &self.value))
    }
}

impl From<NameValue> for Parameter {
    fn from(env: NameValue) -> Self {
        Parameter {
            name: env.name.clone(),
            value: env.value.clone(),
        }
    }
}