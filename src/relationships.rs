use serde_derive::{Serialize, Deserialize};
use crate::{Links, Identifier, OptionalVec, Meta};
use std::collections::BTreeMap;

pub type Relationships = BTreeMap<String, Relationship>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Relationship {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    #[serde(skip_serializing_if = "OptionalVec::is_not_present", default)]
    pub data: OptionalVec<Identifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

#[cfg(test)]
mod relationship_test {
    use super::*;
    use serde_json;

    #[test]
    fn serde_empty() {
        let r1 = Relationship{
            links: None,
            data: OptionalVec::NotPresent,
            meta: None,
        };
        let s = serde_json::to_string(&r1).unwrap();
        assert_eq!(s, "{}");
        let r2 = serde_json::from_str(&s).unwrap();
        assert_eq!(r1, r2);
    }

    #[test]
    fn serde_full() {
        let r1 = Relationship{
            links: Some(Links::new()),
            data: OptionalVec::Many(vec![]),
            meta: Some(Meta::new()),
        };
        let s = serde_json::to_string(&r1).unwrap();
        assert_eq!(s, "{\"links\":{},\"data\":[],\"meta\":{}}");
        let r2 = serde_json::from_str(&s).unwrap();
        assert_eq!(r1, r2);
    }
}

#[cfg(test)]
mod relationships_test {
    use super::*;
    use serde_json;

    #[test]
    fn serde() {
        let mut rs1 = Relationships::new();
        let r1 = Relationship{
            links: None,
            data: OptionalVec::NotPresent,
            meta: None,
        };
        rs1.insert("a".into(), r1);
        let r2 = Relationship{
            links: Some(Links::new()),
            data: OptionalVec::Many(vec![]),
            meta: Some(Meta::new()),
        };
        rs1.insert("b".into(), r2);
        let s = serde_json::to_string(&rs1).unwrap();
        assert_eq!(s, "{\"a\":{},\"b\":{\"links\":{},\"data\":[],\"meta\":{}}}");
        let rs2 = serde_json::from_str(&s).unwrap();
        assert_eq!(rs1, rs2);
    }
}
