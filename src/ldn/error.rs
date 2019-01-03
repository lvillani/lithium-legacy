// Lithium
// Copyright (C) 2018 Lorenzo Villani
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3 of the License.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::fmt;
use std::result;

use super::pos::{Position, Span};

/// A specialized `Result` type for LDN parser operation.
pub type Result<T> = result::Result<T, Error>;

/// The error type for LDN parsing operations.
#[derive(Debug, PartialEq)]
pub enum Error {
    IntegerLeadingZero(String, Span),
    IntegerParseError(String, Span),
    InvalidCharacter(u8, Position),
    SymbolParseError(String, Span),
    UnbalancedParentheses(Position),
    Utf8Error(Span),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IntegerLeadingZero(token, span) => write!(
                f,
                "{} found leading zero while parsing integer constant '{}'",
                span, token
            ),
            Error::IntegerParseError(token, span) => {
                write!(f, "{} cannot parse '{}' as integer", span, token)
            }
            Error::InvalidCharacter(ch, pos) => write!(f, "{} invalid character '{}'", pos, ch),
            Error::SymbolParseError(token, span) => {
                write!(f, "{} cannot parse '{}' as symbol", span, token)
            }
            Error::UnbalancedParentheses(pos) => {
                write!(f, "{} unbalanced parentheses in list`", pos)
            }
            Error::Utf8Error(s) => write!(f, "{} utf-8 decode error", s),
        }
    }
}
