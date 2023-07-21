use std::borrow::Cow;
use std::collections::hash_map::RandomState;
type HashMap<K, V, S = RandomState> = indexmap::IndexMap<K, V, S>;

use crate::types::*;
impl Agpref<'_> {
    /// Parse the given string into an Agpref struct.
    #[deprecated]
    pub fn cfrom_str(s: &str) -> Result<Agpref, crate::errors::Errors> {
        Self::cparse(s)
    }

    #[inline(always)]
    pub fn cparse(s: &str) -> Result<Agpref, crate::errors::Errors> {
        todo!()
    }
}

fn parser<'a>() -> impl Parser<'a, &'a str, Json, extra::Err<Rich<'a, char>>> {
