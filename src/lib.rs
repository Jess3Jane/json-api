mod optional_vec;
pub use crate::optional_vec::OptionalVec;

mod object;
pub use crate::object::{GenericObject, ResourceObject, Identifier, Attributes};

mod meta;
pub use crate::meta::Meta;

mod links;
pub use crate::links::{Links, Link};

mod relationships;
pub use crate::relationships::{Relationship, Relationships};
