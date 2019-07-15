use crate::Meta;
use serde_derive::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::BTreeMap;

pub type Links = BTreeMap<String, Link>;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum Link {
    Url(String),
    Object { 
        #[serde(skip_serializing_if = "Option::is_none")]
        href: Option<String>, 
        #[serde(skip_serializing_if = "Option::is_none")]
        meta: Option<Meta>, 
    },
}

impl Link {
    pub fn href<'a>(&'a self) -> Option<&'a str> {
        match &self {
            Link::Url(href) => Some(href),
            Link::Object{ href: Some(href), meta: _ } => Some(&href),
            _ => None,
        }
    }
}

impl Default for Link {
    fn default() -> Self {
        Link::Object { 
            href: None,
            meta: None,
        }
    }
}

#[cfg(test)]
mod link_test {
    use super::*;
    use serde_json;

    #[test]
    fn serde_url() {
        let l1 = Link::Url("a".into());
        let s = serde_json::to_string(&l1).unwrap();
        assert_eq!(s, "\"a\"");
        let l2 = serde_json::from_str(&s).unwrap();
        assert_eq!(l1, l2);
    }

    #[test]
    fn serde_object() {
        let l1 = Link::Object{
            href: Some("a".into()),
            meta: Some(BTreeMap::new()),
        };
        let s = serde_json::to_string(&l1).unwrap();
        assert_eq!(s, "{\"href\":\"a\",\"meta\":{}}");
        let l2 = serde_json::from_str(&s).unwrap();
        assert_eq!(l1, l2);
    }
}

#[cfg(test)]
mod links_test {
    use super::*;
    use serde_json;

    #[test]
    fn serde() {
        let mut links = Links::new();
        links.insert("a".into(), Link::Url("b".into()));
        links.insert("b".into(), Link::Object{ href: Some("c".into()), meta: None });
        links.insert("c".into(), Link::Object{ href: None, meta: None });

        let s = serde_json::to_string(&links).unwrap();
        assert_eq!(s, "{\"a\":\"b\",\"b\":{\"href\":\"c\"},\"c\":{}}");
        let links2 = serde_json::from_str(&s).unwrap();
        assert_eq!(links, links2);
    }
}
