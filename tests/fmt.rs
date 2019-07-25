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

use std::fs;
use std::path;

use pretty_assertions::assert_eq;

use lithium::ldn::{fmt, Parser};

#[test]
fn fmt_by_example() {
    let mut input_files = fs::read_dir("testdata/fmt/input")
        .unwrap()
        .map(|x| x.unwrap().path())
        .collect::<Vec<path::PathBuf>>();

    input_files.sort_unstable();

    for input_file in input_files {
        let actual = &fmt(
            &Parser::from(fs::read_to_string(&input_file).unwrap().as_ref())
                .parse()
                .unwrap(),
        );

        let expected = &fs::read_to_string(format!(
            "testdata/fmt/expected/{}",
            input_file.file_name().unwrap().to_str().unwrap()
        ))
        .unwrap();

        assert_eq!(expected, actual);
    }
}
