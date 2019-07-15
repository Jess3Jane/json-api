use crate::Meta;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct JsonApi {
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<Meta>,
}

impl Default for JsonApi {
    fn default() -> Self {
        Self {
            version: None,
            meta: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;

    #[test]
    fn serde_empty() {
        let j1 : JsonApi = Default::default();
        let s = serde_json::to_string(&j1).unwrap();
        assert_eq!(s, "{}");
        let j2 = serde_json::from_str(&s).unwrap();
        assert_eq!(j1, j2);
    }

    #[test]
    fn serde_full() {
        let j1 = JsonApi {
            version: Some("a".into()),
            meta: Some(Meta::new()),
        };
        let s = serde_json::to_string(&j1).unwrap();
        assert_eq!(s, "{\"version\":\"a\",\"meta\":{}}");
        let j2 = serde_json::from_str(&s).unwrap();
        assert_eq!(j1, j2);
    }
}
