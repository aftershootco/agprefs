use std::collections::HashMap;
#[derive(Debug, Clone, Default)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Values(Vec<Value>),
    NamedList(NamedList),
    // Item(Item),
    #[default]
    Unit,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Bool(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
            Value::Values(v) => write!(f, "{:?}", v),
            // Value::VecItem(v) => write!(f, "{:?}", v),
            // Value::VecAgpref(v) => write!(f, "{:?}", v),
            // Value::Agpref(v) => write!(f, "{:?}", v),
            Value::NamedList(nl) => write!(f, "{:?}", nl),
            Value::Unit => write!(f, "{{}}"),
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

impl From<NamedList> for Value {
    fn from(nl: NamedList) -> Self {
        Value::NamedList(nl)
    }
}

// impl From<Agpref> for Value {
//     fn from(a: Agpref) -> Self {
//         Value::Agpref(a)
//     }
// }

impl<T> From<Vec<T>> for Value
where
    T: Into<Value>,
{
    fn from(v: Vec<T>) -> Self {
        Value::Values(v.into_iter().map(|x| x.into()).collect())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Item {
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

#[derive(Debug, Default, Clone)]
pub struct Agpref {
    pub name: String,
    pub values: HashMap<String, Value>,
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

#[derive(Debug, Clone)]
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
