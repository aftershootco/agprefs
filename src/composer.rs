use crate::types::{Agpref, Value};
use cookie_factory::{combinator::string, sequence::tuple, GenResult};
use std::io::Write;

pub fn compose_agpref(agpref: &Agpref, mut w: impl Write) {
    let mut buf = Vec::new();
    let www = cookie_factory::WriteContext::from(&mut buf);
    let gen = gen_agpref(agpref, www);
    match gen {
        Ok(_) => {
            let _ = w.write(&buf);
        }
        Err(_) => {
            println!("Error composing agpref");
        }
    }
}

pub fn gen_agpref<W: Write>(
    agpref: &Agpref,
    writer: cookie_factory::WriteContext<W>,
) -> cookie_factory::GenResult<W> {
    let mut result = writer;
    result = string(&agpref.name)(result)?;
    result = string(" = { ")(result)?;

    let mut len = agpref.values.len();
    for (name, value) in agpref.values.iter() {
        result = string(name)(result)?;
        result = string(" = ")(result)?;
        result = compose_value(&value, result)?;
        if len > 1 {
            result = string(",\n")(result)?;
            len -= 1;
        }
    }
    result = string(" }\n\n")(result)?;
    Ok(result)
}

pub fn compose_value<W: Write>(
    value: &Value,
    writer: cookie_factory::WriteContext<W>,
) -> GenResult<W> {
    let result = match value {
        Value::String(s) => tuple((string("\""), string(s), string("\"")))(writer)?,
        Value::Int(i) => string(i.to_string())(writer)?,
        Value::Float(f) => string(f.to_string())(writer)?,
        Value::Bool(b) => string(b.to_string())(writer)?,
        Value::Values(values) => {
            let mut result = writer;
            result = string("{ ")(result)?;
            let mut len = values.len();
            for value in values {
                result = compose_value(&value, result)?;
                if len > 1 {
                    result = string(",\n")(result)?;
                    len -= 1;
                }
            }
            result = string(" }")(result)?;
            result
        }
        Value::Struct(s) => {
            let mut result = writer;
            result = string("{ ")(result)?;
            let mut len = s.len();
            for (name, value) in s {
                result = string(name)(result)?;
                result = string(" = ")(result)?;
                result = compose_value(value, result)?;
                if len > 1 {
                    result = string(",\n")(result)?;
                    len -= 1;
                }
            }
            result = string(" }")(result)?;
            result
        }
        Value::Unit => string("{ }")(writer)?,

        #[cfg(feature = "namedlist")]
        // Value::NamedList(nl) => compose_namedlist(nl, writer)?,
        Value::NamedList(_nl) => unimplemented!(),
    };
    // let result = string("\n")(result)?;
    Ok(result)
}

// #[cfg(feature = "namedlist")]
// pub fn compose_namedlist<W: Write>(
//     namedlist: &crate::types::NamedList,
//     writer: cookie_factory::WriteContext<W>,
// ) -> GenResult<W> {
//     let mut result = writer;
//     result = string("\"")(result)?;
//     result = string(&namedlist.name)(result)?;
//     result = string(" = {\\\n")(result)?;
//     let mut len = namedlist.values.len();
//     for value in &namedlist.values {
//         result = if let Value::String(s) = value {
//             tuple((string("\\\""), string(escape_string(s)), string("\\\"")))(result)?
//         } else {
//             compose_value(&value, result)?
//         };
//         if len > 1 {
//             result = string(",\\\n")(result)?;
//             len -= 1;
//         }
//     }
//     result = string(" }\\\n")(result)?;
//     result = string("\"")(result)?;
//     Ok(result)
// }

// pub fn escape_string(input: &str) -> std::borrow::Cow<str> {
//     // use nom::*;
//     use nom::character::complete::one_of;
//     one_of("\n\\\"")
// }
