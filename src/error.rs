use crate::{Links, Meta};
use serde_derive::{Serialize, Deserialize};

/// Additional information about any errors encountered while processing a request 
///
/// See the [JSON:API docs](https://jsonapi.org/format/#errors) for more information
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Error {
    /// A unique identifier for this particular occurrence of the problem
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Should contain an `about` link that leads to further details about this
    /// particular occurrence of the problem
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    /// The HTTP status code applicable to this problem
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// An application-specific error code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// A short human-readable summary of the problem that SHOULD NOT change between occurrences
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// A human-readable explaination specific to this occurence of the problem
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// Information about the source of the Error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ErrorSource>,
    /// Non-standard meta information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

impl Default for Error {
    fn default() -> Self {
        Self {
            id: None,
            links: None,
            status: None,
            code: None,
            title: None,
            detail: None,
            source: None,
            meta: None,
        }
    }
}

/// Will fill the `detail` field with the `Display` of the error
impl<E> From<E> for Error 
where E : std::error::Error {
    fn from(err: E) -> Self {
        Self {
            id: None,
            links: None,
            status: None,
            code: None,
            title: None,
            detail: Some(format!("{}", err)),
            source: None,
            meta: None,
        }
    }
}

#[cfg(test)]
mod error_test {
    use super::*;
    use serde_json;

    #[test]
    fn serde_empty() {
        let e1 : Error = Default::default();
        let s = serde_json::to_string(&e1).unwrap();
        assert_eq!(s, "{}");
        let e2 = serde_json::from_str(&s).unwrap();
        assert_eq!(e1, e2);
    }

    #[test]
    fn serde_full() {
        let e1 = Error {
            id: Some("a".into()),
            links: Some(Links::new()),
            status: Some("b".into()),
            code: Some("c".into()),
            title: Some("d".into()),
            detail: Some("e".into()),
            source: Some(Default::default()),
            meta: Some(Meta::new()) 
        };
        let s = serde_json::to_string(&e1).unwrap();
        assert_eq!(s, "{\"id\":\"a\",\"links\":{},\"status\":\"b\",\"code\":\"c\",\"title\":\"d\",\"detail\":\"e\",\"source\":{},\"meta\":{}}");
        let e2 = serde_json::from_str(&s).unwrap();
        assert_eq!(e1, e2);
    }
}

/// Information about the source of the error
///
/// See the [JSON:API docs](https://jsonapi.org/format/#errors) for more information
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ErrorSource {
    /// A JSON pointer ([RFC 6901](https://tools.ietf.org/html/rfc6901)) to the associated
    /// entity in the request document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
    /// Which URI parameter caused the error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
}

impl Default for ErrorSource {
    fn default() -> Self {
        Self {
            pointer: None,
            parameter: None,
        }
    }
}

#[cfg(test)]
mod error_source_test {
    use super::*;
    use serde_json;

    #[test]
    fn serde_empty() {
        let es1 : ErrorSource = Default::default();
        let s = serde_json::to_string(&es1).unwrap();
        assert_eq!(s, "{}");
        let es2 = serde_json::from_str(&s).unwrap();
        assert_eq!(es1, es2);
    }

    #[test]
    fn serde_full() {
        let es1 = ErrorSource {
            pointer: Some("a".into()),
            parameter: Some("b".into()),
        };
        let s = serde_json::to_string(&es1).unwrap();
        assert_eq!(s, "{\"pointer\":\"a\",\"parameter\":\"b\"}");
        let es2 = serde_json::from_str(&s).unwrap();
        assert_eq!(es1, es2);
    }
}
