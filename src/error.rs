// This Source Code Form is subject to the terms of the Mozilla Public License,
// v. 2.0. If a copy of the MPL was not distributed with this file, You can
// obtain one at https://mozilla.org/MPL/2.0/.
use std::{convert, error,
          fmt::{self, Display},
          option};

#[derive(Copy, Clone, Debug)]
/// Errors that may be returned in the course of parsing a Goff file.
pub enum Error
{
  // Parsing
  /// Reached EOF while expecting unspecified data.
  Eof,

  // Types
  /// Expected a Boolean type, but got something else.
  ExpectedBoolean,
  /// Expected an Integer type, but got something else
  ExpectedInteger,
  /// Expected an Integer sign, but got something else
  ExpectedSign,
  /// Expected a String type, but got something else
  ExpectedString,
  /// Reached EOF while looking for the end of a string
  NoEndQuote,
  /// Expected a Real type, but got something else
  ExpectedReal,
  /// Expected a Nothing type, but got something else
  ExpectedNothing,
}

impl fmt::Display for Error
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
  {
    use Error::*;

    let err = match *self {
      Eof => "End-of-file",
      ExpectedBoolean => "Expected a boolean",
      ExpectedInteger => "Expected an integer",
      ExpectedSign => "Expected an integer sign",
      ExpectedString => "Expected a string",
      NoEndQuote => "EOF without closing string",
      ExpectedReal => "Expected a real",
      ExpectedNothing => "Expected a nothing",
    };

    write!(f, "{}", err)
  }
}

impl error::Error for Error {}

impl convert::From<option::NoneError> for Error
{
  fn from(_error: option::NoneError) -> Self
  {
    Self::Eof
  }
}

impl serde::de::Error for Error
{
  fn custom<T>(_msg: T) -> Self
    where T: Display,
  {
    unimplemented!()
  }
}
