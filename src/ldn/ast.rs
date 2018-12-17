// Lithium Platform
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

use super::pos::Span;

/// A list of items.
pub type List = Vec<Item>;

/// An item is either an atom, a coment or a list of items.
#[derive(Debug, PartialEq)]
pub enum Item {
    Atom(Atom),
    Comment(String, Span),
    List(List, Span),
}

/// An indivisible syntactic element. In other words, anything that is not a comment or a list.
#[derive(Debug, PartialEq)]
pub enum Atom {
    Integer(isize, Span),
    Keyword(String, Span),
    String(String, Span),
    Symbol(String, Span),
}
