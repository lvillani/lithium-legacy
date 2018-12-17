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

use std::fs;
use std::path;

use structopt::StructOpt;

use lithium::ldn;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    let buf = fs::read_to_string(args.path).unwrap();
    let ast = ldn::Parser::from(buf.as_str()).parse().unwrap();

    println!("{}", ldn::print(&ast));
}
