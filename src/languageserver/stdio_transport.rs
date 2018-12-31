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

use std::io;
use std::io::{BufRead, Write};

/// Reads a request from the given buffered reader.
///
/// See also: https://microsoft.github.io/language-server-protocol/specification#base-protocol
pub fn read_request<R: BufRead>(reader: &mut R) -> io::Result<String> {
    let mut content_length = 0;

    loop {
        let mut buf = String::new();
        reader.read_line(&mut buf)?;

        let line = buf.trim();
        if line.is_empty() {
            break;
        }

        if !line.contains(':') {
            return Err(io::Error::from(io::ErrorKind::InvalidData));
        }

        if !line.contains("Content-Length") {
            continue;
        }

        // FIXME(lvillani): I don't like how we are ignoring errors.
        content_length = line
            .splitn(2, ':')
            .nth(1)
            .unwrap_or("0")
            .trim()
            .parse::<usize>()
            .unwrap_or(0);
    }

    if content_length == 0 {
        return Err(io::Error::from(io::ErrorKind::InvalidData));
    }

    let mut buf = vec![0; content_length];
    reader.read_exact(&mut buf)?;

    String::from_utf8(buf).or_else(|_| Err(io::Error::from(io::ErrorKind::InvalidData)))
}

/// Writes a response to the given writer.
///
/// See also: https://microsoft.github.io/language-server-protocol/specification#base-protocol
pub fn write_response<W: Write>(writer: &mut W, data: &str) -> io::Result<()> {
    let content_length_header = format!("Content-Length: {}\r\n\r\n", data.len());

    writer.write_all(content_length_header.as_bytes())?;
    writer.write_all(data.as_bytes())?;
    writer.flush()
}
