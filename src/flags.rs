// Copyright Claudio Mattera 2021.
// Distributed under the MIT License.
// See accompanying file License.txt, or online at
// https://opensource.org/licenses/MIT

/// WASM-4 sprite flags
///
/// Only the two flags identifying the sprite's bit depth are defined:
/// `BLIT_1BPP` and `BLIT_2BPP`.
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Flags {
    /// Flag identifying a sprite using one bit per pixel
    OneBitPerPixel,

    /// Flag identifying a sprite using two bits per pixel
    TwoBitsPerPixel,
}

impl Flags {
    /// Return the numeric value of the flag
    pub fn value(&self) -> u32 {
        match self {
            Flags::OneBitPerPixel => 0,
            Flags::TwoBitsPerPixel => 1,
        }
    }

    /// Return the human-readable value of the flag
    ///
    /// Human-readable flag values are `BLIT_1BPP` and `BLIT_2BPP`.
    pub fn human_readable_value(&self) -> &'static str {
        match self {
            Flags::OneBitPerPixel => "BLIT_1BPP",
            Flags::TwoBitsPerPixel => "BLIT_2BPP",
        }
    }
}
