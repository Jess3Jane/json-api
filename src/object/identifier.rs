use crate::{Meta, Relationships, Links, GenericObject, ResourceObject, Attributes};
use serde::{Serialize, de::DeserializeOwned};
use serde_derive::{Serialize, Deserialize};
use super::ObjectConversionError;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Identifier {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

impl Identifier {
    pub fn new(id: String, kind: String) -> Self {
        Self {
            id,
            kind,
            meta: None,
        }
    }
}

impl From<GenericObject> for Identifier {
    fn from(go: GenericObject) -> Self {
        Self {
            id: go.id,
            kind: go.kind,
            meta: go.meta,
        }
    }
}

impl From<&GenericObject> for Identifier {
    fn from(go: &GenericObject) -> Self {
        Self {
            id: go.id.clone(),
            kind: go.kind.clone(),
            meta: go.meta.clone(),
        }
    }
}

impl<A> From<ResourceObject<A>> for Identifier 
where A: Attributes + Serialize + DeserializeOwned {
    fn from(ro: ResourceObject<A>) -> Self {
        Self {
            id: ro.id,
            kind: A::kind(),
            meta: ro.meta,
        }
    }
}

impl<A> From<&ResourceObject<A>> for Identifier
where A: Attributes + Serialize + DeserializeOwned {
    fn from(ro: &ResourceObject<A>) -> Self {
        Self {
            id: ro.id.clone(),
            kind: A::kind(),
            meta: ro.meta.clone(),
        }
    }
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

    #[test]
    fn from_go() {
        let mut meta = Meta::new();
        meta.insert("c".into(), serde_json::json!("d"));
        let go = GenericObject {
            id: "a".into(),
            kind: "b".into(),
            attributes: None,
            relationships: None,
            links: None,
            meta: Some(meta.clone()),
        };

        let id = (&go).into();
        assert_eq!(Identifier {
            id: "a".into(),
            kind: "b".into(),
            meta: Some(meta.clone()),
        }, id);

        let id = go.into();
        assert_eq!(Identifier {
            id: "a".into(),
            kind: "b".into(),
            meta: Some(meta.clone()),
        }, id);
    }

    #[test] 
    fn from_ro() {
        #[derive(Serialize, Deserialize, Eq, PartialEq, Clone)]
        struct Attr;
        impl Attributes for Attr{
            fn kind() -> String { "b".into() }
        }

        let mut meta = Meta::new();
        meta.insert("c".into(), serde_json::json!("d"));
        let ro : ResourceObject<Attr> = ResourceObject {
            id: "a".into(),
            attributes: None,
            relationships: None,
            links: None,
            meta: Some(meta.clone()),
        };

        let id = (&ro).into();
        assert_eq!(Identifier {
            id: "a".into(),
            kind: "b".into(),
            meta: Some(meta.clone()),
        }, id);

        let id = ro.into();
        assert_eq!(Identifier {
            id: "a".into(),
            kind: "b".into(),
            meta: Some(meta.clone()),
        }, id);
    }
}
