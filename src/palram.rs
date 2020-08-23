//! Palette RAM (`PALRAM`).
//!
//! This holds RGB palette entries. Each entry is 16-bit, 5-bits per channel,
//! and the highest bit is ignored. The bits are laid out as follows:
//!
//! ```
//! 0bXBBBBBGG_GGGRRRRR
//! ```
//!
//! There are two palettes, one for backgrounds and one for objects.
//!
//! The palettes can be used in 4bpp mode or 8bpp mode:
//! * In 4bpp mode, the palette consists of 16 "palbanks", and then each palbank
//!   has 16 entries. Entry 0 of each palbank is considered transparent, so
//!   there's 15 usable entries per palbank.
//! * In 8bpp mode, the palette consists of a single 256 entry block. Again,
//!   entry 0 is considered to be a transparent entry, so there's 255 usable
//!   entries.
//! * Each individual background or object decides if it wants to use the
//!   palettes in 4bpp mode of 8bpp mode. You can freely intermix modes, the
//!   overall memory is the same either way.
//!
//! Entry 0 of the background palette is the "backdrop" color. If no background
//! or object would end up drawing to a particular pixel, the backdrop color is
//! used for that pixel.
//!
//! You **cannot** validly write a single byte to `PALRAM`. If you do so, the
//! 8-bits are written to both the upper and lower 8-bits of the given 16-bit
//! segment of memory. This isn't a big deal, since all palette entries are
//! 16-bit values anyway.
//!
//! * **Size:** 1kb
//! * **Wait states:** 0
//! * **Bus Size:** 16-bit
//! * **Read/Write:** 16/32

use super::*;

/// Base Address of the background palette.
pub const BG_PALETTE_RAM_ADDR: usize = 0x0500_0000;

/// Palette entries are 2 bytes each.
pub const BG_PALETTE_RAM_ENTRY_SIZE: usize = 2;

/// There's 256 slots in the background palette.
pub const BG_PALETTE_RAM_COUNT: usize = 256;

/// The address of the backdrop color.
pub const BACKDROP_COLOR_ADDR: usize = 0x0500_0000;

/// Index the background palette to a 4bpp `palbank` and `index`.
///
/// ## Panics
/// Both `p` and `i` must be < 16.
pub const fn index_bg_palette_4bpp(p: usize, i: usize) -> usize {
  let checked_p = const_bound_check(p, 16);
  let checked_i = const_bound_check(i, 16);
  let checked_index = (checked_p << 4) + checked_i;
  BG_PALETTE_RAM_ADDR + (BG_PALETTE_RAM_ENTRY_SIZE * checked_index)
}

/// Index the background palette to a 8bpp `index`.
///
/// ## Panics
/// `i` must be < 16.
pub const fn index_bg_palette_8bpp(i: usize) -> usize {
  let checked_index = const_bound_check(i, BG_PALETTE_RAM_COUNT);
  BG_PALETTE_RAM_ADDR + (BG_PALETTE_RAM_ENTRY_SIZE * checked_index)
}

/// Base Address of the object palette.
pub const OBJ_PALETTE_RAM_ADDR: usize = 0x0500_0200;

/// Palette entries are 2 bytes each.
pub const OBJ_PALETTE_RAM_ENTRY_SIZE: usize = 2;

/// There's 256 slots in the object palette.
pub const OBJ_PALETTE_RAM_COUNT: usize = 256;

/// Index the object palette to a 4bpp `palbank` and `index`.
///
/// ## Panics
/// Both `p` and `i` must be < 16.
pub const fn index_obj_palette_4bpp(p: usize, i: usize) -> usize {
  let checked_p = const_bound_check(p, 16);
  let checked_i = const_bound_check(i, 16);
  let checked_index = (checked_p << 4) + checked_i;
  OBJ_PALETTE_RAM_ADDR + (OBJ_PALETTE_RAM_ENTRY_SIZE * checked_index)
}

/// Index the object palette to a 8bpp `index`.
///
/// ## Panics
/// `i` must be < 16.
pub const fn index_obj_palette_8bpp(i: usize) -> usize {
  let checked_index = const_bound_check(i, OBJ_PALETTE_RAM_COUNT);
  OBJ_PALETTE_RAM_ADDR + (OBJ_PALETTE_RAM_ENTRY_SIZE * checked_index)
}
