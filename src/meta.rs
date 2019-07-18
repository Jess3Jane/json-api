use serde_json::Value;
use std::collections::BTreeMap;

/// An object containing non-standard meta information
///
/// See the [JSON:API docs](https://jsonapi.org/format/#document-meta) for more information
pub type Meta = BTreeMap<String, Value>;

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::{self, json};

    #[test]
    fn serde() {
        let mut meta = Meta::new();
        meta.insert("a".into(), json!("a"));
        meta.insert("b".into(), Value::Null);
        let s = serde_json::to_string(&meta).unwrap();
        assert_eq!(s, "{\"a\":\"a\",\"b\":null}");
        let meta2 = serde_json::from_str(&s).unwrap();
        assert_eq!(meta, meta2);
    }
}
