// Copyright Claudio Mattera 2021.
// Distributed under the MIT License.
// See accompanying file License.txt, or online at
// https://opensource.org/licenses/MIT

use std::env::args;
use std::path::PathBuf;

use anyhow::Result;

use png2wasm4src::build_sprite_modules_tree;

fn main() -> Result<()> {
    let args = args();
    for arg in args.skip(1) {
        let path = PathBuf::from(arg);

        let module = build_sprite_modules_tree(&path)?;
        let module = module.parse()?;
        println!("{}", module);
    }

    Ok(())
}
