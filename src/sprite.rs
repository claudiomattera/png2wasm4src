// Copyright Claudio Mattera 2021.
// Distributed under the MIT License.
// See accompanying file License.txt, or online at
// https://opensource.org/licenses/MIT

#![cfg_attr(not(doctest), doc = include_str!("../Readme.md"))]

use std::collections::HashMap;
use std::io::Cursor;

use image::io::Reader as ImageReader;
use image::{ImageFormat, Rgba, RgbaImage};

use png::Decoder as PngDecoder;

use crate::{Flags, PngToWasm4SrcError, RustVariables};

/// Convert a PNG image to a struct representing Rust source code
///
/// Parameters
/// ----
///
/// * `name` the variables prefix
/// * `bytes` the raw PNG image
///
///
/// Generating Rust source code
/// ----
///
/// The group can be converted to actual Rust source code using the function
/// [`std::string::ToString::to_string()`], or the macro [`format!`].
/// The result is the same as the WASM-4 command `w4 png2src --rust path/to/image`.
///
/// ```
/// # use png2wasm4src::{Flags, RustVariables};
/// let variables = RustVariables::new(
///     "name", 16, 24, Flags::OneBitPerPixel,
///     vec![0x00, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0xff],
/// );
///
/// assert_eq!(
///     format!("{}", variables),
///     "const NAME_WIDTH: u32 = 16;
/// const NAME_HEIGHT: u32 = 24;
/// const NAME_FLAGS: u32 = 0; // BLIT_1BPP
/// const NAME: [u8; 10] = [0x00, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0xff];\n",
/// );
/// ```
pub fn convert_png_to_rust_variables(
    name: &str,
    bytes: &[u8],
) -> Result<RustVariables, PngToWasm4SrcError> {
    let palette = extract_palette(bytes)?;
    let palette = compute_palette_mapping(&palette);

    let image = read_image(bytes)?;

    let (data, flags) = match palette.len() {
        2 => (encode_1bpp_image(&image, &palette), Flags::OneBitPerPixel),
        4 => (encode_2bpp_image(&image, &palette), Flags::TwoBitsPerPixel),
        n => return Err(PngToWasm4SrcError::InvalidPaletteSize(n)),
    };

    let rust_variables = RustVariables::new(name, image.width(), image.height(), flags, data);

    Ok(rust_variables)
}

fn extract_palette(bytes: &[u8]) -> Result<Vec<u32>, PngToWasm4SrcError> {
    let decoder = PngDecoder::new(bytes);
    let reader = decoder.read_info()?;
    let info = reader.info();

    info.palette
        .as_ref()
        .ok_or(PngToWasm4SrcError::NotIndexedPng)
        .map(|palette| {
            let palette_size = palette.len() / 3;
            let palette: Vec<_> = (0..palette_size)
                .map(|i| {
                    let r = palette[i * 3];
                    let g = palette[i * 3 + 1];
                    let b = palette[i * 3 + 2];
                    let a = 0;
                    quadruple_to_value(r, g, b, a)
                })
                .collect();
            palette
        })
}

fn compute_palette_mapping(palette: &[u32]) -> HashMap<u32, usize> {
    palette
        .iter()
        .enumerate()
        .map(|(index, value)| (*value, index))
        .collect()
}

fn read_image(bytes: &[u8]) -> Result<RgbaImage, PngToWasm4SrcError> {
    let mut reader = ImageReader::new(Cursor::new(bytes));
    reader.set_format(ImageFormat::Png);
    let image = reader.decode()?.into_rgba8();
    Ok(image)
}

fn encode_1bpp_image(image: &RgbaImage, palette: &HashMap<u32, usize>) -> Vec<u8> {
    let encoder = |x, y| {
        let idx = ((y * image.width() + x) as usize) >> 3;
        let shift = 7 - ((x as u8) & 0x07);
        let mask = 0x1 << shift;
        (idx, shift, mask)
    };
    encode_image(image, palette, encoder)
}

fn encode_2bpp_image(image: &RgbaImage, palette: &HashMap<u32, usize>) -> Vec<u8> {
    let encoder = |x, y| {
        let idx = ((y * image.width() + x) as usize) >> 2;
        let shift = 6 - (((x as u8) & 0x3) << 1);
        let mask = 0x3 << shift;
        (idx, shift, mask)
    };
    encode_image(image, palette, encoder)
}

fn encode_image<F>(image: &RgbaImage, palette: &HashMap<u32, usize>, encode: F) -> Vec<u8>
where
    F: Fn(u32, u32) -> (usize, u8, u8),
{
    let mut bytes = Vec::default();

    for (x, y, color) in image.enumerate_pixels() {
        let value = color_to_value(color);
        let index = palette.get(&value).expect("Missing pixel value in mapping");
        let (idx, shift, mask) = encode(x, y);
        if bytes.len() <= idx {
            bytes.push(0);
        }
        bytes[idx] = ((*index as u8) << shift) | (bytes[idx] & (!mask));
    }

    bytes
}

fn color_to_value(color: &Rgba<u8>) -> u32 {
    let Rgba([r, g, b, a]) = color;
    quadruple_to_value(*r, *g, *b, *a)
}

fn quadruple_to_value(r: u8, g: u8, b: u8, _a: u8) -> u32 {
    let value = (r as u32) << 24 | (g as u32) << 16 | (b as u32) << 8;
    value as u32
}
