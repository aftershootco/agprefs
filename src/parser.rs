use crate::types::*;
use indexmap::IndexMap as HashMap;
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
impl Agpref {
    pub fn from_str(s: impl AsRef<str>) -> Result<Agpref, crate::errors::Errors> {
        Ok(_agprefs(s.as_ref())?.1)
    }
}

// pub const BLUE: &str = "\x1b[34m";
// pub const GREEN: &str = "\x1b[32m";
// pub const RED: &str = "\x1b[31m";
// pub const RESET: &str = "\x1b[0m";
// pub const YELLOW: &str = "\x1b[33m";
// pub const PINK: &str = "\x1b[35m";

fn _agprefs(s: &str) -> Result<(&str, Agpref), nom::Err<nom::error::Error<&str>>> {
    let (s, (name, value)) = get_key_value(s)?;
    let mut prefs = Agpref::with_name(name);
    if let Value::Struct(v) = value {
        prefs.values = v;
    } else {
        return Err(nom::Err::Error(nom::error::Error::new(s, ErrorKind::Fail)));
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
            "C:\\Users\\harsh\\Pictures\\Lightroom\\Lightroom Catalog.lrcat".to_string()
        )
    );
    let s = "d 0.578103 0.415124";
    assert_eq!(esc(s).unwrap(), ("", s.to_string()));
}
#[test]
fn esc_test_empty() {
    let s = esc(r#"""#).unwrap();
    assert_eq!(s, (r#"""#, String::new()));
}

/// Returns an escaped string from a double escaped string
fn esc(input: &str) -> IResult<&str, String> {
    let (input, v) = opt(peek(tag("\"")))(input)?;
    if v.is_some() {
        return Ok((input, "".into()));
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

fn get_string(s: &str) -> IResult<&str, String> {
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

fn get_vec(s: &str) -> IResult<&str, Vec<Value>> {
    let (s, _) = open(s)?;
    let (s, v) = separated_list0(comma, get_value)(s)?;
    let (s, _) = opt(comma)(s)?;
    let (s, _) = close(s)?;
    Ok((s, v))
}

pub fn get_key_value(s: &str) -> IResult<&str, (&str, Value)> {
    let (s, k) = get_key(s)?;
    let (s, _) = equals(s)?;
    let (s, v) = get_value(s)?;
    Ok((s, (k, v)))
}

fn get_struct(s: &str) -> IResult<&str, HashMap<String, Value>> {
    let (s, _) = open(s)?;
    let (s, v) = separated_list0(comma, get_key_value)(s)?;
    let (s, _) = opt(comma)(s)?;
    let (s, _) = close(s)?;
    Ok((s, v.into_iter().map(|v| (v.0.to_owned(), v.1)).collect()))
}

#[cfg(feature = "namedlist")]
fn get_namedlist(s: &str) -> IResult<&str, Value> {
    let (s, _) = quote(s)?;
    let (s, text) = esc(s)?;
    let (s, _) = quote(s)?;

    let (ts, kv) = get_key_value(&text).map_err(|_| {
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
