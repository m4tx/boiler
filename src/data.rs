use std::collections::HashMap;
use std::ops::Index;
use std::path::PathBuf;

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
    Object(HashMap<String, Value>),
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
    pub fn new_string(value: String) -> Self {
        Self::String(value)
    }

    #[must_use]
    pub fn new_array(value: Vec<Value>) -> Self {
        Self::Array(value)
    }

    #[must_use]
    pub fn new_object(value: HashMap<String, Value>) -> Self {
        Self::Object(value)
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
    pub fn as_object(&self) -> Option<&HashMap<String, Value>> {
        match self {
            Self::Object(value) => Some(value),
            _ => None,
        }
    }

    pub fn insert(&mut self, key: String, value: Value) {
        match self {
            Self::Object(map) => {
                map.insert(key, value);
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
}

impl Default for Value {
    fn default() -> Self {
        Self::new_null()
    }
}

impl Index<&str> for Value {
    type Output = Value;

    fn index(&self, index: &str) -> &Self::Output {
        match self {
            Self::Object(value) => value.get(index).unwrap(),
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
