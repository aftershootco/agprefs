use pretty_assertions::{assert_eq, assert_ne};

use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::container::Container;
use chumsky::prelude::*;
use indexmap::IndexMap;
use std::borrow::Cow;
use std::collections::hash_map::RandomState;
// type HashMap<K, V, S = RandomState> = indexmap::IndexMap<K, V, S>;
// use std::collections::HashMap;
pub struct IndexWrap<K, V, S = RandomState>(pub IndexMap<K, V, S>);
impl<K, V> Default for IndexWrap<K, V> {
    fn default() -> Self {
        Self(IndexMap::default())
    }
}

impl<K: Eq + core::hash::Hash, V> Container<(K, V)> for IndexWrap<K, V> {
    fn with_capacity(n: usize) -> Self {
        Self(IndexMap::with_capacity(n))
    }
    fn push(&mut self, (key, value): (K, V)) {
        self.0.insert(key, value);
    }
}

use crate::types::*;
impl Agpref<'_> {
    #[inline(always)]
    pub fn cparse(src: &str) -> Result<Agpref, crate::errors::Errors> {
        match agpref().parse(src.trim()).into_result() {
            Err(errs) => {
                #[cfg(feature = "fancy_errors")]
                errs.iter()
                    .try_for_each(|e| -> Result<(), crate::errors::Errors> {
                        Report::build(ReportKind::Error, (), e.span().start)
                            .with_message(e.to_string())
                            .with_label(
                                Label::new(e.span().into_range())
                                    .with_message(e.reason().to_string())
                                    .with_color(Color::Red),
                            )
                            .finish()
                            .print(Source::from(&src))?;
                        Ok(())
                    })?;

                Err(crate::errors::Errors::Other(
                    errs.into_iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join("\n"),
                ))
            }
            Ok(agpref) => Ok(agpref),
        }
    }
}

fn agpref<'a>() -> impl Parser<'a, &'a str, Agpref<'a>, extra::Err<Rich<'a, char>>> {
    value().then_ignore(end()).map(|v| {
        if let Value::Root(k, v) = v {
            Agpref {
                name: k,
                values: *v,
            }
        } else {
            unreachable!()
        }
    })
}

fn value<'a>() -> impl Parser<'a, &'a str, Value<'a>, extra::Err<Rich<'a, char>>> {
    recursive(|value| {
        let digits = text::digits(10).slice();
        let mantissa = just('.').then(digits);
        let int = just('-').or_not().then(digits).map_slice(|input: &str| {
            Value::Int(input.parse::<i64>().expect("Impossible to reach"))
        });
        let float = int.then(mantissa).map_slice(|input: &str| {
            Value::Float(input.parse::<f64>().expect("Impossible to reach"))
        });

        let bool = just("true")
            .to(true)
            .or(just("false").to(false))
            .map(Value::Bool);

        let unit = just('{')
            .ignored()
            .then(text::whitespace())
            .then(just('}').ignored())
            .map(|_| Value::Unit);

        let escape = just('\\')
            .then(choice((
                just('\\'),
                just('\"'),
                just('\''),
                just('\n').to('\n'),
                just('n').to('\n'),
            )))
            .ignored();

        let string = none_of("\\\"")
            .ignored()
            .or(escape)
            .repeated()
            .slice()
            .map(|v: &str| Value::String(v.into()))
            .delimited_by(just('"'), just('"'));

        let ident = none_of(" \n\t\r=").repeated().slice().map(Cow::from);

        let key_value = ident
            .then_ignore(just('=').padded())
            .then(value.clone())
            .map(|(k, v)| (k, v));

        let object = key_value
            .clone()
            .separated_by(just(',').or_not().padded())
            .allow_trailing()
            .collect::<IndexWrap<Cow<'a, str>, Value<'a>>>()
            .delimited_by(just('{').padded(), just('}').padded())
            .map(|v| Value::Struct(v.0));

        // let object = object.or(key_value.clone().map(|(k, v)| Value::Root(k, Box::new(v))));

        let array = value
            .clone()
            .separated_by(just(',').or_not().padded())
            .at_least(1)
            .allow_trailing()
            .collect::<Vec<_>>()
            .delimited_by(just('{').padded(), just('}').padded())
            .map(Value::Values);

        let root = key_value.map(|(k, v)| Value::Root(k, Box::new(v)));

        choice((unit, object, float, int, bool, string, array, root)).padded()
    })
}
