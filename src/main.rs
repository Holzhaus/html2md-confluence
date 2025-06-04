// Copyright (c) 2025 Jan Holthuis <jan.holthuis@rub.de>
//
// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with this program. If
// not, see <https://www.gnu.org/licenses/>.
//
// SPDX-License-Identifier: GPL-3.0-or-later

use html2md_confluence::{ParseOptions, parse_confluence};
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();

    stdin.read_to_string(&mut buffer)?;

    let options = ParseOptions::default();
    print!("{}", parse_confluence(&buffer, &options));

    Ok(())
}
