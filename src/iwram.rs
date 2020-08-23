//! Internal Work Ram (`IWRAM`).
//!
//! The internal work ram has a number of uses, mostly as space for stack
//! values. It is also common for your linker script to put any globals at the
//! base of `IWRAM` (though Rust sorta hates globals, of course).
//!
//! Each stack pointer starts at the *top* of its respective region, and then
//! stack usage moves the stack pointer *down*. Don't stack overflow.
//!
//! * **Size:** 32kb
//! * **Wait states:** 0
//! * **Bus Size:** 32-bit
//! * **Read/Write:** 8/16/32

/// Base Address of `IWRAM`
pub const IWRAM_START_ADDR: usize = 0x0300_0000;

/// `IWRAM` can be accessed byte by byte.
pub const IWRAM_ENTRY_SIZE: usize = 1;

/// There is 32kb of memory in `IWRAM`, though parts of it are reserved.
pub const IWRAM_COUNT: usize = 32 * 1024;

/// All `IWRAM` above this point is reserved.
pub const IWRAM_RESERVED: usize = 0x0300_7F00;

/// The location of the interrupt handler function address.
pub const IRQ_HANDLER_ADDR: usize = 0x0300_7FFC;

/// The address of the check flag for `IntrWait`/`VBlankIntrWait`.
pub const IRQ_INTR_WAIT_CHECK_FLAG_ADDR: usize = 0x0300_7FF8;

/// A mirror of
/// [`IRQ_INTR_WAIT_CHECK_FLAG_ADDR`](IRQ_INTR_WAIT_CHECK_FLAG_ADDR).
///
/// This one is chosen because it's only a few bytes away from `0x0400_0000`,
/// and so you can access it with an offset load/store.
pub const IRQ_INTR_WAIT_CHECK_FLAG_LAST_MIRROR_ADDR: usize = 0x03FF_FFF8;

/// Default stack pointer for supervisor mode.
pub const DEFAULT_SP_SVC: usize = 0x0300_7FE0;

/// Default stack pointer for interrupt mode.
pub const DEFAULT_SP_IRQ: usize = 0x0300_7FA0;

/// Default stack pointer for user/system mode.
pub const DEFAULT_SP_USER: usize = 0x0300_7F00;
