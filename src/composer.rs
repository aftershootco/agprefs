use crate::types::{Agpref, Value};
use cookie_factory::{combinator::string, sequence::tuple, GenResult};
use std::io::BufWriter;
use std::io::Write;

impl Agpref {
    pub fn write<W: Write>(&self, mut w: W) -> Result<(), crate::errors::Errors> {
        let mut bw = BufWriter::new(&mut w);
        let cfw = cookie_factory::WriteContext::from(&mut bw);
        gen_agpref(self, cfw)?;
        Ok(())
    }
    pub fn to_str(&self) -> Result<String, crate::errors::Errors> {
        let mut buf = Vec::new();
        let cfw = cookie_factory::WriteContext::from(&mut buf);
        gen_agpref(self, cfw)?;
        Ok(String::from_utf8(buf)?)
    }
}

fn gen_agpref<W: Write>(
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
