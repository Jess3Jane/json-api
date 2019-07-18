use serde_json::{
    Error as SerdeError,
    self,
    Value,
};
use std::{
    error::{Error as StdError},
    fmt::{Display, Formatter, Error as FormatError},
};

/// The error produced when something has gone wrong converting into `ResourceObject`'s
///
/// `ObjectConversionError::FailedDeserialization` is produced when the `attributes` field
/// fails to deserialize
///
/// `ObjectConversionError::ImproperType` is produced when the type of the object does not
/// match the output of the attribute object's `kind` function
#[derive(Debug)]
pub enum ObjectConversionError {
    FailedDeserialization(SerdeError),
    ImproperType{expected: String, got: String},
}

impl From<SerdeError> for ObjectConversionError {
    fn from(err: SerdeError) -> Self {
        ObjectConversionError::FailedDeserialization(err)
    }
}

impl Display for ObjectConversionError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        write!(f, "Object Conversion Error: ")?;
        match self {
            ObjectConversionError::FailedDeserialization(e) 
                => write!(f, "Failed to Deserialize Attributes ({})", e),
            ObjectConversionError::ImproperType{expected, got} 
                => write!(f, "Improper type (expected {}, got {})", expected, got),
        }
    }
}

impl StdError for ObjectConversionError {
    fn cause(&self) -> Option<&dyn StdError> {
        match self {
            ObjectConversionError::FailedDeserialization(e) => Some(e),
            ObjectConversionError::ImproperType{expected: _, got: _} => None,
        }
    }
}
