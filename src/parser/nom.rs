use crate::types::*;
use std::collections::hash_map::RandomState;
type HashMap<K, V, S = RandomState> = indexmap::IndexMap<K, V, S>;
use nom::{
    branch::alt,
    bytes::complete::*,
    character::complete::*,
    combinator::*,
    error::{ErrorKind, ParseError},
    multi::*,
    sequence::*,
    IResult,
};
use std::borrow::Cow;

impl Agpref<'_> {
    /// Parse the given string into an Agpref struct.
    #[deprecated]
    pub fn from_str(s: &str) -> Result<Agpref, crate::errors::Errors> {
        Self::parse(s)
    }

    #[inline(always)]
    pub fn parse(s: &str) -> Result<Agpref, crate::errors::Errors> {
        Ok(_agprefs(s)?.1)
    }

    pub fn into_static(self) -> Agpref<'static> {
        Agpref {
            name: Cow::Owned(self.name.into_owned()),
            values: self.values.into_static(),
        }
    }
}

// impl<'a> FromStr for Agpref<'a> {
//     type Err = crate::errors::Errors;

//     fn from_str(s: &'a str) -> Result<Agpref<'a>, Self::Err> {
//         Self::from_str(s)
//     }
// }

fn _agprefs(s: &str) -> Result<(&str, Agpref), nom::Err<nom::error::Error<&str>>> {
    let (s, (name, value)) = get_key_value(s)?;
    let mut prefs = Agpref::with_name(name);
    match value {
        Value::Struct(_) => prefs.values = value,
        Value::Values(_) => prefs.values = value,

        _ => return Err(nom::Err::Error(nom::error::Error::new(s, ErrorKind::Fail))),
    }
    Ok((s, prefs))
}

#[test]
fn esc_test() {
    let s = esc(r#"C:\\Users\\harsh\\Pictures\\Lightroom\\Lightroom Catalog.lrcat"#).unwrap();
    assert_eq!(
        s,
        (
            "",
            "C:\\Users\\harsh\\Pictures\\Lightroom\\Lightroom Catalog.lrcat".into()
        )
    );
    let s = "d 0.578103 0.415124";
    assert_eq!(esc(s).unwrap(), ("", s.to_string().into()));
}
#[test]
fn esc_test_empty() {
    let s = esc(r#"""#).unwrap();
    assert_eq!(s, (r#"""#, String::new().into()));
}
// #[test]
// fn esc_test_cow() {
//     let s = esc(r#" "\"" "#).unwrap();
//     assert_eq!(s, ("", r"\\".into()));
// }

/// Returns an escaped string from a double escaped string
fn esc(input: &str) -> IResult<&str, Cow<'_, str>> {
    // Is it an empty string ?
    let (input, v) = opt(peek(tag("\"")))(input)?;
    if v.is_some() {
        return Ok((input, Cow::Borrowed("")));
    }

    escaped_transform(
        none_of("\r\n\\\""),
        '\\',
        alt((
            value("\\", tag("\\")),
            value("\"", tag("\"")),
            value("\n", tag("\n")),
            value("\n", tag("\r\n")),
            value("\r", tag("\r")),
        )),
    )(input)
    .map(|(s, r)| (s, Cow::Owned(r)))
}

fn get_key(s: &str) -> IResult<&str, &str> {
    let (s, _) = multispace0(s)?;
    let (s, key) = take_till1(|c| c == ' ' || c == '=')(s)?;
    let (s, _) = multispace0(s)?;
    Ok((s, key))
}

pub fn take_eov(s: &str) -> IResult<&str, &str> {
    take_till1(|c| c == ',' || c == ' ' || c == '}' || c == '\n')(s)
}

fn quote(s: &str) -> IResult<&str, &str> {
    recognize(tuple((multispace0, tag("\""), multispace0)))(s)
}

fn equals(s: &str) -> IResult<&str, &str> {
    recognize(tuple((multispace0, tag("="), multispace0)))(s)
}
fn comma(s: &str) -> IResult<&str, &str> {
    recognize(tuple((multispace0, tag(","), multispace0)))(s)
}

fn open(s: &str) -> IResult<&str, &str> {
    recognize(tuple((multispace0, tag("{"), multispace0)))(s)
}

fn close(s: &str) -> IResult<&str, &str> {
    recognize(tuple((multispace0, tag("}"), multispace0)))(s)
}

pub fn get_value(s: &str) -> IResult<&str, Value> {
    alt((
        map(get_vec, Value::from),
        map(get_struct, Value::from),
        #[cfg(feature = "namedlist")]
        map(get_namedlist, Value::from),
        map(get_string, Value::from),
        map(get_num, Value::from),
        map(get_float, Value::from),
        map(get_bool, Value::from),
        map(get_unit, Value::from),
    ))(s)
}

fn get_string(s: &str) -> IResult<&str, std::borrow::Cow<'_, str>> {
    let (s, _) = quote(s)?;
    let (s, text) = esc(s)?;
    let (s, _) = quote(s)?;

    Ok((s, text))
}

fn get_num(s: &str) -> IResult<&str, i64> {
    let (s, _) = multispace0(s)?;
    let (s, num) = take_eov(s)?;
    let (s, _) = multispace0(s)?;
    Ok((
        s,
        num.parse::<i64>().map_err(|_| {
            nom::Err::Error(ParseError::from_error_kind(
                "Failed to parse as float",
                ErrorKind::AlphaNumeric,
            ))
        })?,
    ))
}

fn get_float(s: &str) -> IResult<&str, f64> {
    let (s, _) = multispace0(s)?;
    let (s, float) = take_eov(s)?;
    let (s, _) = multispace0(s)?;
    Ok((
        s,
        float.parse::<f64>().map_err(|_| {
            nom::Err::Error(ParseError::from_error_kind(
                "Failed to parse as float",
                ErrorKind::AlphaNumeric,
            ))
        })?,
    ))
}

fn get_bool(s: &str) -> IResult<&str, bool> {
    let (s, _) = multispace0(s)?;
    let (s, text) = alphanumeric1(s)?;
    let (s, _) = multispace0(s)?;
    match text {
        "true" => Ok((s, true)),
        "false" => Ok((s, false)),
        _ => Err(nom::Err::Error(ParseError::from_error_kind(
            "Failed to parse as bool",
            ErrorKind::AlphaNumeric,
        ))),
    }
}

fn get_unit(s: &str) -> IResult<&str, ()> {
    let (s, _) = multispace0(s)?;
    let (s, _) = open(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = close(s)?;
    let (s, _) = multispace0(s)?;
    Ok((s, ()))
}

fn get_vec(s: &str) -> IResult<&str, Vec<Value<'_>>> {
    let (s, _) = open(s)?;
    let (s, v) = separated_list0(comma, get_value)(s)?;
    let (s, _) = opt(comma)(s)?;
    let (s, _) = close(s)?;
    Ok((s, v))
}

pub fn get_key_value(s: &str) -> IResult<&str, (&str, Value<'_>)> {
    let (s, k) = get_key(s)?;
    let (s, _) = equals(s)?;
    let (s, v) = get_value(s)?;
    Ok((s, (k, v)))
}

fn get_struct(s: &str) -> IResult<&str, HashMap<Cow<'_, str>, Value<'_>>> {
    let (s, _) = open(s)?;
    let (s, v) = separated_list0(comma, get_key_value)(s)?;
    let (s, _) = opt(comma)(s)?;
    let (s, _) = close(s)?;
    Ok((
        s,
        v.into_iter().map(|v| (Cow::Borrowed(v.0), v.1)).collect(),
    ))
}

#[cfg(feature = "namedlist")]
fn get_namedlist<'v>(s: &'v str) -> IResult<&'v str, Value<'v>> {
    let (s, _) = quote(s)?;
    let (s, text) = esc(s)?;
    let (s, _) = quote(s)?;

    use std::borrow::Borrow;
    let (ts, kv) = get_key_value(text.borrow()).map_err(|_| {
        nom::Err::Error(ParseError::from_error_kind(
            "Failed to parse as named list",
            ErrorKind::AlphaNumeric,
        ))
    })?;
    if !ts.is_empty() {
        return Err(nom::Err::Error(ParseError::from_error_kind(
            "Failed to parse as named list",
            ErrorKind::AlphaNumeric,
        )));
    }

    if let (name, Value::Values(v)) = kv {
        Ok((s, (name, v).into()))
    } else {
        Err(nom::Err::Error(ParseError::from_error_kind(
            "Failed to parse as named list",
            ErrorKind::AlphaNumeric,
        )))
    }
}
