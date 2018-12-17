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

use std::fmt;

/// Position in a text document expressed as zero-based line and column offsets.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    /// Creates a new position with the given zero-based line and column offsets.
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Most editors interpret offsets printed to stdout/stderr as one-based.
        write!(f, "{}:{}", self.line + 1, self.column + 1)
    }
}

/// Convenience function to create a position from raw offsets.
#[inline]
pub fn pos(line: usize, column: usize) -> Position {
    Position::new(line, column)
}

/// A span in a text document expressed as zero-based start and end positions.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    /// Creates a new span from the given zero-based start and end positions.
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    /// Creates a new span from raw start/end lines and columns. This is mainly used in unit tests.
    pub fn from_parts(
        start_line: usize,
        start_column: usize,
        end_line: usize,
        end_column: usize,
    ) -> Self {
        Self {
            start: Position::new(start_line, start_column),
            end: Position::new(end_line, end_column),
        }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // This results in "a:b:c:d" since we piggy-back on Position's Display impl.
        write!(f, "{}:{}", self.start, self.end)
    }
}

/// Convenience function to create a `Span` from raw offsets.
#[inline]
pub fn span(start_line: usize, start_column: usize, end_line: usize, end_column: usize) -> Span {
    Span::from_parts(start_line, start_column, end_line, end_column)
}
