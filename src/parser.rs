use nom::{
    branch::alt,
    bytes::complete::*,
    character::complete::*,
    combinator::*,
    // complete::*,
    error::{Error, ErrorKind},
    multi::*,
    sequence::*,
    IResult,
};

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Bool(bool),
    String(String),
    Vec(Vec<Value>),
    VecItem(Vec<Item>),
    Unit,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Bool(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
            Value::Vec(v) => write!(f, "{:?}", v),
            Value::VecItem(v) => write!(f, "{:?}", v),
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

impl<T> From<Vec<T>> for Value
where
    T: Into<Value>,
{
    fn from(v: Vec<T>) -> Self {
        Value::Vec(v.into_iter().map(|x| x.into()).collect())
    }
}

#[derive(Debug, Clone)]
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
    pub values: Vec<Item>,
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
    type Target = Vec<Item>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl std::ops::DerefMut for Agpref {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}

pub fn agprefs(s: &str) -> Result<Agpref, nom::Err<nom::error::Error<&str>>> {
    let (s, name) = take_until1("=")(s)?;
    let mut prefs = Agpref::with_name(name);
    let (s, _) = tag("=")(s)?;
    // let (s, _) = multispace0(s)?;
    // let (mut s, _) = tag("{")(s)?;

    // loop {
    //     let item = get_item(s);
    //     if item.is_err() {
    //         println!("{:?}", item);
    //         break;
    //     }
    //     if let Ok((s_, i)) = item {
    //         prefs.push(i.into());
    //         s = s_;
    //     };
    // }

    let (s, v) = item_list(s)?;
    prefs.values = v;

    if !s.is_empty() {
        // return nom::Err(Error(ErrorKind::Custom("Unexpected character".to_string())));
        return Err(nom::Err::Failure(nom::error::Error::new(
            "Couln't parse the whole file",
            ErrorKind::Complete,
        )));
    }

    Ok(prefs)
}

fn get_item<'a>(s: &str) -> IResult<&str, Item> {
    alt((
        get_escaped_string,
        alt((get_num, alt((get_bool, alt((get_unit, get_vec)))))),
    ))(s)
}

fn get_escaped_string(s: &str) -> IResult<&str, Item> {
    let (s, _) = multispace0(s)?;
    let (s, key) = take_until1("=")(s)?;
    let (s, _) = tag("=")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("\"")(s)?;
    let (s, text) = esc(s)?;
    let (s, _) = tag("\",")(s)?;
    // println!("\x1b[32m{}\x1b[0m", text);
    Ok((s, (key, text).into()))
}

fn esc(input: &str) -> IResult<&str, String> {
    escaped_transform(
        none_of("\r\n\\\""),
        '\\',
        alt((
            value("\\", tag("\\")),
            // value("\\", tag("\\\\")),
            value("\"", tag("\"")),
            value("\n", tag("\n")),
            value("\n", tag("\r\n")),
            value("\n", tag("\r")),
            // value("\r\n"
        )),
    )(input)
}

fn get_num(s: &str) -> IResult<&str, Item> {
    let (s, _) = multispace0(s)?;
    let (s, key) = alphanumeric1(s)?;
    let (s, _) = take_until1("=")(s)?;
    let (s, _) = tag("=")(s)?;
    let (s, _) = multispace0(s)?;
    // println!("\x1b[32m{}\x1b[0m", s);
    let (s, text) = digit1(s)?;
    let (s, _) = take_until(",")(s)?;
    let (s, _) = tag(",")(s)?;
    // println!("\x1b[32m{}\x1b[0m", text);
    Ok((s, (key, text).into()))
}

fn get_bool(s: &str) -> IResult<&str, Item> {
    let (s, _) = multispace0(s)?;
    let (s, key) = alphanumeric1(s)?;
    let (s, _) = take_until1("=")(s)?;
    let (s, _) = tag("=")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, text) = alphanumeric1(s)?;
    let (s, _) = take_until(",")(s)?;
    let (s, _) = tag(",")(s)?;

    match text.to_ascii_lowercase().as_str() {
        "true" => Ok((s, (key, true).into())),
        "false" => Ok((s, (key, false).into())),
        _ => Err(nom::Err::Error(Error::new("", ErrorKind::Tag))),
    }
}

fn get_unit(s: &str) -> IResult<&str, Item> {
    let (s, _) = multispace0(s)?;
    let (s, key) = alphanumeric1(s)?;
    let (s, _) = take_until1("=")(s)?;
    let (s, _) = tag("=")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("{")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("}")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = tag(",")(s)?;
    Ok((s, (key, ()).into()))
}

fn get_vec(s: &str) -> IResult<&str, Item> {
    let (s, _) = multispace0(s)?;
    let (s, key) = alphanumeric1(s)?;
    let (s, _) = take_until1("=")(s)?;
    let (s, _) = tag("=")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("{")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, v) = separated_list1(tuple((multispace0, tag(","), multispace0)), digit1)(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("}")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = tag(",")(s)?;
    Ok((s, (key, v).into()))
}

pub fn item_list(s: &str) -> IResult<&str, Vec<Item>> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("{")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, v) = separated_list1(tuple((multispace0, tag(","), multispace0)), get_item)(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("}")(s)?;
    let (s, _) = multispace0(s)?;
    Ok((s, v))
}


pub fn strip_equals(s: &str) -> IResult<&str, &str> {
    recognize(tuple((multispace0, tag("="), multispace0)))(s)
}
pub fn strip_comma(s: &str) -> IResult<&str, &str> {
    recognize(tuple((multispace0, tag(","), multispace0)))(s)
}
pub fn strip_open(s: &str) -> IResult<&str, &str> {
    recognize(tuple((multispace0, tag("{"), multispace0)))(s)
}
pub fn strip_close(s: &str) -> IResult<&str, &str> {
    recognize(tuple((multispace0, tag("}"), multispace0)))(s)
}

#[test]
fn list() {
    dbg!(recognize::<_, _, (), _>(tuple((
        multispace0,
        tag(","),
        multispace0
    )))(
        "
        ,
        "
    ))
    .unwrap();
    dbg!(pair::<_, _, _, (), _, _>(multispace0, digit1)("  01")).unwrap();
    let (s, v) = separated_list0::<&str, &str, &str, (), _, _>(
        recognize(tuple((multispace0, tag(","), multispace0))),
        digit1,
    )("1,   3,5")
    .unwrap();
    println!("{s}\n{v:?}")
}
