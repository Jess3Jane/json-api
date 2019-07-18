use crate::{Meta, Relationships, Links, ResourceObject, Attributes, Identifier};
use serde::{Serialize, de::DeserializeOwned};
use serde_derive::{Serialize, Deserialize}; 
use serde_json::{self, Value};
use std::collections::BTreeMap;

/// A generic resource object of some unknown type
///
/// Should never be directly manipulated, convert to/from `ResourceObject` or 
/// `Identifier` instead
///
/// See the [JSON:API docs](https://jsonapi.org/format/#document-resource-objects)
/// for more information
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct GenericObject {
    pub (crate) id: String,
   #[serde(rename = "type")]
    pub (crate) kind: String,
    // This should maybe be a serde_json::Map to make converting faster
    // Alternately convert with a custom deserializer but also no,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub (crate) attributes: Option<BTreeMap<String, Value>>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub (crate) relationships: Option<Relationships>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub (crate) links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub (crate) meta: Option<Meta>,
}

impl GenericObject {
    /// Returns the type of the object (renamed here to `kind` due to keyword restrictuons)
    pub fn kind<'a>(&'a self) -> &'a str {
        &self.kind
    }
}

impl<A> From<ResourceObject<A>> for GenericObject
where A: Attributes + Serialize + DeserializeOwned {
    fn from(ro: ResourceObject<A>) -> Self {
        let v = serde_json::to_value(&ro.attributes).unwrap();
        Self {
            id: ro.id,
            kind: A::kind(),
            attributes: serde_json::from_value(v).unwrap(),
            relationships: ro.relationships,
            links: ro.links,
            meta: ro.meta,
        }
    }
}

impl<A> From<&ResourceObject<A>> for GenericObject 
where A: Attributes + Serialize + DeserializeOwned {
    fn from(ro: &ResourceObject<A>) -> Self {
        let v = serde_json::to_value(&ro.attributes).unwrap();
        Self {
            id: ro.id.clone(),
            kind: A::kind(),
            attributes: serde_json::from_value(v).unwrap(),
            relationships: ro.relationships.clone(),
            links: ro.links.clone(),
            meta: ro.meta.clone(),
        }
    }
}

impl From<Identifier> for GenericObject {
    fn from(id: Identifier) -> Self {
        Self {
            id: id.id,
            kind: id.kind,
            attributes: None,
            relationships: None,
            links: None,
            meta: id.meta,
        }
    }
}

impl From<&Identifier> for GenericObject {
    fn from(id: &Identifier) -> Self {
        Self {
            id: id.id.clone(),
            kind: id.kind.clone(),
            attributes: None,
            relationships: None,
            links: None,
            meta: id.meta.clone(),
        }
    }
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
            attributes: None,
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
            attributes: Some(BTreeMap::new()),
            relationships: Some(Relationships::new()),
            links: Some(Links::new()),
            meta: Some(Meta::new()),
        };
        let s = serde_json::to_string(&g1).unwrap();
        assert_eq!(s, "{\"id\":\"a\",\"type\":\"b\",\"attributes\":{},\"relationships\":{},\"links\":{},\"meta\":{}}");
        let g2 = serde_json::from_str(&s).unwrap();
        assert_eq!(g1, g2);
    }

    #[test]
    fn from_ro() {
        #[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
        struct Attr {
            kitty: bool,
        };
        impl Attributes for Attr {
            fn kind() -> String { "b".into() }
        }

        let mut ro = ResourceObject::<Attr>::new("a".into(), None);
        let go = (&ro).into();
        assert_eq!(GenericObject {
            id: "a".into(),
            kind: "b".into(),
            attributes: None, 
            relationships: None,
            links: None,
            meta: None,
        }, go);

        let go = ro.clone().into();
        assert_eq!(GenericObject {
            id: "a".into(),
            kind: "b".into(),
            attributes: None, 
            relationships: None,
            links: None,
            meta: None,
        }, go);

        ro.attributes = Some(Attr{ kitty: true });
        ro.relationships = Some(Relationships::new());
        ro.links = Some(Links::new());
        ro.meta = Some(Meta::new());

        let mut attr = BTreeMap::new();
        attr.insert("kitty".into(), Value::Bool(true));

        let go = (&ro).into();
        assert_eq!(GenericObject {
            id: "a".into(),
            kind: "b".into(),
            attributes: Some(attr.clone()), 
            relationships: Some(Relationships::new()),
            links: Some(Links::new()),
            meta: Some(Meta::new()),
        }, go);

        let go = ro.into();
        assert_eq!(GenericObject {
            id: "a".into(),
            kind: "b".into(),
            attributes: Some(attr.clone()), 
            relationships: Some(Relationships::new()),
            links: Some(Links::new()),
            meta: Some(Meta::new()),
        }, go);
    }

    #[test]
    fn from_id() {
        let id = Identifier {
            id: "a".into(),
            kind: "b".into(),
            meta: Some(Meta::new()),
        };

        let go = (&id).into();
        assert_eq!(GenericObject {
            id: "a".into(),
            kind: "b".into(),
            attributes: None,
            relationships: None, 
            links: None,
            meta: Some(Meta::new()),
        }, go);

        let go = id.into();
        assert_eq!(GenericObject {
            id: "a".into(),
            kind: "b".into(),
            attributes: None,
            relationships: None, 
            links: None,
            meta: Some(Meta::new()),
        }, go);
    }
}
