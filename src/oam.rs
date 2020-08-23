//! Object Attribute Memory (`OAM`).
//!
//! This portion of memory intermixes the Object (`OBJ`) Attributes, as well as
//! the Affine Parameters.
//!
//! * Object Attributes:
//!   * You get 128 object entries.
//!   * Each complete entry is three 16-bit values. These pseudo-fields are
//!     referred to as "attr0", "attr1", and "attr2".
//!   * There's also a 16-bit span that's *not* part of the object attributes
//!     after each set of attributes, before the next attribute entry begins.
//! * Affine Parameters
//!   * You get 32 affine parameter entries.
//!   * Each complete entry is four 16-bit values. These pseudo-fields are
//!     referred to as "pa", "pb", "pc", and "pd".
//!   * The parameters use the memory *in between* the object attribute memory
//!     entries described above. The the first `pa` is 6 bytes in from the start
//!     of `OAM`, then `pb`, `pc`, and `pd` are strided 8 bytes at a time from
//!     there, then the next affine parameter entry begins 8 bytes after that.
//!
//! You **cannot** validly write a single byte to `OAM`. If you do so, the
//! 8-bits are written to both the upper and lower 8-bits of the given 16-bit
//! segment of memory. This isn't a big deal, since all the pseudo-fields of the
//! stuff in `OAM` uses 16-bits anyway.
//!
//! * **Size:** 1kb
//! * **Wait states:** 0
//! * **Bus Size:** 32-bit
//! * **Read/Write:** 16/32

use super::*;

/// Base address of the object attribute `attr0` fields.
pub const OBJ_ATTR0_BASE_ADDR: usize = 0x0700_0000;

/// Base address of the object attribute `attr1` fields.
pub const OBJ_ATTR1_BASE_ADDR: usize = 0x0700_0002;

/// Base address of the object attribute `attr2` fields.
pub const OBJ_ATTR2_BASE_ADDR: usize = 0x0700_0004;

/// Stride of each object attribute entry.
pub const OBJ_ATTR_STRIDE: usize = 8;

/// There are 128 object attribute entries.
pub const OBJ_ATTR_COUNT: usize = 128;

/// Index the object attributes to a given entry.
///
/// This is the address of `attr0` for the requested set of object attributes.
/// The `attr1` and `attr2` addresses are each 2 bytes farther along
/// respectively.
///
/// ## Panics
/// `i` must be < 128.
pub const fn index_obj_attr(i: usize) -> usize {
  let checked_index = const_bound_check(i, OBJ_ATTR_COUNT);
  OBJ_ATTR0_BASE_ADDR + (OBJ_ATTR_STRIDE * checked_index)
}

/// Base address of the affine parameter `pa` fields.
pub const OBJ_AFFINE_PA_BASE_ADDR: usize = 0x0700_0006;

/// Base address of the affine parameter `pb` fields.
pub const OBJ_AFFINE_PB_BASE_ADDR: usize = 0x0700_000E;

/// Base address of the affine parameter `pc` fields.
pub const OBJ_AFFINE_PC_BASE_ADDR: usize = 0x0700_0016;

/// Base address of the affine parameter `pd` fields.
pub const OBJ_AFFINE_PD_BASE_ADDR: usize = 0x0700_001E;

/// Stride of each affine parameter entry.
pub const OBJ_AFFINE_STRIDE: usize = OBJ_ATTR_STRIDE * 4;

/// There are 32 affine parameter entries.
pub const OBJ_AFFINE_COUNT: usize = 32;

/// Index the affine parameters to a given entry.
///
/// This is the address of `pa` for the requested set of object attributes. The
/// `pb`, `pc`, and `pd` addresses are each 8 bytes farther along respectively.
///
/// ## Panics
/// `i` must be < 32.
pub const fn index_obj_affine_param(i: usize) -> usize {
  let checked_index = const_bound_check(i, OBJ_AFFINE_COUNT);
  OBJ_AFFINE_PA_BASE_ADDR + (OBJ_AFFINE_STRIDE * checked_index)
}
