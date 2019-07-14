use crate::{Meta, Relationships, Links};
use serde_derive::{Serialize, Deserialize}; 
use serde_json::Value;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct GenericObject {
    id: String,
    #[serde(rename = "type")]
    kind: String,
    #[serde(skip_serializing_if = "Value::is_null", default)]
    attributes: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    relationships: Option<Relationships>,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<Meta>,
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;

    #[test]
    fn serde_empty() {
        let g1 = GenericObject {
            id: "a".into(),
            kind: "b".into(),
            attributes: Value::Null,
            relationships: None,
            links: None,
            meta: None,
        };
        let s = serde_json::to_string(&g1).unwrap();
        assert_eq!(s, "{\"id\":\"a\",\"type\":\"b\"}");
        let g2 = serde_json::from_str(&s).unwrap();
        assert_eq!(g1, g2);
    }

    #[test]
    fn serde_full() {
        let g1 = GenericObject {
            id: "a".into(),
            kind: "b".into(),
            attributes: serde_json::json!({}),
            relationships: Some(Relationships::new()),
            links: Some(Links::new()),
            meta: Some(Meta::new()),
        };
        let s = serde_json::to_string(&g1).unwrap();
        assert_eq!(s, "{\"id\":\"a\",\"type\":\"b\",\"attributes\":{},\"relationships\":{},\"links\":{},\"meta\":{}}");
        let g2 = serde_json::from_str(&s).unwrap();
        assert_eq!(g1, g2);
    }
}
