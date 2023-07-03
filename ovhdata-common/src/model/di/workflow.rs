use crate::model::di::common::{ErrorDetails, Parameter};
use crate::model::utils::{AgeEntity, DescribedDateTime};
use chrono::{DateTime, Utc};
use descriptor::Descriptor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Descriptor)]
#[serde(rename_all = "camelCase")]
#[descriptor(default_headers = ["name", "enabled", "id", "source_name", "destination_name", "schedule", "last_execution", "status"])]
#[descriptor(extra_fields = AgeEntity)]
pub struct Workflow {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub region: String,
    pub source_id: Option<String>,
    pub source_name: Option<String>,
    pub destination_id: Option<String>,
    pub destination_name: Option<String>,
    pub parameters: Vec<Parameter>,
    #[descriptor(into = DescribedDateTime)]
    pub last_execution_date: Option<DateTime<Utc>>,
    pub schedule: Option<String>,
    pub enabled: bool,
    pub status: Option<String>,
    pub error_details: Option<ErrorDetails>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Descriptor)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowSpec {
    pub name: String,
    pub region: String,
    pub description: Option<String>,
    pub source_id: String,
    pub destination_id: String,
    pub schedule: Option<String>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, Descriptor)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowPatch {
    pub name: Option<String>,
    pub description: Option<String>,
    pub schedule: Option<String>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Descriptor)]
#[serde(rename_all = "camelCase")]
pub struct JobPost {
    // api mandatory parameter, not used yet
    pub parameters: Vec<String>,
}

#[cfg(test)]
mod tests {
    use descriptor::{object_describe_to_string, table_describe_to_string};

    use crate::model::di::workflow::Workflow;
    use crate::utils::date::datetime_micro;

    fn create_workflow() -> Workflow {
        Workflow {
            id: "d2671df0-6718-400e-a3d5-1242a49d464c".to_string(),
            name: "test-workflow".to_string(),
            region: "GRA".to_string(),
            description: Some("a worflow use for test purpose".to_string()),
            source_id: Some("d2671df0-6718-400e-a3d5-1242a49d464f".to_string()),
            source_name: Some("source-name".to_string()),
            destination_id: Some("d2671df0-6718-400e-a3d5-1242a49d464b".to_string()),
            destination_name: Some("destination-name".to_string()),
            parameters: Vec::new(),
            schedule: Some("5 4 * * *".to_string()),
            enabled: true,
            last_execution_date: Some(datetime_micro(2021, 6, 3, 12, 22, 46, 107055)),
            status: Some("READY".to_string()),
            error_details: None,
        }
    }

    #[test]
    fn describe_workflow() {
        let workflow = create_workflow();
        let describe = object_describe_to_string(&workflow).unwrap();

        println!("{}", describe);
        assert_eq!(
            describe,
            r#"
Id:                  d2671df0-6718-400e-a3d5-1242a49d464c
Name:                test-workflow
Description:         a worflow use for test purpose
Region:              GRA
Source Id:           d2671df0-6718-400e-a3d5-1242a49d464f
Source Name:         source-name
Destination Id:      d2671df0-6718-400e-a3d5-1242a49d464b
Destination Name:    destination-name
Parameters:          ~
Last Execution Date: 03-06-21 12:22:46
Schedule:            5 4 * * *
Enabled:             true
Status:              READY
Error Details:       ~
"#
        )
    }

    #[test]
    fn table_workflow() {
        let workflow = create_workflow();
        let table = table_describe_to_string(std::slice::from_ref(&workflow)).unwrap();
        println!("{}", table);
        assert_eq!(
            table,
            r#"NAME          ENABLED ID                                   SOURCE_NAME DESTINATION_NAME SCHEDULE  LAST_EXECUTION STATUS
test-workflow true    d2671df0-6718-400e-a3d5-1242a49d464c source-name destination-name 5 4 * * * 34d            READY
"#
        )
    }
}
