//! Video RAM (`VRAM`).
//!
//! The exact usage of this memory depends on your video mode:
//! * **Mode 0:** Layers 0, 1, 2 and 3 are text mode.
//! * **Mode 1:** Layers 0 and 1 are textual, layer 2 is affine.
//! * **Mode 2:** Layers 2 and 3 are affine.
//! * **Mode 3:** Layer 2 is a 240x160 direct color bitmap.
//! * **Mode 4:** Layer 2 is two 240x160 indexed color bitmaps.
//! * **Mode 5:** Layer 2 is two 160x128 direct color bitmaps.
//!
//! A tile is always 8x8, but tiles can be either 4bpp and 8bpp.
//!
//! Many things related to `VRAM` work in "Charblocks":
//! * A charblock is 16kb.
//! * In terms of charblocks, `VRAM` has 4 blocks for background tiles and 2
//!   blocks for object tiles.
//! * In video modes 0, 1, and 2 both tile data and screenblocks occupy the
//!   first 4 charblocks.
//! * In video mode 3, 4, and 5, the bitmap data occupies all background
//!   charblocks and also the first object charblock, so only the final
//!   charblock is available for object tile data.
//!
//! A "Screenblock" gives the layout of how tiles should be arranged.
//! * A screenblock index always is in terms of 2kb, so indexes are always in
//!   the range `0..32`. Note that the actual size of a full screenblock isn't
//!   necessarily 2kb.
//! * Screenblocks and tile image data both have to share space in VRAM.
//! * **Text Mode:**
//!   * 2 bytes per screen entry, 32x32 entries per screenblock (2kb).
//!   * A layer can be 1, 2, or 4 screenblocks:
//!     * 32x32: one screenblock (square)
//!     * 64x32: one screenblock, and the next screenblock on the right
//!       (horizontal).
//!     * 32x64: one screenblock, and the next screenblock beneath (vertical).
//!     * 64c64: the base screenblock, then down, then right, then down right
//!       (large square).
//! * **Affine Mode:**
//!   * 1 byte per entry, entry count per screenblock depends on the background
//!     size: 16x16 (256b), 32x32 (1kb), 64x64 (4kb), or 128x128 (16kb).
//!   * Each layer is a one screenblock.
//!
//! You **cannot** validly write a single byte to `VRAM`. If you do so, the
//! 8-bits are written to both the upper and lower 8-bits of the given 16-bit
//! segment of memory. This can cause a problem when working with the video mode
//! 4 bitmap, or with the video mode 1 or 2 affine screenblocks. To modify a
//! single byte correctly you must read at least 16-bits, mask in the new 8-bit
//! value, and then write back the larger value.
//!
//! * **Size:** 96kb
//! * **Wait states:** 0
//! * **Bus Size:** 16-bit
//! * **Read/Write:** 16/32

use super::*;

/// The base address for `VRAM`, regardless of video mode.
pub const VRAM_BASE_ADDR: usize = 0x0600_0000;

/// The size of a 4bpp tile.
pub const TILE_4BPP_SIZE: usize = 32;

/// The size of an 8bpp tile.
pub const TILE_8BPP_SIZE: usize = 64;

/// The number of 4bpp tiles in a Charblock.
pub const CHARBLOCK_4BPP_COUNT: usize = 512;

/// The number of 8bpp tiles in a Charblock.
pub const CHARBLOCK_8BPP_COUNT: usize = 256;

/// The size of a single Charblock.
///
/// A charblock contains more or less tiles depending on tile size, but the
/// number of bytes per charblock is always fixed.
pub const CHARBLOCK_SIZE: usize = CHARBLOCK_4BPP_COUNT * TILE_4BPP_SIZE;
const_assert!(
  CHARBLOCK_4BPP_COUNT * TILE_4BPP_SIZE
    == CHARBLOCK_8BPP_COUNT * TILE_8BPP_SIZE
);

/// The base address of the background charblocks.
pub const CHARBLOCK_BG_BASE_ADDR: usize = VRAM_BASE_ADDR;

/// There are 4 background charblocks.
pub const CHARBLOCK_BG_COUNT: usize = 4;

/// Index to a given background charblock address.
///
/// ## Panics
/// `i` must be < 4.
pub const fn index_bg_charblock(i: usize) -> CharblockAddress {
  let checked_index = const_bound_check(i, CHARBLOCK_BG_COUNT);
  CharblockAddress(CHARBLOCK_BG_BASE_ADDR + (CHARBLOCK_SIZE * checked_index))
}

/// This is just a `usize`, but it also allows indexing to a specific tile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct CharblockAddress(usize);
impl CharblockAddress {
  /// Indexes to a given 4bpp tile within this charblock.
  ///
  /// ## Panics
  /// `i` must be < 512.
  pub const fn index_tile_4bpp(self, i: usize) -> usize {
    let checked_index = const_bound_check(i, CHARBLOCK_4BPP_COUNT);
    self.0 + (CHARBLOCK_SIZE * checked_index)
  }

  /// Indexes to a given 8bpp tile within this charblock.
  ///
  /// ## Panics
  /// `i` must be < 256.
  pub const fn index_tile_8bpp(self, i: usize) -> usize {
    let checked_index = const_bound_check(i, CHARBLOCK_8BPP_COUNT);
    self.0 + (CHARBLOCK_SIZE * checked_index)
  }

  /// Unwrap the value into a `usize`.
  pub const fn as_usize(self) -> usize {
    self.0
  }
}

/// The base address of the object charblocks.
pub const CHARBLOCK_OBJ_BASE_ADDR: usize =
  VRAM_BASE_ADDR + CHARBLOCK_SIZE * CHARBLOCK_BG_COUNT;

/// There are 2 object charblocks.
///
/// The lower object charblock isn't usable in video modes 3, 4, and 5.
pub const CHARBLOCK_OBJ_COUNT: usize = 2;

/// The number of object tile indexes available.
///
/// Object tile indexes always index into memory as if they were 4bpp.
pub const CHARBLOCK_OBJ_TILE_COUNT: usize =
  CHARBLOCK_OBJ_COUNT * TILE_4BPP_SIZE;

/// Index to a given object tile.
///
/// Unlike with background tiles:
/// * Object tile values are *always* relative to the beginning of the object
///   charblocks.
/// * Object tile indexes *always* index into memory as if they were 4bpp
///   entries (even if the object is in 8bpp mode).
///
/// ## Panics
/// `i` must be < 1024.
pub const fn index_obj_tile(i: usize) -> usize {
  let checked_index = const_bound_check(i, CHARBLOCK_OBJ_TILE_COUNT);
  CHARBLOCK_OBJ_BASE_ADDR + (TILE_4BPP_SIZE * checked_index)
}

/// Index to the start of a particular screenblock.
///
/// ## Panics
/// `i` must be < 32.
pub const fn index_screenblock(i: usize) -> usize {
  let checked_index = const_bound_check(i, 32);
  CHARBLOCK_OBJ_BASE_ADDR + (TILE_4BPP_SIZE * checked_index)
}

/// The size of a text mode screen entry.
pub const TEXT_SCREENBLOCK_ENTRY_SIZE: usize = 2;

/// The number of entries in a text screenblock.
pub const TEXT_SCREENBLOCK_ENTRY_COUNT: usize = 32 * 32;

/// The size of a text mode screenblock.
pub const TEXT_SCREENBLOCK_SIZE: usize =
  TEXT_SCREENBLOCK_ENTRY_SIZE * TEXT_SCREENBLOCK_ENTRY_COUNT;
const_assert!(TEXT_SCREENBLOCK_SIZE == 2 * 1024);

/// The size of an affine mode screen entry.
pub const AFFINE_SCREENBLOCK_ENTRY_SIZE: usize = 1;

/// The number of entries in a size 0 affine screenblock.
pub const AFFINE_SIZE0_SCREENBLOCK_ENTRY_COUNT: usize = 16 * 16;

/// The size of a size 0 affine screenblock.
pub const AFFINE_SIZE0_SCREENBLOCK_SIZE: usize =
  AFFINE_SCREENBLOCK_ENTRY_SIZE * AFFINE_SIZE0_SCREENBLOCK_ENTRY_COUNT;
const_assert!(AFFINE_SIZE0_SCREENBLOCK_SIZE == 256);

/// The number of entries in a size 1 affine screenblock.
pub const AFFINE_SIZE1_SCREENBLOCK_ENTRY_COUNT: usize = 32 * 32;

/// The size of a size 1 affine screenblock.
pub const AFFINE_SIZE1_SCREENBLOCK_SIZE: usize =
  AFFINE_SCREENBLOCK_ENTRY_SIZE * AFFINE_SIZE1_SCREENBLOCK_ENTRY_COUNT;
const_assert!(AFFINE_SIZE1_SCREENBLOCK_SIZE == 1024);

/// The number of entries in a size 2 affine screenblock.
pub const AFFINE_SIZE2_SCREENBLOCK_ENTRY_COUNT: usize = 64 * 64;

/// The size of a size 2 affine screenblock.
pub const AFFINE_SIZE2_SCREENBLOCK_SIZE: usize =
  AFFINE_SCREENBLOCK_ENTRY_SIZE * AFFINE_SIZE2_SCREENBLOCK_ENTRY_COUNT;
const_assert!(AFFINE_SIZE2_SCREENBLOCK_SIZE == 1024 * 4);

/// The number of entries in a size 3 affine screenblock.
pub const AFFINE_SIZE3_SCREENBLOCK_ENTRY_COUNT: usize = 128 * 128;

/// The size of a size 3 affine screenblock.
pub const AFFINE_SIZE3_SCREENBLOCK_SIZE: usize =
  AFFINE_SCREENBLOCK_ENTRY_SIZE * AFFINE_SIZE3_SCREENBLOCK_ENTRY_COUNT;
const_assert!(AFFINE_SIZE3_SCREENBLOCK_SIZE == 1024 * 16);

/// Base address of the bitmap frame 0 (video modes 3, 4, and 5).
pub const VRAM_FRAME0_BASE_ADDR: usize = VRAM_BASE_ADDR;

/// Base address of the bitmap frame 1 (video modes 4 or 5).
pub const VRAM_MODE4_FRAME1_BASE_ADDR: usize = 0x0600_A000;
