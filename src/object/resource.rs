use crate::{Meta, Relationships, Relationship, Links, Link, GenericObject, Identifier};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Error, Value, self};
use std::convert::{TryFrom, TryInto};
use super::ObjectConversionError;

/// A resource object
///
/// Much like the object in the 
/// [JSON:API docs](https://jsonapi.org/format/#document-resource-objects) with the
/// notable difference that the type of the object is encoded into the strictly typed
/// `Attributes` genric
#[derive(PartialEq, Debug, Clone)]
pub struct ResourceObject<A: Attributes + Serialize + DeserializeOwned> {
    /// The object identifier which together with `A::kind` must identify
    /// a unique resource
    pub id: String,
    /// Attributes representing some of the resource's data
    pub attributes: Option<A>,
    /// Describes relationships between this resource and others
    pub relationships: Option<Relationships>,
    /// Contains links relating to the resource
    pub links: Option<Links>,
    /// Contains non-standard meta information
    pub meta: Option<Meta>,
}

impl<A> ResourceObject<A> 
where A: Attributes + Serialize + DeserializeOwned {
    pub fn new(id: String, attributes: Option<A>) -> Self {
        Self {
            links: A::links(&id),
            id,
            attributes,
            relationships: None,
            meta: None,
        }
    }

    /// Inserts an entry into the relationship map, creating the map if it does not
    /// exist
    pub fn add_relationship(&mut self, name: String, relationship: Relationship) {
        if let Some(r) = &mut self.relationships {
            r.insert(name, relationship);
        } else {
            let mut r = Relationships::new();
            r.insert(name, relationship);
            self.relationships = Some(r);
        }
    }

    /// Inserts an entry into the link map, creating the map if it does not exist
    pub fn add_link(&mut self, name: String, link: Link) {
        if let Some(l) = &mut self.links {
            l.insert(name, link);
        } else {
            let mut l = Links::new();
            l.insert(name, link);
            self.links = Some(l);
        }
    }
}

/// Strictly type attributes of a `ResourceObject`
pub trait Attributes {
    /// The `type` field of the resulting JSON:API `ResourceObject`
    ///
    /// This SHOULD NOT change between calls
    fn kind() -> String;

    /// Called by `ResourceObject::new` with the intended id of the object
    /// to allow default links to be provided
    fn links(id: &str) -> Option<Links> {
        None
    }
}

impl<A> TryFrom<GenericObject> for ResourceObject<A>
where A: Attributes + Serialize + DeserializeOwned {
    type Error = ObjectConversionError;
    fn try_from(go: GenericObject) -> Result<Self, Self::Error> {
        if go.kind != A::kind() {
            return Err(ObjectConversionError::ImproperType{expected: A::kind(), got: go.kind});
        }
        let attributes = if let Some(m) = go.attributes {
            let v = Value::Object(m.into_iter().collect());
            let a : A = serde_json::from_value(v)?;
            Some(a)
        } else { None };

        Ok(Self {
            id: go.id,
            attributes,
            relationships: go.relationships,
            links: go.links,
            meta: go.meta,
        })
    }
}

impl<A> TryFrom<&GenericObject> for ResourceObject<A> 
where A: Attributes + Serialize + DeserializeOwned {
    type Error = ObjectConversionError;
    fn try_from(go: &GenericObject) -> Result<Self, Self::Error> {
        if go.kind != A::kind() {
            return Err(ObjectConversionError::ImproperType{expected: A::kind(), got: go.kind.clone()});
        }
        let attributes = if let Some(m) = &go.attributes {
            let v = Value::Object(m.iter().map(|(k,v)| (k.clone(), v.clone())).collect());
            let a : A = serde_json::from_value(v)?;
            Some(a)
        } else { None };

        Ok( Self{
            id: go.id.clone(),
            attributes,
            relationships: go.relationships.clone(),
            links: go.links.clone(),
            meta: go.meta.clone(),
        })
    }
}

impl<A> TryFrom<Identifier> for ResourceObject<A>
where A: Attributes + Serialize + DeserializeOwned {
    type Error = ObjectConversionError;
    fn try_from(id: Identifier) -> Result<Self, Self::Error> {
        if id.kind != A::kind() {
            return Err(ObjectConversionError::ImproperType{expected: A::kind(), got: id.kind});
        }
        Ok(Self {
            id: id.id,
            attributes: None,
            relationships: None,
            links: None,
            meta: id.meta,
        })
    }
}

impl<A> TryFrom<&Identifier> for ResourceObject<A>
where A: Attributes + Serialize + DeserializeOwned {
    type Error = ObjectConversionError;
    fn try_from(id: &Identifier) -> Result<Self, Self::Error> {
        if id.kind != A::kind() {
            return Err(ObjectConversionError::ImproperType{expected: A::kind(), got: id.kind.clone()});
        }
        Ok(Self {
            id: id.id.clone(),
            attributes: None,
            relationships: None,
            links: None,
            meta: id.meta.clone(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_derive::{Serialize, Deserialize};
    use std::collections::BTreeMap;

    #[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
    struct Attr {
        kitty: bool,
    }

    impl Attributes for Attr { 
        fn kind() -> String { "b".into() }
    }

    #[test]
    fn from_go_success() {
        let mut attr = BTreeMap::new();
        attr.insert("kitty".into(), Value::Bool(true));
        let go = GenericObject {
            id: "a".into(),
            kind: "b".into(),
            attributes: Some(attr),
            relationships: Some(Relationships::new()),
            links: Some(Links::new()),
            meta: Some(Meta::new()),
        };

        let ro = (&go).try_into().unwrap();
        assert_eq!(ResourceObject {
            id: "a".into(),
            attributes: Some(Attr{ kitty: true }),
            relationships: Some(Relationships::new()),
            links: Some(Links::new()),
            meta: Some(Meta::new()),
        }, ro);

        let ro = go.try_into().unwrap();
        assert_eq!(ResourceObject {
            id: "a".into(),
            attributes: Some(Attr{ kitty: true }),
            relationships: Some(Relationships::new()),
            links: Some(Links::new()),
            meta: Some(Meta::new()),
        }, ro);
    }

    #[test]
    fn from_go_fail_type() {
        let mut attr = BTreeMap::new();
        attr.insert("kitty".into(), Value::Bool(true));
        let go = GenericObject {
            id: "a".into(),
            kind: "c".into(),
            attributes: Some(attr),
            relationships: Some(Relationships::new()),
            links: Some(Links::new()),
            meta: Some(Meta::new()),
        };

        assert!(ResourceObject::<Attr>::try_from(&go).is_err());
        assert!(ResourceObject::<Attr>::try_from(go).is_err());
    }

    #[test]
    fn from_go_fail_attributes() {
        let go = GenericObject {
            id: "a".into(),
            kind: "b".into(),
            attributes: Some(BTreeMap::new()),
            relationships: None,
            links: None,
            meta: None,
        };

        assert!(ResourceObject::<Attr>::try_from(&go).is_err());
        assert!(ResourceObject::<Attr>::try_from(go).is_err());
    }

    #[test]
    fn from_id() {
        let id = Identifier {
            id: "a".into(),
            kind: "b".into(),
            meta: Some(Meta::new()),
        };

        let ro = ResourceObject::<Attr>::try_from(&id).unwrap();
        assert_eq!(ResourceObject {
            id: "a".into(),
            attributes: None,
            relationships: None,
            links: None,
            meta: Some(Meta::new()),
        }, ro);

        let ro = ResourceObject::<Attr>::try_from(id).unwrap();
        assert_eq!(ResourceObject {
            id: "a".into(),
            attributes: None,
            relationships: None,
            links: None,
            meta: Some(Meta::new()),
        }, ro);
    }

    #[test]
    fn from_id_fail() {
        let id = Identifier {
            id: "a".into(),
            kind: "c".into(),
            meta: None,
        };

        assert!(ResourceObject::<Attr>::try_from(&id).is_err());
        assert!(ResourceObject::<Attr>::try_from(id).is_err());
    }
}
