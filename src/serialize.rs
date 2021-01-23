// This Source Code Form is subject to the terms of the Mozilla Public License,
// v. 2.0. If a copy of the MPL was not distributed with this file, You can
// obtain one at https://mozilla.org/MPL/2.0/.
use std::ops::{AddAssign, MulAssign, Neg};

use serde::de::{self, Visitor};

use super::Error;

/// Simplified Result type for functions returning Goff errors.
type Result<T> = std::result::Result<T, Error>;

/// Main deserialiser
pub struct Deserializer<'de>
{
  /// Begins as a copy of the given Goff data, but is modified by parse_* etc.
  /// functions throughout parsing.
  input: &'de str,
}

impl<'de> Deserializer<'de>
{
  // Public
  /// Turns a string into a `Deserializer`.
  pub const fn from_str(input: &'de str) -> Self
  {
    Deserializer { input }
  }

  // Parsing
  /// Look at the next char in the sequence without consuming it.
  fn peek_char(&mut self) -> Result<char>
  {
    self.input.chars().next().ok_or(Error::Eof)
  }

  /// Look at the next char in the sequence and advance parsing.
  fn next_char(&mut self) -> Result<char>
  {
    let ch = self.peek_char()?;
    self.input = self.input.get(ch.len_utf8()..)?;
    Ok(ch)
  }

  // Goff types
  /// Parses a Boolean
  fn parse_bool(&mut self) -> Result<bool>
  {
    if self.input.starts_with("yes") {
      self.input = self.input.get("yes".len()..)?;
      Ok(true)
    } else if self.input.starts_with("no") {
      self.input = self.input.get("no".len()..)?;
      Ok(false)
    } else {
      Err(Error::ExpectedBoolean)
    }
  }

  /// Parses a (Positive) Integer
  fn parse_unsigned<T>(&mut self) -> Result<T>
    where T: AddAssign<T> + MulAssign<T> + From<u8>,
  {
    let mut int = match self.next_char()? {
      ch @ '0'..='9' => T::from(ch as u8 - b'0'),
      _ => {
        return Err(Error::ExpectedInteger);
      },
    };

    loop {
      match self.input.chars().next() {
        Some(ch @ '0'..='9') => {
          self.input = self.input.get(1..)?;
          int *= T::from(10);
          int += T::from(ch as u8 - b'0');
        },
        _ => {
          return Ok(int);
        },
      }
    }
  }

  /// Parses an integer that may contain a sign (-).
  /// + is an invalid signal because it is always redunant.
  /// integers without signs are allowed, where this function will act the same
  /// as [`parse_unsigned`](fn.parse_unsigned.html)
  fn parse_signed<T>(&mut self) -> Result<T>
    where T: Neg<Output = T> + AddAssign<T> + MulAssign<T> + From<i8>,
  {
    let is_negative = self.peek_char()? == '-';

    if is_negative {
      self.next_char()?;
    }

    let mut int = match self.next_char()? {
      ch @ '0'..='9' => T::from((ch as u8 - b'0') as i8),
      _ => {
        return Err(Error::ExpectedInteger);
      },
    };

    loop {
      if let Some(ch @ '0'..='9') = self.input.chars().next() {
        self.input = self.input.get(1..)?;
        int *= T::from(10);
        int += T::from((ch as u8 - b'0') as i8);
      } else {
        if is_negative {
          return Ok(-int);
        }
        return Ok(int);
      }
    }
  }

  // String
  // TODO: Not this, is format! slow?
  /// Parses a string. Considers escape sequences.
  fn parse_string(&mut self) -> Result<String>
  {
    if self.next_char()? != '\'' {
      return Err(Error::ExpectedString);
    }

    let mut res = String::new();
    while let Ok(next_char) = self.next_char() {
      match next_char {
        '\\' => res = format!("{}{}", res, self.parse_escape()?),
        '\'' => return Ok(res),
        ch => res = format!("{}{}", res, ch),
      };
    }
    Err(Error::NoEndQuote)
  }

  /// TODO: PIA. Is there a lib for this or something
  // fn parse_real<T>(&mut self) -> Result<T>
  // where T: AddAssign<T> + MulAssign<T> + Div<T> + DivAssign<T> + From<u8>,
  // {
  // let mut real = match self.next_char()? {
  // ch @ '0'..='9' => T::from(ch as u8 - b'0'),
  // _ => { return Err(Error::ExpectedReal); }
  // };
  //
  // let points_past: u8;
  //
  // loop {
  // if points_past != 0 {
  // points_past += 1;
  // }
  //
  // match self.input.chars().next() {
  // Some(ch @ '0'..='9') => {
  // self.input = &self.input[1..];
  // if points_past == 0 {
  // real *= T::from(10);
  // real += T::from(ch as u8 - b'0');
  // } else {
  // real += T::from(ch as u8 - b'0') / T::from(points_past);
  // }
  // },
  // Some('.') => points_past += 1,
  // _ => { return Ok(real); }
  // }
  // }
  // }

  // Goff non-types
  /// Parses valid escape characters. `\n`, `\r`, `\t`, `\\`, and `\'`
  fn parse_escape(&mut self) -> Result<char>
  {
    match self.next_char()? {
      'n' => Ok('\n'),
      'r' => Ok('\r'),
      't' => Ok('\t'),
      '\\' => Ok('\\'),
      '\'' => Ok('\''),
      c => Ok(c),
    }
  }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de>
{
  type Error = Error;

  #[inline]
  fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where V: Visitor<'de>,
  {
    unimplemented!()
  }

  #[inline]
  fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where V: Visitor<'de>,
  {
    visitor.visit_bool(self.parse_bool()?)
  }

  #[inline]
  fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where V: Visitor<'de>,
  {
    visitor.visit_i8(self.parse_signed()?)
  }

  #[inline]
  fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where V: Visitor<'de>,
  {
    visitor.visit_i16(self.parse_signed()?)
  }

  #[inline]
  fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where V: Visitor<'de>,
  {
    visitor.visit_i32(self.parse_signed()?)
  }

  #[inline]
  fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where V: Visitor<'de>,
  {
    visitor.visit_i64(self.parse_signed()?)
  }

  #[inline]
  fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where V: Visitor<'de>,
  {
    visitor.visit_u8(self.parse_unsigned()?)
  }

  #[inline]
  fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where V: Visitor<'de>,
  {
    visitor.visit_u16(self.parse_unsigned()?)
  }

  #[inline]
  fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where V: Visitor<'de>,
  {
    visitor.visit_u32(self.parse_unsigned()?)
  }

  #[inline]
  fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where V: Visitor<'de>,
  {
    visitor.visit_u64(self.parse_unsigned()?)
  }

  #[inline]
  fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where V: Visitor<'de>,
  {
    unimplemented!()
  }

  #[inline]
  fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where V: Visitor<'de>,
  {
    unimplemented!()
  }

  #[inline]
  fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where V: Visitor<'de>,
  {
    // visitor.visit_borrowed_str(self.parse_string()?)
    visitor.visit_string(self.parse_string()?)
  }

  #[inline]
  fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where V: Visitor<'de>,
  {
    visitor.visit_string(self.parse_string()?)
  }

  #[inline]
  fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where V: Visitor<'de>,
  {
    self.deserialize_str(visitor)
  }

  #[inline]
  fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where V: Visitor<'de>,
  {
    unimplemented!()
  }

  #[inline]
  fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where V: Visitor<'de>,
  {
    unimplemented!()
  }

  #[inline]
  fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where V: Visitor<'de>,
  {
    unimplemented!()
  }

  #[inline]
  fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where V: Visitor<'de>,
  {
    unimplemented!()
  }

  #[inline]
  fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where V: Visitor<'de>,
  {
    self.deserialize_unit(visitor)
  }

  #[inline]
  fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where V: Visitor<'de>,
  {
    visitor.visit_newtype_struct(self)
  }

  // (Lists)
  #[inline]
  fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value>
    where V: Visitor<'de>,
  {
    unimplemented!();
  }

  #[inline]
  fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where V: Visitor<'de>,
  {
    unimplemented!()
  }

  #[inline]
  fn deserialize_tuple_struct<V>(self,
                                 _name: &'static str,
                                 _len: usize,
                                 _visitor: V)
                                 -> Result<V::Value>
    where V: Visitor<'de>,
  {
    unimplemented!()
  }

  #[inline]
  fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where V: Visitor<'de>,
  {
    unimplemented!()
  }

  #[inline]
  fn deserialize_struct<V>(self,
                           _name: &'static str,
                           _fields: &'static [&'static str],
                           _visitor: V)
                           -> Result<V::Value>
    where V: Visitor<'de>,
  {
    unimplemented!()
    // if self.next_char()? == '('
  }

  #[inline]
  fn deserialize_enum<V>(self,
                         _name: &'static str,
                         _variants: &'static [&'static str],
                         _visitor: V)
                         -> Result<V::Value>
    where V: Visitor<'de>,
  {
    unimplemented!()
  }

  // (Keys)
  #[inline]
  fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where V: Visitor<'de>,
  {
    self.deserialize_str(visitor)
  }

  #[inline]
  fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where V: Visitor<'de>,
  {
    self.deserialize_any(visitor)
  }
}
