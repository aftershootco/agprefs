use serde::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Value {
    // Core types
    #[default]
    Unit,
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Values(Vec<Value>),
    Struct(HashMap<String, Value>),
    // Extra items
    // Opaque(String),
    #[cfg(feature = "namedlist")]
    NamedList(NamedList),
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::*;
        match self {
            Value::Int(i) => serializer.serialize_i64(*i),
            Value::Float(f) => serializer.serialize_f64(*f),
            Value::Bool(b) => serializer.serialize_bool(*b),
            Value::String(s) => serializer.serialize_str(s),
            Value::Values(v) => {
                let mut vs = serializer.serialize_seq(Some(v.len()))?;
                for i in v {
                    vs.serialize_element(i)?;
                }
                vs.end()
            }
            Value::Struct(s) => {
                let mut ss = serializer.serialize_map(Some(s.len()))?;
                for (k, v) in s {
                    ss.serialize_entry(&k, v)?;
                }
                ss.end()
            }
            Value::Unit => serializer.serialize_unit(),
            #[cfg(feature = "namedlist")]
            Value::NamedList(n) => n.serialize(serializer),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Bool(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
            Value::Values(v) => write!(f, "{:?}", v),
            Value::Struct(s) => write!(f, "{:?}", s),
            Value::Unit => write!(f, "{{}}"),
            #[cfg(feature = "namedlist")]
            Value::NamedList(nl) => write!(f, "{:?}", nl),
            // Value::Opaque(o) => write!(f, "{}", o),
        }
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Unit
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Int(i)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Float(f)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}

#[cfg(feature = "namedlist")]
impl<T: Into<NamedList>> From<T> for Value {
    fn from(nl: T) -> Self {
        Value::NamedList(nl.into())
    }
}

impl From<HashMap<String, Value>> for Value {
    fn from(s: HashMap<String, Value>) -> Self {
        Value::Struct(s)
    }
}

impl From<Vec<Item>> for Value {
    fn from(vs: Vec<Item>) -> Self {
        Value::Struct(vs.into_iter().map(|i| (i.name, i.value)).collect())
    }
}

impl<T> From<Vec<T>> for Value
where
    T: Into<Value>,
{
    fn from(v: Vec<T>) -> Self {
        Value::Values(v.into_iter().map(|x| x.into()).collect())
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Item {
    pub name: String,
    pub value: Value,
}

impl<S, V> From<(S, V)> for Item
where
    S: Into<String>,
    V: Into<Value>,
{
    fn from(sv: (S, V)) -> Self {
        Item {
            name: sv.0.into(),
            value: sv.1.into(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Agpref {
    pub name: String,
    pub values: HashMap<String, Value>,
}

impl Serialize for Agpref {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::*;
        let mut ss = serializer.serialize_map(Some(1))?;
        ss.serialize_entry(&self.name, &self.values)?;
        ss.end()
    }
}

impl Agpref {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_name(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Self::default()
        }
    }
}

impl std::ops::Deref for Agpref {
    type Target = HashMap<String, Value>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl std::ops::DerefMut for Agpref {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct NamedList {
    pub name: String,
    pub values: Vec<Value>,
}

impl<S, V> From<(S, V)> for NamedList
where
    S: Into<String>,
    V: Into<Vec<Value>>,
{
    fn from(sv: (S, V)) -> Self {
        NamedList {
            name: sv.0.into(),
            values: sv.1.into(),
        }
    }
}

impl std::ops::Deref for NamedList {
    type Target = Vec<Value>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl std::ops::DerefMut for NamedList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}
