// Copyright Claudio Mattera 2021.
// Distributed under the MIT License.
// See accompanying file License.txt, or online at
// https://opensource.org/licenses/MIT

#![cfg_attr(not(doctest), doc = include_str!("../Readme.md"))]

mod error;
pub use error::PngToWasm4SrcError;

mod flags;
pub use flags::Flags;

mod lookup;
pub use lookup::build_sprite_modules_tree;
pub use lookup::Module;
pub use lookup::ParsedModule;

mod rust;
pub use rust::RustVariables;

mod sanitization;
use sanitization::sanitize_variable_name;

mod sprite;
pub use sprite::convert_png_to_rust_variables;
