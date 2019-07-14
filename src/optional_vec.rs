use serde_derive::{Serialize, Deserialize};
use serde::de::{DeserializeOwned, Deserializer, Deserialize, Error};
use serde_json::{Value, self};

#[derive(Serialize, PartialEq, Eq, Debug, Clone)]
#[serde(untagged)]
pub enum OptionalVec<T> {
    NotPresent,
    One(Option<T>),
    Many(Vec<T>),
}

impl<T> OptionalVec<T> {
    pub fn is_not_present(&self) -> bool {
        match self {
            OptionalVec::NotPresent => true,
            _ => false,
        }
    }

    pub fn is_one(&self) -> bool {
        match self {
            OptionalVec::One(_) => true,
            _ => false,
        }
    }

    pub fn is_many(&self) -> bool {
        match self {
            OptionalVec::Many(_) => true,
            _ => false,
        }
    }
}

impl<'de, T> Deserialize<'de> for OptionalVec<T> 
where T: DeserializeOwned {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> 
    where D: Deserializer<'de> {
        // okay so this should probably use a visitor but that's like
        //
        // effort
        //
        // ya know?
        let v = Value::deserialize(deserializer)?;
        match serde_json::from_value::<Option<T>>(v.clone()) {
            Ok(one) => Ok(OptionalVec::One(one)),
            Err(_) => match serde_json::from_value(v) {
                Ok(many) => Ok(OptionalVec::Many(many)),
                Err(_) => Err(D::Error::custom("Neither one nor many")),
            },
        }
    }
}

impl<T> Default for OptionalVec<T> {
    fn default() -> Self {
        OptionalVec::NotPresent
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
    struct TestStruct {
        #[serde(skip_serializing_if = "OptionalVec::is_not_present", default)]
        inner: OptionalVec<usize>,
    }

    #[test]
    fn serde_not_present() {
        let ts1 = TestStruct{ inner: OptionalVec::NotPresent };
        let s = serde_json::to_string(&ts1).unwrap();
        assert_eq!(s, "{}");
        let ts2 = serde_json::from_str(&s).unwrap();
        assert_eq!(ts1, ts2);
    }

    #[test]
    fn serde_empty_to_one() {
        let ts1 = TestStruct{ inner: OptionalVec::One(None) };
        let s = serde_json::to_string(&ts1).unwrap();
        assert_eq!(s, "{\"inner\":null}");
        let ts2 = serde_json::from_str(&s).unwrap();
        assert_eq!(ts1, ts2);
    }

    #[test]
    fn serde_to_one() {
        let ts1 = TestStruct{ inner: OptionalVec::One(Some(1)) };
        let s = serde_json::to_string(&ts1).unwrap();
        assert_eq!(s, "{\"inner\":1}");
        let ts2 = serde_json::from_str(&s).unwrap();
        assert_eq!(ts1, ts2);
    }

    #[test]
    fn serde_to_many() {
        let ts1 = TestStruct{ inner: OptionalVec::Many(vec![1,2]) };
        let s = serde_json::to_string(&ts1).unwrap();
        assert_eq!(s, "{\"inner\":[1,2]}");
        let ts2 = serde_json::from_str(&s).unwrap();
        assert_eq!(ts1, ts2);
    }

    #[test]
    #[should_panic]
    fn failed_deserialize() {
        let _ : TestStruct = serde_json::from_str("{\"inner\":\"oops i'm a string\"}").unwrap();
    }

    #[test]
    fn is() {
        let ov : OptionalVec<usize> = OptionalVec::NotPresent;
        assert!(ov.is_not_present());
        assert!(!ov.is_one());
        assert!(!ov.is_many());

        let ov : OptionalVec<usize> = OptionalVec::One(None);
        assert!(!ov.is_not_present());
        assert!(ov.is_one());
        assert!(!ov.is_many());

        let ov : OptionalVec<usize> = OptionalVec::Many(vec![]);
        assert!(!ov.is_not_present());
        assert!(!ov.is_one());
        assert!(ov.is_many());
    }
}
