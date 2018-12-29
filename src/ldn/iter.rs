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

use super::pos::Position;

/// Wraps an iterator over bytes to add position tracking.
pub struct PositionIterator<I>
where
    I: Iterator<Item = u8>,
{
    iter: I,
    pos: Position,
}

impl<I> PositionIterator<I>
where
    I: Iterator<Item = u8>,
{
    /// Creates a new position-tracking iterator by wrapping the given `iter`.
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            pos: Position::default(),
        }
    }

    /// Returns the current position.
    pub fn pos(&self) -> &Position {
        &self.pos
    }
}

impl<I> Iterator for PositionIterator<I>
where
    I: Iterator<Item = u8>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        let ch = self.iter.next();

        match ch {
            None => (),
            Some(ch) if ch == b'\n' => {
                self.pos.line += 1;
                self.pos.column = 0;
            }
            Some(_) => self.pos.column += 1,
        }

        ch
    }
}

/// Creates a position-tracking iterator from a string reference.
impl<'a> From<&'a str> for PositionIterator<str::Bytes<'a>> {
    fn from(v: &'a str) -> Self {
        Self::new(v.bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut p = PositionIterator::from("");
        p.by_ref().last();

        assert_eq!(0, p.pos.line);
        assert_eq!(0, p.pos.column);
    }

    #[test]
    fn one() {
        let mut p = PositionIterator::from("foo");
        p.by_ref().last();

        assert_eq!(0, p.pos.line);
        assert_eq!(3, p.pos.column);
    }

    #[test]
    fn trailing_newline() {
        let mut p = PositionIterator::from("foo\n");
        p.by_ref().last();

        assert_eq!(1, p.pos.line);
        assert_eq!(0, p.pos.column);
    }

    #[test]
    fn two_lines() {
        let mut p = PositionIterator::from("foo\nbar");
        p.by_ref().last();

        assert_eq!(1, p.pos.line);
        assert_eq!(3, p.pos.column);
    }
}
