use crate::{Links, Meta};
use serde_derive::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Error {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<ErrorSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<Meta>,
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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ErrorSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pointer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter: Option<String>,
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
