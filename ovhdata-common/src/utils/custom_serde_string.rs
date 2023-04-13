/**
 * Transform none string value into Option<None>
 * Used fox fix API
 */
pub mod custom_serde_string {
    use serde::{self, Deserialize, Serializer, Deserializer};

    pub fn serialize<S>(
        maybe_value: &Option<String>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        if let Some(value) = maybe_value {
            serializer.serialize_str(value)
        } else {
            serializer.serialize_str("none")
        }
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<String>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        if s.eq("none") || s.is_empty() {
            Ok(None)
        } else {
            Ok(Some(s))
        }
    }
}