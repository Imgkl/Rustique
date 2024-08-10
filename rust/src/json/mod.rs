use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub enum ValueTypes {
    Int(i32),
    Float(f64),
    Text(String),
    Map(HashMap<String, ValueTypes>),
    List(Vec<ValueTypes>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonStruct {
    pub field: String,
    pub value: ValueTypes,
}

pub fn json_encode(data: JsonStruct) -> String {
    serde_json::to_string(&data).unwrap()
}

pub fn json_decode(json_str: &str) -> JsonStruct {
    serde_json::from_str(json_str).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::json::{json_encode, json_decode, JsonStruct, ValueTypes};
    use std::collections::HashMap;

    #[test]
    fn test_json_encode_basic() {
        let data = JsonStruct {
            field: "example".to_string(),
            value: ValueTypes::Int(42),
        };
        let json_str = json_encode(data);
        assert_eq!(json_str, r#"{"field":"example","value":{"Int":42}}"#);
    }

    #[test]
    fn test_json_decode_basic() {
        let json_str = r#"{"field":"example","value":{"Int":42}}"#;
        let result = json_decode(json_str);
        match result.value {
            ValueTypes::Int(value) => assert_eq!(value, 42),
            _ => panic!("Unexpected type"),
        }
    }

    #[test]
    fn test_json_encode_with_map() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), ValueTypes::Int(100));
        map.insert("key2".to_string(), ValueTypes::Text("value".to_string()));

        let data = JsonStruct {
            field: "complex example".to_string(),
            value: ValueTypes::Map(map),
        };
        let json_str = json_encode(data);
        assert_eq!(
            json_str,
            r#"{"field":"complex example","value":{"Map":{"key1":{"Int":100},"key2":{"Text":"value"}}}}"#
        );
    }
}
