use chrono::{DateTime, Utc};

use descriptor::Descriptor;
use serde::{Deserialize, Serialize};

use crate::model::di::common::Parameter;
use crate::model::utils::{AgeEntity, DescribedDateTime};

#[derive(Debug, Clone, Deserialize, Serialize, Descriptor)]
#[serde(rename_all = "camelCase")]
#[descriptor(default_headers = ["name", "id", "connector_id", "status", "age", "last_update"])]
#[descriptor(extra_fields = AgeEntity)]
pub struct Source {
    pub id: String,
    pub name: String,
    pub status: String,
    #[descriptor(into = DescribedDateTime)]
    pub creation_date: DateTime<Utc>,
    #[descriptor(into = DescribedDateTime)]
    pub last_update_date: Option<DateTime<Utc>>,
    pub connector_id: String,
    #[descriptor(output_table)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Descriptor)]
#[serde(rename_all = "camelCase")]
pub struct SourceSpec {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    #[descriptor(output_table)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<Parameter>,
}

#[cfg(test)]
mod tests {
    use descriptor::{object_describe_to_string, table_describe_to_string};

    use crate::model::di::source::{Parameter, Source};
    use crate::utils::date::datetime_micro;

    fn create_source() -> Source {
        Source {
            id: "d2671df0-6718-400e-a3d5-1242a49d464c".to_string(),
            name: "test-source".to_string(),
            connector_id: "d2671df0-6718-400e-a3d5-1242a49d464f".to_string(),
            creation_date: datetime_micro(2021, 6, 3, 12, 22, 46, 107055),
            last_update_date: Some(datetime_micro(2021, 6, 3, 12, 22, 46, 107055)),
            status: "CONNECTION_SUCCEED".to_string(),
            parameters: vec![
                Parameter {
                    name: "parameter_1".to_string(),
                    value: "value_1".to_string(),
                },
                Parameter {
                    name: "parameter_2".to_string(),
                    value: "value_2".to_string(),
                },
            ],
        }
    }

    #[test]
    fn describe_source() {
        let source = create_source();
        let describe = object_describe_to_string(&source).unwrap();

        println!("{}", describe);
        assert_eq!(
            describe,
            r#"
Id:               d2671df0-6718-400e-a3d5-1242a49d464c
Name:             test-source
Status:           CONNECTION_SUCCEED
Creation Date:    03-06-21 12:22:46
Last Update Date: 03-06-21 12:22:46
Connector Id:     d2671df0-6718-400e-a3d5-1242a49d464f
Parameters:
  NAME          VALUE
  parameter_1   value_1
  parameter_2   value_2
"#
        )
    }

    #[test]
    fn table_source() {
        let source = create_source();
        let table = table_describe_to_string(std::slice::from_ref(&source)).unwrap();
        println!("{}", table);
        assert_eq!(
            table,
            r#"NAME        ID                                   CONNECTOR_ID                         STATUS             AGE LAST_UPDATE
test-source d2671df0-6718-400e-a3d5-1242a49d464c d2671df0-6718-400e-a3d5-1242a49d464f CONNECTION_SUCCEED 34d 34d
"#
        )
    }
}
