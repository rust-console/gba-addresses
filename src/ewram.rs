//! External Work Ram (`EWRAM`).
//!
//! There is no default usage of this memory. You can use it for anything that
//! you like.
//!
//! Note that because of the 2 wait cycles on all accesses of this memory, heavy
//! work should generally move the data into `IWRAM`, do the changes, and then
//! copy the results back to `EWRAM`.
//!
//! * **Size:** 256kb
//! * **Wait states:** 2
//! * **Bus Size:** 32-bit
//! * **Read/Write:** 8/16/32

/// Base Address of `EWRAM`
pub const EWRAM_START_ADDR: usize = 0x0200_0000;

/// `EWRAM` can be accessed byte by byte.
pub const EWRAM_ENTRY_SIZE: usize = 1;

/// There is 256kb of memory in `EWRAM`.
pub const EWRAM_COUNT: usize = 256 * 1024;
