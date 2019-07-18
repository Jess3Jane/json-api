use crate::Meta;
use serde_derive::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::BTreeMap;

/// A collection of links
///
/// See the [JSON:API docs](https://jsonapi.org/format/#document-links) for more information
pub type Links = BTreeMap<String, Link>;

/// A link object
/// 
/// Enum represents every valid configuration but in practice you should probably either use
/// `Links::Url` for links with just a URL or `Links::Object` if you need to include a meta
/// object
///
/// See the [JSON:API docs](https://jsonapi.org/format/#document-links) for more information
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
    /// Grabs the url of the link regardless of its variant
    pub fn href<'a>(&'a self) -> Option<&'a str> {
        match &self {
            Link::Url(href) => Some(href),
            Link::Object{ href: Some(href), meta: _ } => Some(&href),
            _ => None,
        }
    }

    /// If meta is `None` will construct `Link::Url` instead of `Link::Object`
    pub fn new(url: String, meta: Option<Meta>) -> Self {
        if let Some(m) = meta {
            Link::Object{ href: Some(url), meta: Some(m) }
        } else {
            Link::Url(url)
        }
    }
}

/// Defaults to `Link::Object` with all fields `None`
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
