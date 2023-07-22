// use indexmap::IndexMap as HashMap;
use std::collections::hash_map::RandomState;
type HashMap<K, V, S = RandomState> = indexmap::IndexMap<K, V, S>;

#[cfg(feature = "serde")]
use serde::*;
use std::borrow::Cow;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Value<'v> {
    // Core types
    #[default]
    Unit,
    Int(i64),
    Float(f64),
    Bool(bool),
    String(Cow<'v, str>),
    Values(Vec<Value<'v>>),
    Struct(HashMap<Cow<'v, str>, Value<'v>>),
    Root(Cow<'v, str>, Box<Value<'v>>),
}

macro_rules! into_getter {
    ($name:ident, $ty:ty, $variant:ident) => {
        pub fn $name(self) -> Option<$ty> {
            match self {
                Value::$variant(v) => Some(v),
                _ => None,
            }
        }
    };
}

macro_rules! mut_getter {
    ($name:ident, $ty:ty, $variant:ident) => {
        pub fn $name(&mut self) -> Option<&mut $ty> {
            match self {
                Value::$variant(v) => Some(v),
                _ => None,
            }
        }
    };
}

impl<'v> Value<'v> {
    pub fn into_static(self) -> Value<'static> {
        match self {
            Value::Unit => Value::Unit,
            Value::Int(i) => Value::Int(i),
            Value::Float(f) => Value::Float(f),
            Value::Bool(b) => Value::Bool(b),
            Value::String(s) => Value::String(Cow::Owned(s.into_owned())),
            Value::Values(v) => Value::Values(v.into_iter().map(|v| v.into_static()).collect()),
            Value::Struct(s) => Value::Struct(
                s.into_iter()
                    .map(|(k, v)| (Cow::Owned(k.into_owned()), v.into_static()))
                    .collect(),
            ),
            Value::Root(k, v) => Value::Root(Cow::Owned(k.into_owned()), Box::new(v.into_static())),
        }
    }

    pub fn is_unit(&self) -> bool {
        matches!(self, Value::Unit)
    }

    pub fn get_int(&self) -> Option<i64> {
        match self {
            Value::Int(i) => Some(*i),
            _ => None,
        }
    }

    pub fn get_float(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn get_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn get_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn get_values(&self) -> Option<&[Value<'v>]> {
        match self {
            Value::Values(v) => Some(v),
            _ => None,
        }
    }

    pub fn get_struct(&self) -> Option<&HashMap<Cow<'v, str>, Value<'v>>> {
        match self {
            Value::Struct(s) => Some(s),
            _ => None,
        }
    }

    into_getter!(into_int, i64, Int);
    into_getter!(into_float, f64, Float);
    into_getter!(into_bool, bool, Bool);
    into_getter!(into_string, Cow<'v, str>, String);
    into_getter!(into_values, Vec<Value<'v>>, Values);
    into_getter!(into_struct, HashMap<Cow<'v, str>, Value<'v>>, Struct);

    mut_getter!(get_mut_int, i64, Int);
    mut_getter!(get_mut_float, f64, Float);
    mut_getter!(get_mut_bool, bool, Bool);
    mut_getter!(get_mut_string, Cow<'v, str>, String);
    mut_getter!(get_mut_values, Vec<Value<'v>>, Values);
    mut_getter!(get_mut_struct, HashMap<Cow<'v, str>, Value<'v>>, Struct);

}

#[cfg(feature = "serde")]
impl<'v> Serialize for Value<'v> {
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
            Value::Root(k, v) => {
                let mut ss = serializer.serialize_map(Some(1))?;
                ss.serialize_entry(&k, v)?;
                ss.end()
            }
        }
    }
}
#[cfg(all(feature = "serde", feature = "composer"))]
impl<'de: 'v, 'v> Deserialize<'de> for Value<'v> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::*;
        struct ValueVisitor;
        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value<'de>;
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
                // TODO: Try to make this 0 copy
                Ok(Value::String(Cow::Owned(v.to_string())))
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Value::String(Cow::Owned(v)))
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
                // let mut values = HashMap::with_capacity_and_hasher(1, Default::default());
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
    // assert_type(Value::String("test".to_string()));
    assert_type(Value::Values(vec![Value::Int(69), Value::Float(42.0)]));
    // assert_type(Value::Struct(
    //     vec![("test".to_string(), Value::Int(666))]
    //         .into_iter()
    //         .collect(),
    // ));
    assert_type(Value::Unit);
}

impl std::fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Bool(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
            Value::Values(v) => write!(f, "{:?}", v),
            Value::Struct(s) => write!(f, "{:?}", s),
            Value::Unit => write!(f, "{{}}"),
            Value::Root(k, v) => write!(f, "{{{}: {}}}", k, v),
        }
    }
}

impl From<()> for Value<'_> {
    fn from(_: ()) -> Self {
        Value::Unit
    }
}

impl From<i64> for Value<'_> {
    fn from(i: i64) -> Self {
        Value::Int(i)
    }
}

impl From<f64> for Value<'_> {
    fn from(f: f64) -> Self {
        Value::Float(f)
    }
}

impl From<bool> for Value<'_> {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<String> for Value<'_> {
    fn from(s: String) -> Self {
        Value::String(Cow::Owned(s))
    }
}

impl<'v> From<&'v str> for Value<'v> {
    fn from(s: &'v str) -> Self {
        Value::String(Cow::Borrowed(s))
    }
}

impl<'v> From<Cow<'v, str>> for Value<'v> {
    fn from(s: Cow<'v, str>) -> Self {
        Value::String(s)
    }
}

#[cfg(feature = "namedlist")]
impl<'v, T: Into<NamedList<'v>>> From<T> for Value<'v> {
    fn from(nl: T) -> Self {
        Value::NamedList(nl.into())
    }
}

impl<'v> From<HashMap<Cow<'v, str>, Value<'v>>> for Value<'v> {
    fn from(s: HashMap<Cow<'v, str>, Value<'v>>) -> Self {
        Value::Struct(s)
    }
}

impl<'v> From<Vec<Item<'v>>> for Value<'v> {
    fn from(vs: Vec<Item<'v>>) -> Self {
        Value::Struct(vs.into_iter().map(|i| (i.name, i.value)).collect())
    }
}

impl<'v, T> From<Vec<T>> for Value<'v>
where
    T: Into<Value<'v>>,
{
    fn from(v: Vec<T>) -> Self {
        Value::Values(v.into_iter().map(|x| x.into()).collect())
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Item<'i> {
    pub name: Cow<'i, str>,
    pub value: Value<'i>,
}

impl<'i, S, V> From<(S, V)> for Item<'i>
where
    S: Into<Cow<'i, str>>,
    V: Into<Value<'i>>,
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
pub struct Agpref<'a> {
    pub name: Cow<'a, str>,
    pub values: Value<'a>,
}

#[cfg(feature = "serde")]
impl<'a> Serialize for Agpref<'a> {
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
impl<'de> Deserialize<'de> for Agpref<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::*;
        pub struct AgprefVisitor;
        impl<'de> Visitor<'de> for AgprefVisitor {
            type Value = Agpref<'de>;
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
                    let n = key;
                    if values.is_some() {
                        return Err(Error::duplicate_field("values"));
                    }
                    values = Some(visitor.next_value()?);
                    name = Some(n);
                }
                let name = name.ok_or_else(|| Error::missing_field("values"))?;
                let values = values.ok_or_else(|| Error::missing_field("values"))?;
                Ok(Agpref { name, values })
            }
        }
        deserializer.deserialize_struct("Agpref", &["values"], AgprefVisitor)
    }
}

impl<'a> Agpref<'a> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_name(name: impl Into<Cow<'a, str>>) -> Self {
        Self {
            name: name.into(),
            ..Self::default()
        }
    }
}

impl<'a> std::ops::Deref for Agpref<'a> {
    type Target = Value<'a>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<'a> std::ops::DerefMut for Agpref<'a> {
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
pub struct NamedList<'n> {
    pub name: Cow<'n, str>,
    pub values: Vec<Value<'n>>,
}

#[cfg(feature = "namedlist")]
impl<'n, S, V> From<(S, V)> for NamedList<'n>
where
    S: Into<Cow<'n, str>>,
    V: Into<Vec<Value<'n>>>,
{
    fn from(sv: (S, V)) -> Self {
        NamedList {
            name: sv.0.into(),
            values: sv.1.into(),
        }
    }
}

#[cfg(feature = "namedlist")]
impl<'n> std::ops::Deref for NamedList<'n> {
    type Target = Vec<Value<'n>>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

#[cfg(feature = "namedlist")]
impl<'n> std::ops::DerefMut for NamedList<'n> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}
