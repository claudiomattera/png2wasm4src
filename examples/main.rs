// Copyright Claudio Mattera 2021.
// Distributed under the MIT License.
// See accompanying file License.txt, or online at
// https://opensource.org/licenses/MIT

use std::env::args;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use anyhow::Result;

use png2wasm4src::convert_png_to_rust_variables;

fn main() -> Result<()> {
    let args = args();
    for arg in args.skip(1) {
        let path = PathBuf::from(arg);

        let mut file = File::open(&path)?;
        let mut bytes = Vec::default();
        file.read_to_end(&mut bytes)?;

        let name = path
            .file_stem()
            .expect("Not a file")
            .to_str()
            .expect("Not an UTF-8 file name");

        let rust_code = convert_png_to_rust_variables(name, &bytes)?;

        println!("{}", rust_code);
    }

    Ok(())
}
