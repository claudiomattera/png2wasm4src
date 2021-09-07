PNG to WASM-4 Source (png2wasm4src)
====

Convert indexed PNG images to Rust source code for WASM-4 engine.

<https://gitlab.com/claudiomattera/png2wasm4src>


[WASM-4] is an old-style fantasy game console implemented in WebAssembly.
Games can be developed in Rust (and in other languages), and the runtime has support for drawing sprites.
Sprites must either have a bit depth of one bit per pixel, or two bits per pixel, and must be properly encoded in variables, which can be done using the WASM-4 `w4` command-line application.

This crate allows to perform the conversion from within Rust code, which allows to dynamically create variables from PNG images using a `build.rs` [build script].


[WASM-4]: https://wasm4.org/
[build script]: https://doc.rust-lang.org/cargo/reference/build-scripts.html


Usage
----

This crate can be used to automatically generate Rust variables from PNG images on `cargo build`.
This way the Rust variables always reflect the current PNG image, and there is no risk of forgetting to update them.

Assume the following crate structure.
Directory `assets` contains a subdirectory `sprites`, which contains all the sprites.
Sprites are organized in subdirectories: sprite `letters.png` is inside directory `fonts`, and sprite `tiles.png` is inside directory `tiles`.

~~~~plain
.
├── assets
│   └── sprites
│       ├── fonts
│       │   └── letters.png
│       └── tiles
│           └── tiles.png
├── build.rs
├── Cargo.lock
├── Cargo.toml
└── src
    └── lib.rs
~~~~


Now it is possible to generate the Rust code from the sprites PNG image inside a `build.rs` build script.

~~~~rust
use std::env::var;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use anyhow::Result;

use png2wasm4src::build_sprite_modules_tree;

fn main() -> Result<()> {
    let module = build_sprite_modules_tree("assets/sprites")?;

    // Instruct cargo to re-run the build script if any source PNGs are changed
    // cargo:rerun-if-changed=assets/sprites/player.png
    // cargo:rerun-if-changed=assets/sprites/monsters/slime.png
    // cargo:rerun-if-changed=assets/sprites/monsters/bandit.png
    let mut cargo_instructions = String::default();
    module.generate_cargo_build_instructions(&mut cargo_instructions)?;
    println!("{}", cargo_instructions);

    let mut output_file = open_output_file()?;
    let module = module.parse()?;
    writeln!(output_file, "{}", module)?;

    Ok(())
}

fn open_output_file() -> Result<File> {
    let output_directory = PathBuf::from(var("OUT_DIR")?);
    let output_path = output_directory.join("sprites.rs");
    let output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output_path)?;
    Ok(output_file)
}
~~~~


The build script generates the following code, and writes it into the file `${OUT_DIR}/sprites.rs`.

~~~~rust
pub mod sprites {
    pub mod fonts {
        pub const LETTERS_WIDTH: u32 = 320;
        pub const LETTERS_HEIGHT: u32 = 32;
        pub const LETTERS_FLAGS: u32 = 1; // BLIT_2BPP
        pub const LETTERS: [u8; 200] = [0x12, 0x34, 0x56...];
    }
    pub mod tiles {
        pub const TILES_WIDTH: u32 = 32;
        pub const TILES_HEIGHT: u32 = 32;
        pub const TILES_FLAGS: u32 = 0; // BLIT_1BPP
        pub const TILES: [u8; 30] = [0x12, 0x34, 0x56...];
    }
}
~~~~


From any of the crate modules (for instance in `lib.rs`) it is possible to include that file, and use all entities defined there.

~~~~rust
use wasm::*;

// Include the generated file in the current module.
//
// Note: this is done at top level, not inside any function (but it could be
// inside a module).
include!(concat!(env!("OUT_DIR"), "/sprites.rs"));

fn draw_sprite() {
    blit(
        sprites::tiles::TILES, 
        10, 
        10, 
        sprites::tiles::TILES_WIDTH, 
        sprites::tiles::TILES_HEIGHT, 
        sprites::tiles::TILES_FLAGS,
    );
    blit(
        sprites::fonts::LETTERS, 
        10, 
        10, 
        sprites::fonts::LETTERS_WIDTH, 
        sprites::fonts::LETTERS_HEIGHT, 
        sprites::fonts::LETTERS_FLAGS,
    );
}
~~~~


License
----

Copyright Claudio Mattera 2021

You are free to copy, modify, and distribute this application with attribution under the terms of the [MIT license]. See the [`License.txt`](./License.txt) file for details.

[MIT license]: https://opensource.org/licenses/MIT
