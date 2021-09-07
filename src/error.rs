// Copyright Claudio Mattera 2021.
// Distributed under the MIT License.
// See accompanying file License.txt, or online at
// https://opensource.org/licenses/MIT

use std::fmt::Error as FmtError;
use std::io::Error as IoError;

use thiserror::Error;

use image::ImageError;

use png::DecodingError;

/// Error occurred when converting from PNG to WASM-4 Rust source code
#[derive(Error, Debug)]
pub enum PngToWasm4SrcError {
    /// An IO error occurred
    #[error("an IO error occurred")]
    IoError(#[from] IoError),

    /// Could not format text
    #[error("could not format text")]
    FmtError(#[from] FmtError),

    /// The input image is not encoded in PNG format
    #[error("image is not encoded in PNG format")]
    PngDecoding(#[from] DecodingError),

    /// The image processing failed
    ///
    /// Further information are stored in the wrapped error.
    #[error("image processing failed")]
    Image(#[from] ImageError),

    /// The input PNG image is not indexed
    #[error("PNG image is not indexed")]
    NotIndexedPng,

    /// The input indexed PNG image has a palette of invalid size
    ///
    /// WASM-4 only supports sprites with 2-colours or 4-colours palettes.
    #[error("palette has invalid size {0}")]
    InvalidPaletteSize(usize),

    /// A file does not have a stem
    ///
    /// [File stem](std::path::Path::file_stem) is the part of file name
    /// before the extension.
    #[error("file does not have a stem")]
    FileWithoutStem,

    /// A file or directory path is not valid UTF-8
    #[error("path is not valid UTF-8")]
    NonUtf8Path,
}
