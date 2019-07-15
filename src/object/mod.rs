mod generic;
pub use generic::GenericObject;

mod resource;
pub use resource::{ResourceObject, Attributes};

mod identifier;
pub use identifier::Identifier;

mod conversion_error;
pub use conversion_error::ObjectConversionError;
