use crate::{Meta, Relationships, Links};
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Identifier {
    id: String,
    #[serde(rename = "type")]
    kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<Meta>,
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;

    #[test]
    fn serde() {
        let id1 = Identifier{
            id: "a".into(),
            kind: "b".into(),
            meta: None,
        };
        let s = serde_json::to_string(&id1).unwrap();
        assert_eq!(s, "{\"id\":\"a\",\"type\":\"b\"}");
        let id2 = serde_json::from_str(&s).unwrap();
        assert_eq!(id1, id2);
    }
}
