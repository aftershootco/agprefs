use indexmap::IndexMap as HashMap;
#[cfg(feature = "serde")]
use serde::*;

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
    #[cfg(feature = "namedlist")]
    #[cfg_attr(docsrs, doc(cfg(feature = "namedlist")))]
    NamedList(NamedList),
}

#[cfg(feature = "serde")]
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
#[cfg(all(feature = "serde", feature = "composer"))]
impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::*;
        struct ValueVisitor;
        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a value")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Value::Int(v))
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Value::Int(v as i64))
            }
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Value::Float(v))
            }
            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Value::Bool(v))
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Value::String(v.to_string()))
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Value::String(v))
            }
            fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = visitor.next_element()? {
                    values.push(value);
                }
                Ok(Value::Values(values))
            }
            fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut values = HashMap::new();
                while let Some((key, value)) = visitor.next_entry()? {
                    values.insert(key, value);
                }
                Ok(Value::Struct(values))
            }
            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Value::Unit)
            }
            fn visit_none<E>(self) -> Result<Self::Value, E> {
                Ok(Value::Unit)
            }
        }
        deserializer.deserialize_any(ValueVisitor)
    }
}

#[cfg(feature = "serde")]
#[test]
fn test_value() {
    use crate::types::Value;

    fn assert_type(value: Value) {
        let str_out = serde_json::to_string(&value).expect("Failed to serialize");
        let origninal_val = serde_json::from_str::<Value>(&str_out).expect("Failed to deserialize");
        assert_eq!(value, origninal_val);
    }

    assert_type(Value::Int(69));
    assert_type(Value::Float(42.0));
    assert_type(Value::Bool(true));
    assert_type(Value::String("test".to_string()));
    assert_type(Value::Values(vec![Value::Int(69), Value::Float(42.0)]));
    assert_type(Value::Struct(
        vec![("test".to_string(), Value::Int(666))]
            .into_iter()
            .collect(),
    ));
    assert_type(Value::Unit);
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
/// A named list of key value pairs that can be used to represent a text field of lrcat files
pub struct Agpref {
    pub name: String,
    pub values: HashMap<String, Value>,
}

#[cfg(feature = "serde")]
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

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Agpref {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::*;
        pub struct AgprefVisitor;
        impl<'de> Visitor<'de> for AgprefVisitor {
            type Value = Agpref;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Agpref")
            }
            fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut name = None;
                let mut values = None;
                while let Some(key) = visitor.next_key()? {
                    match key {
                        n => {
                            if values.is_some() {
                                return Err(Error::duplicate_field("values"));
                            }
                            values = Some(visitor.next_value()?);
                            name = Some(n);
                        }
                    }
                }
                let name = name.ok_or_else(|| Error::missing_field("values"))?;
                let values = values.ok_or_else(|| Error::missing_field("values"))?;
                Ok(Agpref { name, values })
            }
        }
        deserializer.deserialize_struct("Agpref", &["values"], AgprefVisitor)
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

#[cfg(feature = "namedlist")]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(docsrs, doc(cfg(feature = "namedlist")))]
/// Named list of values for parsing agprefs files
/// Which store the recent lrcat catalogs
pub struct NamedList {
    pub name: String,
    pub values: Vec<Value>,
}

#[cfg(feature = "namedlist")]
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

#[cfg(feature = "namedlist")]
impl std::ops::Deref for NamedList {
    type Target = Vec<Value>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

#[cfg(feature = "namedlist")]
impl std::ops::DerefMut for NamedList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}
