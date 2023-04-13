use descriptor::{Descriptor};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Descriptor)]
#[descriptor(default_headers = ["project_id", "description"])]
pub struct Project {
    pub project_id: String,
    pub description: String,
}