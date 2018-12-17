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

// TODO(lvillani): The pretty-printer is incomplete.

/// Pretty-prints an LDN document.
pub fn print(top: &[Item]) -> String {
    print_items(top, true)
}

fn print_items(items: &[Item], is_top_level: bool) -> String {
    if is_top_level {
        items
            .iter()
            .map(print_item)
            .collect::<Vec<String>>()
            .join("\n\n")
    } else {
        format!(
            "({})",
            items
                .iter()
                .map(print_item)
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

fn print_item(item: &Item) -> String {
    match item {
        Item::Atom(atom) => print_atom(atom),
        Item::Comment(comment, _) => format!("{}\n", comment),
        Item::List(items, _) => print_items(items, false),
    }
}

fn print_atom(atom: &Atom) -> String {
    match atom {
        Atom::Integer(integer, _) => integer.to_string(),
        Atom::Keyword(keyword, _) => format!(":{}", keyword),
        Atom::String(string, _) => format!(r#""{}""#, string),
        Atom::Symbol(symbol, _) => symbol.clone(),
    }
}
