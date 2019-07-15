use crate::{OptionalVec, Meta, GenericObject, JsonApi, Links, Error};
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Document {
    #[serde(skip_serializing_if = "OptionalVec::is_not_present", default)]
    data: OptionalVec<GenericObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<Meta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jsonapi: Option<JsonApi>,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    included: Option<Vec<GenericObject>>,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            data: OptionalVec::NotPresent,
            errors: None,
            meta: None,
            jsonapi: None,
            links: None,
            included: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;

    #[test]
    fn serde_none() {
        let d1 : Document = Default::default();
        let s = serde_json::to_string(&d1).unwrap();
        assert_eq!(s, "{}");
        let d2 = serde_json::from_str(&s).unwrap();
        assert_eq!(d1, d2);
    }

    #[test]
    fn serde_full() {
        let d1 = Document {
            data: OptionalVec::One(None),
            errors: Some(Vec::new()),
            meta: Some(Meta::new()),
            jsonapi: Some(Default::default()),
            links: Some(Links::new()),
            included: Some(Vec::new()),
        };
        let s = serde_json::to_string(&d1).unwrap();
        assert_eq!(s, "{\"data\":null,\"errors\":[],\"meta\":{},\"jsonapi\":{},\"links\":{},\"included\":[]}");
        let d2 = serde_json::from_str(&s).unwrap();
        assert_eq!(d1, d2);
    }
}


