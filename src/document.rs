use crate::{OptionalVec, Meta, GenericObject, JsonApi, Links, Error};
use serde_derive::{Serialize, Deserialize};

/// The object at the root of every JSON:API message
///
/// For validitity it must contain at least one of `data`, `errors`, or `meta`
///
/// See the [JSON:API docs](https://jsonapi.org/format/#document-top-level) for more information
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Document {
    /// The document's primary data
    #[serde(skip_serializing_if = "OptionalVec::is_not_present", default)]
    pub data: OptionalVec<GenericObject>,
    /// Any errors that were encountered
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    /// Non-standard meta information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    /// Information about the version of JSON:API being used 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsonapi: Option<JsonApi>,
    /// Links related to the primary data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    /// Included resources related to the primary data or each other
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included: Option<Vec<GenericObject>>,
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
    fn serde_empty() {
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


