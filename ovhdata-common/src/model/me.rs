use descriptor::Descriptor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Descriptor)]
#[serde(rename_all = "camelCase")]
pub struct Me {
    pub user: Option<String>,
    pub description: Option<String>,
    pub roles: Vec<String>,
}

#[test]
fn deserialize_me() {
    use descriptor::{object_describe_to_string};

    let body_string = r#"{"method":"account","user":null,"description":null,"roles":["ADMIN"],"allowedRoutes":[{"method":"GET","path":"*"},{"method":"PUT","path":"*"},{"method":"POST","path":"*"},{"method":"DELETE","path":"*"}]}"#;
    let me: Me = serde_json::from_str(body_string).unwrap();

    let describe = object_describe_to_string(&me).unwrap();

    println!("{}", describe);
    assert_eq!(
        describe,
        r#"
User:        ~
Description: ~
Roles:
- ADMIN
"#
    )
}
