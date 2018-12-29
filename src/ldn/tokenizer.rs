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

use std::str;

use super::error::{Error, Result};
use super::iter::PositionIterator;
use super::pos::{Position, Span};

/// Wraps a position-tracking iterator to provide facilities for tokenizing the underlying text
/// stream.
///
/// It acts like a Peekable, in that it provides a `peek_ch()` method that can be called exactly
/// once, but doesn't provide an implementation of the `Iterator` and `Peekable` traits/structs.
pub struct Tokenizer<I>
where
    I: Iterator<Item = u8>,
{
    iter: PositionIterator<I>,
    peeked_pos: Position,

    #[cfg_attr(feature = "cargo-clippy", allow(clippy::option_option))]
    peeked: Option<Option<I::Item>>,
}

impl<I> Tokenizer<I>
where
    I: Iterator<Item = u8>,
{
    /// Creates a new tokenizer wrapping the given `iter`.
    pub fn new(iter: PositionIterator<I>) -> Self {
        Self {
            iter,
            peeked_pos: Position::default(),
            peeked: None,
        }
    }

    /// Returns a token from bytes read from the underlying iterator until the predicate returns
    /// false.
    ///
    /// This method internally uses `peek_ch()`.
    pub fn take_until<F>(&mut self, predicate: F) -> Result<(String, Span)>
    where
        F: Fn(&u8) -> bool,
    {
        let mut ret: Vec<u8> = Vec::new();

        let start = self.pos().clone();

        while let Some(ch) = self.peek_ch() {
            if !predicate(ch) {
                break;
            }

            ret.push(*ch);

            self.next_ch();
        }

        let end = self.pos().clone();

        let span = Span::new(start, end);

        match String::from_utf8(ret) {
            Ok(s) => Ok((s, span)),
            Err(_) => Err(Error::Utf8Error(span)),
        }
    }

    /// Peeks a character from the iterator, without advancing the current position. Always returns
    /// the last peeked character until someone advances the underlying iterator by calling
    /// `next_ch()`.
    pub fn peek_ch(&mut self) -> Option<&I::Item> {
        if self.peeked.is_none() {
            self.peeked_pos = self.iter.pos().clone();
            self.peeked = Some(self.next_ch());
        }

        match self.peeked {
            Some(Some(ref v)) => Some(v),
            Some(None) => None,
            _ => unreachable!(),
        }
    }

    /// Returns the next character from the underlying iterator. If `peek_ch()` was called, first
    /// consumes the peeked byte.
    pub fn next_ch(&mut self) -> Option<I::Item> {
        match self.peeked.take() {
            Some(v) => v,
            None => self.iter.next(),
        }
    }

    /// Returns the current position, taking peeking into account.
    pub fn pos(&self) -> &Position {
        match self.peeked {
            None => &self.iter.pos(),
            Some(_) => &self.peeked_pos,
        }
    }
}

/// Creates a tokenizer from a string reference.
impl<'a> From<&'a str> for Tokenizer<str::Bytes<'a>> {
    fn from(v: &'a str) -> Self {
        Self::new(PositionIterator::from(v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::pos::{pos, span};

    #[test]
    fn take_until_empty() {
        let mut t = Tokenizer::from("");
        let s = t.take_until(|_| true).unwrap();

        assert_eq!(("".to_string(), Span::default()), s);
        assert_eq!(&Position::default(), t.pos());
    }

    #[test]
    fn take_until_everything() {
        let mut t = Tokenizer::from("foo bar");
        let s = t.take_until(|_| true).unwrap();

        assert_eq!(("foo bar".to_string(), span(0, 0, 0, 7)), s);
        assert_eq!(&pos(0, 7), t.pos());
    }

    #[test]
    fn take_until_nothing() {
        let mut t = Tokenizer::from("foo bar");
        let s = t.take_until(|_| false).unwrap();

        assert_eq!(("".to_string(), Span::default()), s);
        assert_eq!(&Position::default(), t.pos());
    }

    #[test]
    fn take_until() {
        let mut t = Tokenizer::from("foo bar");

        let s1 = t.take_until(|ch| *ch != b' ').unwrap();
        assert_eq!(("foo".to_string(), span(0, 0, 0, 3)), s1);
        assert_eq!(&pos(0, 3), t.pos());

        assert_eq!(Some(b' '), t.next_ch());
        assert_eq!(&pos(0, 4), t.pos());

        let s2 = t.take_until(|ch| *ch != b' ').unwrap();
        assert_eq!(("bar".to_string(), span(0, 4, 0, 7)), s2);
        assert_eq!(&pos(0, 7), t.pos());

        assert_eq!(None, t.next_ch());
        assert_eq!(&pos(0, 7), t.pos());
    }

    #[test]
    fn peek_ch_empty() {
        let mut t = Tokenizer::from("");

        assert_eq!(None, t.peek_ch());
        assert_eq!(&Position::default(), t.pos());
        assert_eq!(None, t.next_ch());
        assert_eq!(&Position::default(), t.pos());
    }

    #[test]
    fn peek_ch() {
        let mut t = Tokenizer::from("foo");

        assert_eq!(Some(&b'f'), t.peek_ch());
        assert_eq!(&Position::default(), t.pos());
        // Repeated peeks always return the previous peek and pos
        assert_eq!(Some(&b'f'), t.peek_ch());
        assert_eq!(&Position::default(), t.pos());
        // Advance
        assert_eq!(Some(b'f'), t.next_ch());
        assert_eq!(Some(b'o'), t.next_ch());
        assert_eq!(&pos(0, 2), t.pos());
        // Peek again
        assert_eq!(Some(&b'o'), t.peek_ch());
        assert_eq!(&pos(0, 2), t.pos());
        // Advance again
        assert_eq!(Some(b'o'), t.next_ch());
        assert_eq!(&pos(0, 3), t.pos());
        // End
        assert_eq!(None, t.next_ch());
        assert_eq!(&pos(0, 3), t.pos());
    }

    #[test]
    fn next_ch_empty() {
        let mut t = Tokenizer::from("");

        assert_eq!(None, t.next_ch());
        assert_eq!(&Position::default(), t.pos());
    }

    #[test]
    fn next_ch() {
        let mut t = Tokenizer::from("foo");

        assert_eq!(Some(b'f'), t.next_ch());
        assert_eq!(Some(b'o'), t.next_ch());
        assert_eq!(Some(b'o'), t.next_ch());
        assert_eq!(None, t.next_ch());
        assert_eq!(&pos(0, 3), t.pos());
    }
}
