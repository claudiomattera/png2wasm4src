// Copyright Claudio Mattera 2021.
// Distributed under the MIT License.
// See accompanying file License.txt, or online at
// https://opensource.org/licenses/MIT

use std::fmt;

use crate::{sanitize_variable_name, Flags};

/// A group of Rust variables defining a WASM-4 sprite
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
///
/// ### Alternate Form
///
/// Passing the `#` flag to the `format!` macro will typeset the image data in
/// binary format, instead of hexadecimal format.
///
/// ```
/// # use png2wasm4src::{Flags, RustVariables};
/// let variables = RustVariables::new(
///     "name", 16, 24, Flags::OneBitPerPixel,
///     vec![0x00, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0xff],
/// );
///
/// assert_eq!(
///     format!("{:#}", variables),
///     "const NAME_WIDTH: u32 = 16;
/// const NAME_HEIGHT: u32 = 24;
/// const NAME_FLAGS: u32 = 0; // BLIT_1BPP
/// const NAME: [u8; 10] = [0b00000000, 0b00000001, 0b00000010, 0b00000100, 0b00001000, 0b00010000, 0b00100000, 0b01000000, 0b10000000, 0b11111111];\n",
/// );
/// ```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RustVariables {
    name: String,
    width: u32,
    height: u32,
    flags: Flags,
    data: Vec<u8>,
}

impl RustVariables {
    /// Create an instance
    pub fn new(
        name: impl Into<String>,
        width: u32,
        height: u32,
        flags: Flags,
        data: Vec<u8>,
    ) -> Self {
        Self {
            name: name.into(),
            width,
            height,
            flags,
            data,
        }
    }

    /// Return the variables prefix
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Return the sprite width
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Return the sprite height
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Return the sprite flags (only for bit depth)
    pub fn flags(&self) -> Flags {
        self.flags
    }

    /// Return the sprite encoded data
    pub fn data(&self) -> &[u8] {
        self.data.as_ref()
    }
}

impl fmt::Display for RustVariables {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = sanitize_variable_name(&self.name);
        writeln!(f, "const {}_WIDTH: u32 = {};", name, self.width)?;
        writeln!(f, "const {}_HEIGHT: u32 = {};", name, self.height)?;
        write!(f, "const {}_FLAGS: u32 = {};", name, self.flags.value())?;
        writeln!(f, " // {}", self.flags.human_readable_value())?;
        write!(f, "const {}: [u8; {}] = [", name, self.data.len())?;
        if let Some(byte) = self.data.first() {
            if f.alternate() {
                write!(f, "{:#010b}", byte)?;
            } else {
                write!(f, "{:#04x}", byte)?;
            }
        }
        for byte in self.data.iter().skip(1) {
            if f.alternate() {
                write!(f, ", {:#010b}", byte)?;
            } else {
                write!(f, ", {:#04x}", byte)?;
            }
        }
        writeln!(f, "];")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::Flags;

    #[test]
    fn to_string() {
        let rust_variables = RustVariables::new(
            "some_name",
            10,
            12,
            Flags::OneBitPerPixel,
            vec![0x01, 0x02, 0x04, 0x1f],
        );
        let rust_code = rust_variables.to_string();

        let expected = "const SOME_NAME_WIDTH: u32 = 10;
const SOME_NAME_HEIGHT: u32 = 12;
const SOME_NAME_FLAGS: u32 = 0; // BLIT_1BPP
const SOME_NAME: [u8; 4] = [0x01, 0x02, 0x04, 0x1f];\n";

        assert_eq!(rust_code, expected);
    }

    #[test]
    fn to_string_two_bits_per_pixel() {
        let rust_variables = RustVariables::new(
            "some_name",
            10,
            12,
            Flags::TwoBitsPerPixel,
            vec![0x01, 0x02, 0x04, 0x1f],
        );
        let rust_code = rust_variables.to_string();

        let expected = "const SOME_NAME_WIDTH: u32 = 10;
const SOME_NAME_HEIGHT: u32 = 12;
const SOME_NAME_FLAGS: u32 = 1; // BLIT_2BPP
const SOME_NAME: [u8; 4] = [0x01, 0x02, 0x04, 0x1f];\n";

        assert_eq!(rust_code, expected);
    }

    #[test]
    fn format() {
        let rust_variables = RustVariables::new(
            "some_name",
            10,
            12,
            Flags::OneBitPerPixel,
            vec![0x01, 0x02, 0x04, 0x1f],
        );
        let rust_code = format!("{}", rust_variables);

        let expected = "const SOME_NAME_WIDTH: u32 = 10;
const SOME_NAME_HEIGHT: u32 = 12;
const SOME_NAME_FLAGS: u32 = 0; // BLIT_1BPP
const SOME_NAME: [u8; 4] = [0x01, 0x02, 0x04, 0x1f];\n";

        assert_eq!(rust_code, expected);
    }

    #[test]
    fn format_alternate() {
        let rust_variables = RustVariables::new(
            "some_name",
            10,
            12,
            Flags::OneBitPerPixel,
            vec![0x01, 0x02, 0x04, 0x1f],
        );
        let rust_code = format!("{:#}", rust_variables);

        let expected = "const SOME_NAME_WIDTH: u32 = 10;
const SOME_NAME_HEIGHT: u32 = 12;
const SOME_NAME_FLAGS: u32 = 0; // BLIT_1BPP
const SOME_NAME: [u8; 4] = [0b00000001, 0b00000010, 0b00000100, 0b00011111];\n";

        assert_eq!(rust_code, expected);
    }
}
