//! Save RAM (`SRAM`).
//!
//! This lets you save/restore data for when the GBA is off.
//!
//! Note that memory in the `SRAM` region can **only** be accessed one byte at a
//! time.
//! * Attempting to read 16-bit or 32-bit units from `SRAM` will produce garbage
//!   values. Specifically, you get the byte at the target address (as if it was
//!   a byte pointer) and then that byte is mirrored to the given width.
//! * Attempting to write 16-bit or 32-bit values will write garbage to the
//!   target. Specifically, the target has `data ROR address*8` written to it
//!   (as if it was a byte pointer).
//!
//! `SRAM` cannot be accessed via any of the `DMA` units.
//!
//! `SRAM` is mirrored for 32MB starting at the base address. If the cart only
//! has 32kb of `SRAM`, then that is mirrored across the 64kb span.
//!
//! * **Size:** up to 64kb
//! * **Wait states:** variable (default is 4), but always more than zero.
//! * **Bus Size:** 8-bit
//! * **Reads:** 8

/// Base Address of `SRAM`
pub const SRAM_BASE_ADDR: usize = 0x0E00_0000;

/// `SRAM` **must** be accessed byte by byte.
pub const SRAM_ENTRY_SIZE: usize = 1;

/// There is up to 64kb of memory in `SRAM`.
///
/// Some carts have less than 64kb, in which case the available memory is
/// mirrored out to 64kb.
pub const SRAM_COUNT: usize = 64 * 1024;
