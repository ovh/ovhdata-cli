use chrono::{DateTime, Utc};

use descriptor::Descriptor;
use serde::{Deserialize, Serialize};

use crate::model::utils::{AgeEntity, DescribedDateTime};

#[derive(Debug, Clone, Deserialize, Serialize, Descriptor)]
#[serde(rename_all = "camelCase")]
#[descriptor(default_headers = ["id", "status", "age", "duration"])]
#[descriptor(extra_fields = AgeEntity)]
pub struct Job {
    pub id: String,
    pub status: String,
    #[descriptor(into = DescribedDateTime)]
    pub created_at: DateTime<Utc>,
    #[descriptor(into = DescribedDateTime)]
    pub started_at: Option<DateTime<Utc>>,
    #[descriptor(into = DescribedDateTime)]
    pub ended_at: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod tests {
    use descriptor::{object_describe_to_string, table_describe_to_string};

    use crate::model::di::job::Job;
    use crate::utils::date::datetime_micro;

    fn create_job() -> Job {
        Job {
            id: "d2671df0-6718-400e-a3d5-1242a49d464c".to_string(),
            status: "COMPLETED".to_string(),
            created_at: datetime_micro(2021, 6, 3, 12, 22, 46, 107055),
            started_at: Some(datetime_micro(2021, 6, 3, 12, 23, 46, 107055)),
            ended_at: Some(datetime_micro(2021, 6, 3, 12, 29, 46, 107055)),
        }
    }

    #[test]
    fn describe_job() {
        let job = create_job();
        let describe = object_describe_to_string(&job).unwrap();

        println!("{}", describe);
        assert_eq!(
            describe,
            r#"
Id:         d2671df0-6718-400e-a3d5-1242a49d464c
Status:     COMPLETED
Created At: 03-06-21 12:22:46
Started At: 03-06-21 12:23:46
Ended At:   03-06-21 12:29:46
"#
        )
    }

    #[test]
    fn table_job() {
        let job = create_job();
        let table = table_describe_to_string(std::slice::from_ref(&job)).unwrap();
        println!("{}", table);
        assert_eq!(
            table,
            r#"ID                                   STATUS    AGE DURATION
d2671df0-6718-400e-a3d5-1242a49d464c COMPLETED 34d 6m
"#
        )
    }
}
