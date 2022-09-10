use crate::types::*;
use nom::{
    branch::alt,
    bytes::complete::*,
    character::complete::*,
    combinator::*,
    error::{Error, ErrorKind},
    multi::*,
    sequence::*,
    IResult,
};
impl Agpref {
    pub fn from_str(s: impl AsRef<str>) -> Result<Agpref, crate::errors::Errors> {
        Ok(_agprefs(s.as_ref())?.1)
    }
}
fn _agprefs(s: &str) -> Result<(&str, Agpref), nom::Err<nom::error::Error<&str>>> {
    let (s, name) = get_key(s)?;
    let mut prefs = Agpref::with_name(name);
    let (s, _) = equals(s)?;
    let (s, v) = item_list(s)?;
    prefs.values = v.into_iter().map(|i| (i.name, i.value)).collect();

    if !s.is_empty() {
        return Err(nom::Err::Failure(nom::error::Error::new(
            "Unable parse the whole file",
            ErrorKind::Complete,
        )));
    }

    Ok((s, prefs))
}

fn get_item<'a>(s: &str) -> IResult<&str, Item> {
    alt((
        get_num,
        alt((
            get_float,
            alt((
                get_bool,
                alt((
                    get_unit,
                    alt((
                        get_vec,
                        alt((
                            get_escaped_string,
                            alt((get_struct, alt((get_sstruct, value_list)))),
                        )),
                    )),
                )),
            )),
        )),
    ))(s)
}

fn get_escaped_string(s: &str) -> IResult<&str, Item> {
    let (s, key) = get_key(s)?;
    let (s, _) = equals(s)?;
    let (s, _) = quote(s)?;
    let (s, text) = esc(s)?;
    let (s, _) = quote(s)?;
    if let Ok((_, t)) = named_list(&text) {
        return Ok((s, (key, t).into()));
    }

    Ok((s, (key, text).into()))
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

fn get_float(s: &str) -> IResult<&str, Item> {
    let (s, key) = get_key(s)?;
    let (s, _) = equals(s)?;
    // let (s, text) = i64(s)?;
    let (s, text) = nom::number::complete::double(s)?;
    Ok((s, (key, text).into()))
}

fn get_num(s: &str) -> IResult<&str, Item> {
    let (s, key) = get_key(s)?;
    let (s, _) = equals(s)?;
    let (s, text) = take_until(",")(s)?;
    let (_, val) = all_consuming(i64)(text)?;
    Ok((s, (key, val).into()))
}

fn get_bool(s: &str) -> IResult<&str, Item> {
    let (s, key) = get_key(s)?;
    let (s, _) = equals(s)?;
    let (s, text) = alphanumeric1(s)?;

    match text.to_ascii_lowercase().as_str() {
        "true" => Ok((s, (key, true).into())),
        "false" => Ok((s, (key, false).into())),
        _ => Err(nom::Err::Error(Error::new(
            "Unable to read boolean",
            ErrorKind::Tag,
        ))),
    }
}

fn get_unit(s: &str) -> IResult<&str, Item> {
    let (s, key) = get_key(s)?;
    let (s, _) = equals(s)?;
    let (s, _) = open(s)?;
    let (s, _) = close(s)?;
    Ok((s, (key, ()).into()))
}

fn get_vec(s: &str) -> IResult<&str, Item> {
    let (s, key) = get_key(s)?;
    let (s, _) = equals(s)?;
    let (s, _) = open(s)?;
    let (s, v) = separated_list1(comma, i64)(s)?;
    let (s, _) = close(s)?;
    Ok((s, (key, v).into()))
}

fn get_struct(s: &str) -> IResult<&str, Item> {
    let (s, name) = get_key(s)?;
    // let mut prefs = Agpref::with_name(name);
    let (s, _) = equals(s)?;
    let (s, v) = item_list(s)?;

    Ok((s, (name, v).into()))
}

// TODO: Do this properly
fn get_sstruct(s: &str) -> IResult<&str, Item> {
    // println!("\x1b[33m{s}\x1b[0m");
    let (s, name) = get_key(s)?;
    let (s, _) = equals(s)?;
    let (s, v) = item_llist(s)?;

    Ok((s, (name, v).into()))
}

// fn get_unquoted_string(s: &str) -> IResult<&str, Item> {
//     let (s, key) = get_key(s)?;
//     let (s, _) = equals(s)?;
//     let (s, text) = take_until(",")(s)?;
//     Ok((s, (key, text).into()))
// }

// TODO: Do this properly
fn item_llist(s: &str) -> IResult<&str, Vec<Item>> {
    let (s, _) = double_open(s)?;
    let (s, v) = separated_list1(comma, get_item)(s)?;
    let (s, _) = double_close(s)?;
    Ok((s, v))
}

fn item_list(s: &str) -> IResult<&str, Vec<Item>> {
    let (s, _) = open(s)?;
    let (s, v) = separated_list1(comma, get_item)(s)?;
    // There might be an optional trailing comma.
    // let (s, _) = opt(tuple((multispace0, tag(","), multispace0)))(s)?;
    let (s, _) = opt(comma)(s)?;
    let (s, _) = close(s)?;
    Ok((s, v))
}

fn get_key(s: &str) -> IResult<&str, &str> {
    let (s, _) = multispace0(s)?;
    let (s, key) = take_until(" ")(s)?;
    let (s, _) = multispace0(s)?;
    Ok((s, key))
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

fn double_open(s: &str) -> IResult<&str, &str> {
    recognize(tuple((open, open)))(s)
}

fn close(s: &str) -> IResult<&str, &str> {
    recognize(tuple((multispace0, tag("}"), multispace0)))(s)
}

fn double_close(s: &str) -> IResult<&str, &str> {
    recognize(tuple((close, close)))(s)
}

fn named_list(s: &str) -> IResult<&str, NamedList> {
    let (s, key) = get_key(s)?;
    let (s, _) = equals(s)?;
    let (s, _) = open(s)?;
    let (s, v) = separated_list0(comma, delimited(tag("\""), esc, tag("\"")))(s)?;
    let (s, _) = opt(comma)(s)?;
    let (s, _) = close(s)?;
    Ok((
        s,
        NamedList {
            name: key.into(),
            values: v.into_iter().map(Into::into).collect(),
        },
    ))
}

fn value_list(s: &str) -> IResult<&str, Item> {
    let (s, key) = get_key(s)?;
    let (s, _) = equals(s)?;
    let (s, _) = open(s)?;
    let (s, v) = separated_list0(comma, delimited(tag("\""), esc, tag("\"")))(s)?;
    let (s, _) = opt(comma)(s)?;
    let (s, _) = close(s)?;
    Ok((
        s,
        (key, Value::Values(v.into_iter().map(Into::into).collect())).into(),
    ))
}
