use std::collections::BTreeMap;
use std::ops::{Index, IndexMut};
use std::path::PathBuf;

use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug)]
pub struct Repo {
    path: PathBuf,
}

impl Repo {
    #[must_use]
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    #[must_use]
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Number {
    Integer(i64),
    Float(f64),
}

impl Number {
    #[must_use]
    pub fn new_integer(value: i64) -> Self {
        Self::Integer(value)
    }

    #[must_use]
    pub fn new_float(value: f64) -> Self {
        Self::Float(value)
    }

    #[must_use]
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Self::Integer(value) => Some(*value),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Self::Float(value) => Some(*value),
            _ => None,
        }
    }
}

impl From<Number> for tera::Number {
    fn from(value: Number) -> Self {
        match value {
            Number::Integer(value) => Self::from(value),
            Number::Float(value) => {
                Self::from_f64(value).expect("could not convert float to number")
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
}

impl Value {
    #[must_use]
    pub fn new_null() -> Self {
        Self::Null
    }

    #[must_use]
    pub fn new_bool(value: bool) -> Self {
        Self::Bool(value)
    }

    #[must_use]
    pub fn new_number(value: Number) -> Self {
        Self::Number(value)
    }

    #[must_use]
    pub fn new_string<T: Into<String>>(value: T) -> Self {
        Self::String(value.into())
    }

    #[must_use]
    pub fn new_array<T: Into<Vec<Value>>>(value: T) -> Self {
        Self::Array(value.into())
    }

    #[must_use]
    pub fn new_object<T: Into<BTreeMap<String, Value>>>(value: T) -> Self {
        Self::Object(value.into())
    }

    #[must_use]
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_number(&self) -> Option<Number> {
        match self {
            Self::Number(value) => Some(*value),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Self::Array(value) => Some(value),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_object(&self) -> Option<&BTreeMap<String, Value>> {
        match self {
            Self::Object(value) => Some(value),
            _ => None,
        }
    }

    pub fn insert<T: Into<String>, U: Into<Value>>(&mut self, key: T, value: U) {
        match self {
            Self::Object(map) => {
                map.insert(key.into(), value.into());
            }
            _ => panic!("not an object"),
        }
    }

    pub fn union(&mut self, other: &Self) {
        match (self, other) {
            (Self::Object(a), Self::Object(b)) => {
                for (key, value) in b {
                    a.insert(key.clone(), value.clone());
                }
            }
            (Self::Array(a), Self::Array(b)) => {
                for value in b {
                    a.push(value.clone());
                }
            }
            (a, b) => panic!("incompatible types: {:?} and {:?}", a, b),
        }
    }

    pub fn override_with(&mut self, other: &Self) {
        match (self, other) {
            (Self::Object(self_map), Self::Object(other_map)) => {
                for (key, value) in other_map {
                    if let Some(self_value) = self_map.get_mut(key) {
                        self_value.override_with(value);
                    } else {
                        self_map.insert(key.clone(), value.clone());
                    }
                }
            }
            (a, b) => *a = b.clone(),
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::new_null()
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::new_bool(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::new_number(Number::new_integer(value as i64))
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::new_number(Number::new_integer(value))
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::new_number(Number::new_float(value))
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::new_string(value)
    }
}

impl From<&String> for Value {
    fn from(value: &String) -> Self {
        Self::new_string(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::new_string(value)
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Self::new_array(value)
    }
}

impl From<BTreeMap<String, Value>> for Value {
    fn from(value: BTreeMap<String, Value>) -> Self {
        Self::new_object(value)
    }
}

impl Index<&str> for Value {
    type Output = Value;

    fn index(&self, index: &str) -> &Self::Output {
        match self {
            Self::Object(value) => value.get(index).expect("key not found"),
            _ => panic!("not an object"),
        }
    }
}

impl IndexMut<&str> for Value {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        match self {
            Self::Object(value) => value.get_mut(index).expect("key not found"),
            _ => panic!("not an object"),
        }
    }
}

impl From<Value> for tera::Value {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => Self::Null,
            Value::Bool(value) => Self::Bool(value),
            Value::Number(value) => Self::Number(value.into()),
            Value::String(value) => Self::String(value),
            Value::Array(value) => Self::Array(value.into_iter().map(Self::from).collect()),
            Value::Object(value) => {
                Self::Object(value.into_iter().map(|(k, v)| (k, Self::from(v))).collect())
            }
        }
    }
}

impl From<Value> for tera::Context {
    fn from(value: Value) -> Self {
        Self::from_value(value.into()).expect("could not convert value to context")
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Null => serializer.serialize_none(),
            Self::Bool(value) => serializer.serialize_bool(*value),
            Self::Number(value) => match value {
                Number::Integer(value) => serializer.serialize_i64(*value),
                Number::Float(value) => serializer.serialize_f64(*value),
            },
            Self::String(value) => serializer.serialize_str(value),
            Self::Array(value) => serializer.collect_seq(value),
            Self::Object(value) => serializer.collect_map(value),
        }
    }
}

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a value")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::new_bool(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::new_number(Number::new_integer(value)))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::new_number(Number::new_integer(value as i64)))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::new_number(Number::new_float(value)))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::new_string(value))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::new_string(value))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::new_null())
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::new_null())
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut array = Vec::new();
        while let Some(value) = seq.next_element()? {
            array.push(value);
        }
        Ok(Value::new_array(array))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut object = BTreeMap::new();
        while let Some((key, value)) = map.next_entry()? {
            object.insert(key, value);
        }
        Ok(Value::new_object(object))
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::data::Number;

    #[test]
    fn test_serialize_value_yaml() {
        use super::Value;

        let value = Value::new_object({
            let mut map = std::collections::BTreeMap::new();
            map.insert("a".to_string(), Value::new_number(Number::new_integer(1)));
            map.insert("b".to_string(), Value::new_number(Number::new_float(2.3)));
            map.insert("c".to_string(), Value::new_bool(true));
            map.insert("d".to_string(), Value::new_string("hello".to_string()));
            map.insert(
                "e".to_string(),
                Value::new_array(vec![
                    Value::new_number(Number::new_integer(1)),
                    Value::new_number(Number::new_float(2.3)),
                    Value::new_bool(true),
                    Value::new_string("hello".to_string()),
                ]),
            );
            map.insert(
                "f".to_string(),
                Value::new_object({
                    let mut map = std::collections::BTreeMap::new();
                    map.insert("a".to_string(), Value::new_number(Number::new_integer(1)));
                    map.insert("b".to_string(), Value::new_number(Number::new_float(2.3)));
                    map.insert("c".to_string(), Value::new_bool(true));
                    map.insert("d".to_string(), Value::new_string("hello".to_string()));
                    map
                }),
            );
            map
        });

        let yaml = serde_yaml::to_string(&value).unwrap();

        assert_eq!(
            yaml,
            r#"a: 1
b: 2.3
c: true
d: hello
e:
- 1
- 2.3
- true
- hello
f:
  a: 1
  b: 2.3
  c: true
  d: hello
"#
        );

        let value_deserialized: Value = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(value, value_deserialized);
    }
}
