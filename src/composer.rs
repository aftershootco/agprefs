use crate::types::{Agpref, Value};
use cookie_factory::{combinator::string, sequence::tuple, GenResult};
use std::io::BufWriter;
use std::io::Write;

impl Agpref<'_> {
    /// Write the struct to a buffer
    pub fn write<W: Write>(&self, mut w: W) -> Result<(), crate::errors::Errors> {
        let mut bw = BufWriter::new(&mut w);
        let cfw = cookie_factory::WriteContext::from(&mut bw);
        gen_agpref(self, cfw)?;
        Ok(())
    }
    /// Write the struct to a string
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
    result = string(" = ")(result)?;

    // let mut len = match agpref.values {
    //     Value::Struct(ref s) => s.len(),
    //     Value::Values(ref v) => v.len(),
    //     _ => panic!("Impossible"),
    // };
    // for (name, value) in agpref.values.iter() {
    //     result = string(name)(result)?;
    //     result = string(" = ")(result)?;
    //     result = compose_value(value, result)?;
    //     if len > 1 {
    //         result = string(",\n")(result)?;
    //         len -= 1;
    //     }
    // }
    result = compose_value(
        &agpref.values,
        Info {
            newline: true,
            ..Default::default()
        },
        result,
    )?;

    result = string("\n")(result)?;
    Ok(result)
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Info {
    newline: bool,
    inherit: bool,
    depth: usize,
}

// #[derive(Debug, Clone, Copy, Default)]
// pub struct ComposeInfo {
//     pub newline: bool,
//     pub inherit: Inherit,
//     pub indent: Indent,
// }

// impl ComposeInfo {
//     pub fn step_in(self) -> Self {
//         Self {
//             indent: Indent {
//                 indent: self.indent.indent,
//                 depth: self.indent.depth + 1,
//             },
//             inherit: Inherit {
//                 inherit: self.inherit.inherit,
//                 max_depth: self.inherit.max_depth - 1,
//             },
//             ..self
//         }
//     }
// }

// #[derive(Debug, Clone, Copy, Default)]
// pub struct Inherit {
//     pub inherit: bool,
//     pub max_depth: usize,
// }

// #[derive(Debug, Clone, Copy, Default)]
// pub struct Indent {
//     pub indent: bool,
//     pub depth: usize,
// }

pub fn compose_value<W: Write>(
    value: &Value,
    info: Info,
    writer: cookie_factory::WriteContext<W>,
) -> GenResult<W> {
    let result = match value {
        Value::String(s) => tuple((string("\""), string(escape_string(s)), string("\"")))(writer)?,
        Value::Int(i) => string(i.to_string())(writer)?,
        Value::Float(f) => string(f.to_string())(writer)?,
        Value::Bool(b) => string(b.to_string())(writer)?,
        Value::Values(values) => {
            let mut result = writer;
            if info.newline {
                result = string("{\n")(result)?;
            } else {
                result = string("{ ")(result)?;
            }

            let mut len = values.len();
            for value in values {
                result = compose_value(
                    value,
                    Info {
                        inherit: if info.depth > 0 { info.inherit } else { false },
                        depth: if info.depth > 1 { info.depth - 1 } else { 0 },
                        newline: if info.depth > 0 { info.newline } else { false },
                    },
                    result,
                )?;
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
            if info.newline {
                result = string("{\n")(result)?;
            } else {
                result = string("{ ")(result)?;
            }
            let mut len = s.len();
            for (name, value) in s {
                result = string(name)(result)?;
                result = string(" = ")(result)?;
                result = compose_value(
                    value,
                    Info {
                        inherit: if info.depth > 0 { info.inherit } else { false },
                        depth: if info.depth > 1 { info.depth - 1 } else { 0 },
                        newline: if info.depth > 0 { info.newline } else { false },
                    },
                    result,
                )?;
                if len > 1 {
                    result = string(",\n")(result)?;
                    len -= 1;
                }
            }
            if info.newline {
                result = string("\n}")(result)?;
            } else {
                result = string(" }")(result)?;
            }
            result
        }
        Value::Unit => string("{ }")(writer)?,
        _ => unimplemented!(),
    };
    Ok(result)
}

// #[cfg(feature = "namedlist")]
pub fn escape_string<'str>(
    input: &'str (impl AsRef<str> + 'str + ?Sized),
) -> std::borrow::Cow<'str, str> {
    if memchr::memchr3(b'\\', b'"', b'\n', input.as_ref().as_bytes()).is_some()
        || memchr::memchr2(b'\r', b'\t', input.as_ref().as_bytes()).is_some()
    {
        let mut result = String::with_capacity(input.as_ref().len() + 20);
        for c in input.as_ref().chars() {
            match c {
                '\\' => result.push_str("\\\\"),
                '"' => result.push_str("\\\""),
                '\n' => result.push_str("\\\n"),
                '\r' => result.push_str("\\r"),
                // '\t' => result.push_str("\\t"),
                _ => result.push(c),
            }
        }
        std::borrow::Cow::Owned(result)
    } else {
        std::borrow::Cow::Borrowed(input.as_ref())
    }
}
