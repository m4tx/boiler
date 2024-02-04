use std::collections::BTreeMap;
use std::ops::{Index, IndexMut};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug)]
pub struct Repo {
    path: PathBuf,
}

impl Repo {
    #[must_use]
    pub fn new<T: Into<PathBuf>>(path: T) -> Self {
        Self { path: path.into() }
    }

    #[must_use]
    pub fn path(&self) -> &Path {
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

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Self::new_integer(value)
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Self::new_float(value)
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
    pub fn new_number<T: Into<Number>>(value: T) -> Self {
        Self::Number(value.into())
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
    pub fn empty_object() -> Self {
        Self::Object(BTreeMap::new())
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

    pub fn union(&mut self, other: &Self) -> anyhow::Result<()> {
        match (self, other) {
            (Self::Object(a), Self::Object(b)) => {
                for (key, value) in b {
                    if let Some(self_value) = a.get_mut(key) {
                        self_value
                            .union(value)
                            .with_context(|| format!("at key {:?}", key))?;
                    } else {
                        a.insert(key.clone(), value.clone());
                    }
                }
            }
            (Self::Array(a), Self::Array(b)) => {
                for value in b {
                    a.push(value.clone());
                }
            }
            (Self::String(a), Self::String(b)) => {
                if a != b {
                    bail!("incompatible string values: {:?} and {:?}", a, b)
                }
            }
            (Self::Number(a), Self::Number(b)) => {
                if a != b {
                    bail!("incompatible number values: {:?} and {:?}", a, b)
                }
            }
            (Self::Bool(a), Self::Bool(b)) => {
                if a != b {
                    bail!("incompatible bool values: {:?} and {:?}", a, b)
                }
            }
            (Self::Null, Self::Null) => {}
            (a, b) => bail!("incompatible types: {:?} and {:?}", a, b),
        }

        Ok(())
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

    pub fn as_yaml(&self) -> String {
        serde_yaml::to_string(self).expect("could not serialize value to yaml")
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

impl<const N: usize> From<[Value; N]> for Value {
    fn from(value: [Value; N]) -> Self {
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
    use crate::data::{Number, Value};

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

    #[test]
    fn test_value_null_union() {
        let mut val = Value::new_null();
        val.union(&Value::new_null()).unwrap();
        assert_eq!(val, Value::new_null());
    }

    #[test]
    fn test_value_number_union() {
        let mut val = Value::new_number(13);
        val.union(&Value::new_number(13)).unwrap();
        assert_eq!(val, Value::new_number(13));

        let mut val = Value::new_number(13);
        assert!(val.union(&Value::new_number(20)).is_err());
    }

    #[test]
    fn test_value_string_union() {
        let mut val = Value::new_string("test");
        val.union(&Value::new_string("test")).unwrap();
        assert_eq!(val, Value::new_string("test"));

        let mut val = Value::new_string("test");
        assert!(val.union(&Value::new_string("other")).is_err());
    }

    #[test]
    fn test_value_array_union() {
        let mut val = Value::new_array(vec![Value::new_string("a")]);
        val.union(&Value::new_array(vec![Value::new_string("b")]))
            .unwrap();
        assert_eq!(
            val,
            Value::new_array(vec![Value::new_string("a"), Value::new_string("b")])
        );
    }

    #[test]
    fn test_value_object_union() {
        let mut a = Value::new_object([
            ("a".to_owned(), Value::new_string("a")),
            (
                "b".to_owned(),
                Value::new_object([(
                    "c".to_owned(),
                    Value::new_array(vec![Value::new_string("c")]),
                )]),
            ),
        ]);
        let b = Value::new_object([
            ("a".to_owned(), Value::new_string("a")),
            (
                "b".to_owned(),
                Value::new_object([(
                    "c".to_owned(),
                    Value::new_array(vec![Value::new_string("d")]),
                )]),
            ),
            ("e".to_owned(), Value::new_string("e")),
        ]);
        let expected = Value::new_object([
            ("a".to_owned(), Value::new_string("a")),
            (
                "b".to_owned(),
                Value::new_object([(
                    "c".to_owned(),
                    Value::new_array(vec![Value::new_string("c"), Value::new_string("d")]),
                )]),
            ),
            ("e".to_owned(), Value::new_string("e")),
        ]);

        a.union(&b).unwrap();
        assert_eq!(a, expected);
    }

    #[test]
    fn test_value_object_override() {
        let mut a = Value::new_object([
            ("a".to_owned(), Value::new_string("a")),
            (
                "b".to_owned(),
                Value::new_object([(
                    "c".to_owned(),
                    Value::new_array(vec![Value::new_string("c")]),
                )]),
            ),
        ]);
        let b = Value::new_object([
            ("a".to_owned(), Value::new_string("X")),
            (
                "b".to_owned(),
                Value::new_object([(
                    "c".to_owned(),
                    Value::new_array(vec![Value::new_string("Y")]),
                )]),
            ),
            ("e".to_owned(), Value::new_string("Z")),
        ]);
        let expected = Value::new_object([
            ("a".to_owned(), Value::new_string("X")),
            (
                "b".to_owned(),
                Value::new_object([(
                    "c".to_owned(),
                    Value::new_array(vec![Value::new_string("Y")]),
                )]),
            ),
            ("e".to_owned(), Value::new_string("Z")),
        ]);

        a.override_with(&b);
        assert_eq!(a, expected);
    }
}
