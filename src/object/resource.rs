use crate::{Meta, Relationships, Links};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

#[derive(PartialEq, Debug, Clone)]
/// Much like a GenericObject but with two key differences:
/// * Has stricty typed attributes instead of `Value`
/// * Does not have a `kind` attributes, that's handled by the attribute
pub struct ResourceObject<A: Attributes + Serialize + DeserializeOwned> {
    pub id: String,
    pub attributes: Option<A>,
    pub relationships: Option<Relationships>,
    pub links: Option<Links>,
    pub meta: Option<Meta>,
}

impl<A> ResourceObject<A> 
where A: Attributes + Serialize + DeserializeOwned {
    fn new(id: String) -> Self {
        Self {
            id,
            attributes: None,
            relationships: None,
            links: None,
            meta: None,
        }
    }
}

/// Strongly type attributes of a `ResourceObject`
pub trait Attributes {
    /// The `type` field of the resulting JSON:API `ResourceObject`
    fn kind() -> String;
}
