use chrono::{DateTime, Utc};

use descriptor::Descriptor;
use serde::{Deserialize, Serialize};

use crate::model::utils::{OptionDescribedDateTime};

pub type TablesMeta = Vec<TableMeta>;

#[derive(Debug, Clone, Deserialize, Serialize, Descriptor)]
#[serde(rename_all = "camelCase")]
pub struct TableMeta {
    pub table_name: String,
    pub status: String,
    pub error: Option<String>,
    pub error_code: Option<String>,
    #[descriptor(into = OptionDescribedDateTime)]
    pub started_at: Option<DateTime<Utc>>,
    #[descriptor(into = OptionDescribedDateTime)]
    pub ended_at: Option<DateTime<Utc>>,
    #[descriptor(output_table)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<Metadata>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Descriptor)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub name: String,
    #[serde(rename = "type")]
    #[descriptor(rename_header = "TYPE")]
    pub type_name: String,
    pub cardinality: i64,
    pub min: i64,
    pub max: i64,
}


#[cfg(test)]
mod tests {
    use descriptor::{object_describe_to_string};

    use crate::model::di::source_metadata::{TableMeta, Metadata};
    use crate::utils::util::datetime_micro;

    fn create_metadata () -> TableMeta {
        TableMeta {
            table_name: "table-name".to_string(),
            status: "SUCCESS".to_string(),
            error: None,
            error_code: None,
            started_at: Some(datetime_micro(2021, 6, 3, 12, 22, 46, 107055)),
            ended_at: Some(datetime_micro(2021, 6, 3, 12, 24, 46, 107055)),
            metadata: vec![
                Metadata {
                    name: "id".to_string(),
                    type_name: "num".to_string(),
                    cardinality: 788,
                    min: 0,
                    max: 788
                },
                Metadata {
                    name: "some_column".to_string(),
                    type_name: "txt".to_string(),
                    cardinality: 580,
                    min: 0,
                    max: 911
                },
            ]
        }
    }

    #[test]
    fn describe_metadata() {
        let describe_metadata = create_metadata();
        let describe = object_describe_to_string(&describe_metadata).unwrap();

        println!("{}", describe);
        assert_eq!(
            describe,
            r#"
Table Name: table-name
Status:     SUCCESS
Error:      ~
Error Code: ~
Started At: 03-06-21 12:22:46
Ended At:   03-06-21 12:24:46
Metadata:
  NAME          TYPE   CARDINALITY   MIN   MAX
  id            num    788           0     788
  some_column   txt    580           0     911
"#
        )
    }

    fn create_metadata_empty () -> TableMeta {
        TableMeta {
            table_name: "table-name".to_string(),
            status: "PROCESSING".to_string(),
            error: None,
            error_code: None,
            started_at: Some(datetime_micro(2021, 6, 3, 12, 22, 46, 107055)),
            ended_at: None,
            metadata: Vec::new()
        }
    }

    #[test]
    fn describe_metadata_empty() {
        let describe_metadata = create_metadata_empty();
        let describe = object_describe_to_string(&describe_metadata).unwrap();

        println!("{}", describe);
        assert_eq!(
            describe,
            r#"
Table Name: table-name
Status:     PROCESSING
Error:      ~
Error Code: ~
Started At: 03-06-21 12:22:46
Ended At:   ~
Metadata:
  NAME   TYPE   CARDINALITY   MIN   MAXEmpty list
"#
        )
    }

    fn create_metadata_error () -> TableMeta {
        TableMeta {
            table_name: "table-name".to_string(),
            status: "ERROR".to_string(),
            error: Some("Metadata were not extracted".to_string()),
            error_code: Some("404".to_string()),
            started_at: None,
            ended_at: None,
            metadata: Vec::new()
        }
    }

    #[test]
    fn describe_metadata_error() {
        let describe_metadata = create_metadata_error();
        let describe = object_describe_to_string(&describe_metadata).unwrap();

        println!("{}", describe);
        assert_eq!(
            describe,
            r#"
Table Name: table-name
Status:     ERROR
Error:      Metadata were not extracted
Error Code: 404
Started At: ~
Ended At:   ~
Metadata:
  NAME   TYPE   CARDINALITY   MIN   MAXEmpty list
"#
        )
    }

}
