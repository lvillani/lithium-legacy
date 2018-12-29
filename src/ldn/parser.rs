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

use super::ast::{Atom, Item, List};
use super::error::{Error, Result};
use super::iter::PositionIterator;
use super::pos::Span;
use super::tokenizer::Tokenizer;

/// Lithium Platform Data Notation parser.
pub struct Parser<I>
where
    I: Iterator<Item = u8>,
{
    tokenizer: Tokenizer<I>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = u8>,
{
    /// Creates a new parser using the given byte iterator.
    pub fn new(iter: I) -> Self {
        Self {
            tokenizer: Tokenizer::new(PositionIterator::new(iter)),
        }
    }

    /// Parses the given byte stream.
    pub fn parse(&mut self) -> Result<List> {
        self.parse_items(true)
    }

    //
    // Private
    //

    /// Parses an item. This is essentially the same as parsing a list, the only difference is that
    /// top-level items are not enclosed by parens.
    fn parse_items(&mut self, is_top_level: bool) -> Result<List> {
        let mut ret = vec![];

        while let Some(&ch) = self.tokenizer.peek_ch() {
            match ch {
                // Whitespace and comments
                _ if Self::is_whitespace(ch) => {
                    self.tokenizer.next_ch();
                    continue;
                }
                b';' => {
                    ret.push(self.parse_comment()?);
                }
                // Integers (or symbols if the token is '-' alone).
                ch if ch == b'0' || ch == b'-' || Self::is_digit_1_9(ch) => {
                    ret.push(self.parse_integer_or_symbol()?);
                }
                // Strings
                b'"' => {
                    ret.push(self.parse_string()?);
                }
                // Keywords and symbols
                b':' => {
                    ret.push(self.parse_keyword()?);
                }
                ch if Self::is_symbol(ch) => {
                    ret.push(self.parse_symbol()?);
                }
                // Lists
                b'(' => {
                    let list_start = self.tokenizer.pos().clone();
                    self.tokenizer.next_ch();
                    ret.push(Item::List(
                        self.parse_items(false)?,
                        Span::new(list_start, self.tokenizer.pos().clone()),
                    ));
                }
                b')' => {
                    self.tokenizer.next_ch();
                    return Ok(ret);
                }
                // Catch-all error
                ch => {
                    return Err(Error::UnknownCharacter(ch, self.tokenizer.pos().clone()));
                }
            }
        }

        // If we get here we are parsing a list and we didn't encounter a closing parens.
        if !is_top_level {
            return Err(Error::UnbalancedParentheses(self.tokenizer.pos().clone()));
        }

        Ok(ret)
    }

    // Productions

    /// Parses a comment. Called by the main loop at the semicolon's position.
    fn parse_comment(&mut self) -> Result<Item> {
        let (comment, span) = self.tokenizer.take_until(|&ch| ch != b'\n')?;

        Ok(Item::Comment(
            comment.trim_start_matches(';').trim().to_string(),
            span,
        ))
    }

    /// Parses an integer. Called by the main loop at the first digit or negative sign position.
    fn parse_integer_or_symbol(&mut self) -> Result<Item> {
        let (token, span) = self.next_token()?;

        if (token.starts_with('0') && token != "0") || token.starts_with("-0") {
            return Err(Error::IntegerLeadingZero(token, span));
        } else if token == "-" {
            // "-" is a valid symbol when found by itself.
            return Ok(Item::Atom(Atom::Symbol(token, span)));
        }

        match token.parse::<isize>() {
            Ok(val) => Ok(Item::Atom(Atom::Integer(val, span))),
            Err(_) => Err(Error::IntegerParseError(token, span)),
        }
    }

    /// Parses a string. Called by the main loop at the opening quotation mark's position.
    fn parse_string(&mut self) -> Result<Item> {
        // The real starting position includes the position of the opening quotation marks.
        let string_start = self.tokenizer.pos().clone();

        // Skip opening quotation marks.
        self.tokenizer.next_ch();

        // Accumulate chunks until we encounter non-escaped quotation marks.
        let mut string = String::new();
        let mut string_last_span: Span;

        loop {
            let (chunk, span) = self.tokenizer.take_until(|&ch| ch != b'"')?;

            // Skip quotation marks.
            self.tokenizer.next_ch();

            string_last_span = span;

            if chunk.ends_with('\\') {
                string += chunk[..chunk.len() - 1].as_ref();
                string += "\"";
            } else {
                string += chunk.as_ref();
                break;
            }
        }

        Ok(Item::Atom(Atom::String(
            string,
            Span::new(string_start, string_last_span.end),
        )))
    }

    /// Parses a keyword. Called by the main loop at the colon's position. Defers to
    /// `parse_symbol()` for actual parsing.
    fn parse_keyword(&mut self) -> Result<Item> {
        // The real starting position includes the colon peeked by the main loop.
        let keyword_start = self.tokenizer.pos().clone();

        // Skip colon peeked by main loop.
        self.tokenizer.next_ch();

        match self.parse_symbol() {
            Ok(Item::Atom(Atom::Symbol(sym, span))) => Ok(Item::Atom(Atom::Keyword(
                sym,
                Span::new(keyword_start, span.end),
            ))),
            v => v,
        }
    }

    /// Parses a symbol.
    fn parse_symbol(&mut self) -> Result<Item> {
        let (token, span) = self.next_token()?;

        if !token.bytes().by_ref().all(Self::is_symbol) {
            return Err(Error::SymbolParseError(token, span));
        }

        Ok(Item::Atom(Atom::Symbol(token, span)))
    }

    // Token Helpers

    /// Returns the next token and span, by consuming bytes until the first whitespace character or
    /// closing paren.
    fn next_token(&mut self) -> Result<(String, Span)> {
        Ok(self
            .tokenizer
            .take_until(|&ch| !Self::is_whitespace(ch) && ch != b')')?)
    }

    // Recognizers

    /// Returns `true` if `ch` is considered a whitespace character according to the grammar.
    fn is_whitespace(ch: u8) -> bool {
        ch == b' ' || ch == b'\n'
    }

    /// Returns `true` if `ch` is a digit between 1 and 9.
    fn is_digit_1_9(ch: u8) -> bool {
        ch >= b'1' && ch <= b'9'
    }

    /// Returns `true` if `ch` is a symbol constituent.
    fn is_symbol(ch: u8) -> bool {
        match ch {
            b'+' | b'-' | b'*' | b'/' | b'%' | b'=' | b'<' | b'>' | b'?' | b'!' => true,
            _ => Self::is_alpha(ch),
        }
    }

    /// Returns `true` if `ch` is an alphabetic character (`a` to `z` either lowercase or
    /// uppercase).
    fn is_alpha(ch: u8) -> bool {
        ch >= b'a' && ch <= b'z' || ch >= b'A' && ch <= b'Z'
    }
}

impl<'a> From<&'a str> for Parser<str::Bytes<'a>> {
    fn from(v: &'a str) -> Self {
        Self::new(v.bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::pos::{pos, span};

    #[test]
    fn parse_empty() {
        assert!(Parser::from("").parse().unwrap().is_empty());
    }

    #[test]
    fn parse_unknown() {
        assert_eq!(
            Err(Error::UnknownCharacter(b'\r', pos(0, 1))),
            Parser::from(" \r").parse()
        );
    }

    #[test]
    fn parse_comment() {
        assert_eq!(
            vec![
                Item::Comment("foo bar".into(), span(0, 2, 0, 11)),
                Item::Comment("baz".into(), span(1, 0, 1, 5)),
                Item::Comment("qux".into(), span(2, 0, 2, 6)),
                Item::Comment("; quux ;".into(), span(3, 0, 3, 11))
            ],
            Parser::from("  ; foo bar\n; baz\n;; qux\n;; ; quux ;")
                .parse()
                .unwrap()
        )
    }

    #[test]
    fn parse_int_valid() {
        assert_eq!(
            vec![
                Item::Atom(Atom::Integer(0, span(0, 0, 0, 1))),
                Item::Atom(Atom::Integer(5, span(0, 2, 0, 3))),
                Item::Atom(Atom::Integer(-25, span(0, 4, 0, 7))),
            ],
            Parser::from("0 5 -25").parse().unwrap()
        );
    }

    #[test]
    fn parse_int_invalid() {
        assert_eq!(
            Err(Error::IntegerLeadingZero("01".into(), span(0, 0, 0, 2))),
            Parser::from("01").parse()
        );

        assert_eq!(
            Err(Error::IntegerLeadingZero("-0".into(), span(0, 0, 0, 2))),
            Parser::from("-0").parse()
        );
    }

    #[test]
    fn parse_string() {
        assert_eq!(
            vec![Item::Atom(Atom::String(
                r#"foo "bar" baz"#.into(),
                span(0, 0, 0, 16)
            ))],
            Parser::from(r#""foo \"bar\" baz""#).parse().unwrap()
        );

        assert_eq!(
            vec![Item::Atom(Atom::String(
                "foo\nbar".into(),
                span(0, 0, 1, 3)
            ))],
            Parser::from("\"foo\nbar\"").parse().unwrap()
        );
    }

    #[test]
    fn parse_keyword() {
        assert_eq!(
            vec![Item::Atom(Atom::Keyword("foobar".into(), span(0, 0, 0, 7)))],
            Parser::from(":foobar").parse().unwrap()
        );
    }

    #[test]
    fn parse_symbol_valid() {
        assert_eq!(
            vec![Item::Atom(Atom::Symbol(
                "string->int".into(),
                span(0, 0, 0, 11)
            ))],
            Parser::from("string->int").parse().unwrap()
        );

        // "-" by itself is a symbol.
        assert_eq!(
            vec![Item::Atom(Atom::Symbol("-".into(), span(0, 0, 0, 1)))],
            Parser::from("-").parse().unwrap()
        );
    }

    #[test]
    fn parse_symbol_invalid() {
        assert_eq!(
            Err(Error::SymbolParseError("a$b".into(), span(0, 0, 0, 3))),
            Parser::from("a$b").parse()
        );
    }

    #[test]
    fn parse_list_empty() {
        assert_eq!(
            vec![Item::List(vec![], span(0, 0, 0, 2))],
            Parser::from("()").parse().unwrap()
        );
    }

    #[test]
    fn parse_list_balanced() {
        assert_eq!(
            vec![Item::List(
                vec![
                    Item::Atom(Atom::Integer(1, span(0, 1, 0, 2))),
                    Item::List(
                        vec![
                            Item::Atom(Atom::Integer(2, span(0, 4, 0, 5))),
                            Item::Atom(Atom::Integer(3, span(0, 6, 0, 7))),
                        ],
                        span(0, 3, 0, 8)
                    ),
                    Item::Atom(Atom::Integer(4, span(0, 9, 0, 10))),
                    Item::List(
                        vec![Item::Atom(Atom::String("foo".into(), span(0, 12, 0, 16)))],
                        span(0, 11, 0, 18)
                    ),
                ],
                span(0, 0, 0, 19)
            )],
            Parser::from("(1 (2 3) 4 (\"foo\"))").parse().unwrap()
        );
    }

    #[test]
    fn parse_list_unbalanced() {
        assert_eq!(
            Err(Error::UnbalancedParentheses(pos(0, 8))),
            Parser::from("(1 (2) 3").parse()
        );
    }
}
