// This Source Code Form is subject to the terms of the Mozilla Public License,
// v. 2.0. If a copy of the MPL was not distributed with this file, You can
// obtain one at https://mozilla.org/MPL/2.0/.

// TODO: Async?

//! Goff reference implementation.
#![feature(try_trait)]
#![warn(clippy::all,
        clippy::restriction,
        clippy::pedantic,
        clippy::cargo,
        clippy::nursery)]
#![allow(clippy::blanket_clippy_restriction_lints,
         clippy::enum_glob_use,
         clippy::implicit_return,
         clippy::cargo_common_metadata,   // Blame Serde
         clippy::multiple_crate_versions, // Blame Serde
         clippy::wildcard_dependencies,   // Based, but blame Serde.
         clippy::integer_arithmetic,
         clippy::wildcard_enum_match_arm, // Might be good to avoid if possible
         clippy::as_conversions,          // Should be removed asap?
         clippy::panic_in_result_fn,      // Should be removed asap
         clippy::cast_possible_wrap,      // Should be considered but not removed.
                                          // Casts are only used in a safe context.
         clippy::unimplemented )] // Should be removed asap
use serde::Deserialize;

/// Contains custom error types and implementations.
mod error;
/// Contains the actual deserialiser/parser implementation.
mod serialize;

use error::Error;

#[inline]
/// Deserialises the given Goff data as a str and returns it, deserialised to
/// a given type.
/// # Errors
/// `from_str` may return almost any [`Error`](errors/enum.Error.html)
pub fn from_str<'d, T>(inp: &'d str) -> Result<T, Error>
  where T: Deserialize<'d>,
{
  let mut deserializer = serialize::Deserializer::from_str(inp);
  let t = T::deserialize(&mut deserializer)?;
  Ok(t)
}

// Tests
#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests
{
  use serde::Deserialize;

  #[derive(Debug, Clone, PartialEq, Deserialize)]
  struct Simple
  {
    string:    String,
    int:       u8,
    neg_int:   i8,
    r#bool:    bool,
    newline:   String,
    escapes:   String,
    multiline: String,
  }

  #[test]
  fn simple()
  {
    let simple_str = "
string    = 'foobar'
int       = 1
neg-int   = -1
bool      = no
newline   = 'foo\\nbar'
escapes   = 'foo\\\'bar'
multiline = '
foo
bar'
    ";

    assert_eq!(Simple { string:    "foobar".to_owned(),
                        int:       1,
                        neg_int:   -1,
                        bool:      false,
                        newline:   "foo\nbar".to_owned(),
                        escapes:   "foo\\\'bar".to_owned(),
                        multiline: "foo\nbar".to_owned(), },
               super::from_str(simple_str).unwrap(),);
  }
}
