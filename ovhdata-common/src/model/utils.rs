use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};
use descriptor::{Describe, Descriptor};
use serde::{Deserialize, Serialize};
use crate::utils::util::{age, duration};

use crate::model::di::source::Source;
use crate::model::di::destination::Destination;
use crate::model::di::job::Job;
use crate::model::di::workflow::Workflow;

pub const DEFAULT_PAGE_SIZE: u32 = 100;

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseError {
    pub message: String,
}

//impl oauth2::ErrorResponse for ResponseError {}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Descriptor)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    #[descriptor(rename_description = "Object Storage to Job")]
    Pull,
    #[descriptor(rename_description = "Job to Object Storage")]
    Push,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    pub items: Vec<T>,
    pub metadata: PageMetadata,
}

impl<T> Page<T> {
    pub fn new_unique(items: Vec<T>) -> Self {
        let total = items.len() as u32;
        Self {
            items,
            metadata: PageMetadata {
                current_page: 0,
                page_count: 1,
                total,
                links: PageMetadataLinks { next: None },
            },
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageMetadata {
    pub current_page: u32,
    pub page_count: u32,
    pub total: u32,
    pub links: PageMetadataLinks,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageMetadataLinks {
    pub next: Option<String>,
}

#[derive(Descriptor)]
pub struct AgeEntity {
    #[descriptor(skip_description, rename_header = "AGE")]
    pub age: Option<String>,
    #[descriptor(skip_description, rename_header = "LAST_UPDATE")]
    pub last_update: Option<String>,
    #[descriptor(skip_description, rename_header = "DURATION")]
    pub duration: Option<String>,
    #[descriptor(skip_description, rename_header = "AST_EXECUTION")]
    pub last_execution: Option<String>,
}

impl From<&Source> for AgeEntity {
    fn from(s: &Source) -> Self {
        Self {
            age: Some(age(&s.creation_date)),
            last_update: Some(age(&s.last_update_date.unwrap_or(Utc::now()))),
            duration: None,
            last_execution: None,
        }
    }
}

impl From<&Destination> for AgeEntity {
    fn from(d: &Destination) -> Self {
        Self {
            age: Some(age(&d.creation_date)),
            last_update: Some(age(&d.last_update_date.unwrap_or(Utc::now()))),
            duration:None,
            last_execution: None,
        }
    }
}

impl From<&Job> for AgeEntity {
    fn from(j: &Job) -> Self {
        Self {
            age: Some(age(&j.created_at)),
            last_update: None,
            duration:Some(duration(&j.started_at, &j.ended_at)),
            last_execution: None,
        }
    }
}

impl From<&Workflow> for AgeEntity {
    fn from(w: &Workflow) -> Self {
        Self {
            age: None,
            last_update: None,
            duration:None,
            last_execution: Some(age(&w.last_execution_date.unwrap_or(Utc::now()))),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Descriptor, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub message: String,
}

pub struct DescribedDateTime {
    pub datetime: DateTime<Utc>,
}

impl From<&DateTime<Utc>> for DescribedDateTime {
    fn from(datetime: &DateTime<Utc>) -> Self {
        DescribedDateTime {
            datetime: *datetime,
        }
    }
}

impl From<&Option<DateTime<Utc>>> for DescribedDateTime {
    fn from(optional_datetime: &Option<DateTime<Utc>>) -> Self {
        let datetime = optional_datetime.unwrap_or(Utc::now());
        DescribedDateTime {
            datetime: datetime,
        }
    }
}

impl Describe for DescribedDateTime {
    fn to_field(&self, _: &str) -> String {
        self.datetime.format("%d-%m-%y %H:%M:%S").to_string()
    }
    fn default_headers() -> Vec<String> {
        Self::headers()
    }
    fn headers() -> Vec<String> {
        vec![]
    }
    fn header_name(_: &str) -> Option<String> {
        None
    }
}

pub struct OptionDescribedDateTime {
    pub datetime: Option<DateTime<Utc>>,
}

impl From<&Option<DateTime<Utc>>> for OptionDescribedDateTime {
    fn from(datetime: &Option<DateTime<Utc>>) -> Self {
        OptionDescribedDateTime {
            datetime: *datetime,
        }
    }
}

impl Describe for OptionDescribedDateTime {
    fn to_field(&self, _: &str) -> String {
        match self.datetime {
            Some(d) => d.format("%d-%m-%y %H:%M:%S").to_string(),
            None => "~".to_string(),
        }
    }
    fn default_headers() -> Vec<String> {
        Self::headers()
    }
    fn headers() -> Vec<String> {
        vec![]
    }
    fn header_name(_: &str) -> Option<String> {
        None
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenericResponse {
    pub message: String,
}
