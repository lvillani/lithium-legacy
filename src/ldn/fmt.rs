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

use super::ast::{Atom, Item};

const INDENT_LEVEL: usize = 4;

/// Formats (pretty-prints) an LDN document.
pub fn fmt(top: &[Item]) -> String {
    fmt_items(top, true, 0)
}

//
// Private
//

fn fmt_items(items: &[Item], is_top_level: bool, lhs: usize) -> String {
    // TODO(lvillani): Can we rewrite this using iterators?

    let mut prev: Option<&Item> = None;
    let mut ret = String::new();

    if !is_top_level {
        ret += "(";
    }

    for item in items {
        if let Some(prev) = prev {
            let delta = if is_top_level && !item.is_comment() && !prev.is_comment() {
                // Always insert an empty line between consecutive non-comment items at top-level.
                2
            } else {
                item.span().start.line - prev.span().start.line
            };

            if delta == 0 {
                ret += " ";
            } else if delta > 0 {
                ret += "\n";
            }

            if is_top_level && delta > 1 {
                ret += "\n"
            } else if !is_top_level && delta >= 1 {
                ret += &" ".repeat(lhs);
            }
        }

        ret += &fmt_item(item, lhs);

        prev = Some(item);
    }

    if !is_top_level {
        ret += ")";
    } else {
        ret += "\n";
    }

    ret
}

fn fmt_item(item: &Item, lhs: usize) -> String {
    match item {
        Item::Atom(atom) => fmt_atom(atom),
        Item::Comment(comment, _) => format!("; {}", comment),
        Item::List(items, _) => fmt_items(items, false, lhs + INDENT_LEVEL),
    }
}

fn fmt_atom(atom: &Atom) -> String {
    match atom {
        Atom::Integer(integer, _) => integer.to_string(),
        Atom::Keyword(keyword, _) => format!(":{}", keyword),
        Atom::String(string, _) => format!(r#""{}""#, string),
        Atom::Symbol(symbol, _) => symbol.clone(),
    }
}
