//! Read Only Memory (`ROM`).
//!
//! A GBA's ROM can be up to 32MB.
//!
//! An interesting detail is that the ROM content is mirrored to three different
//! locations in the address space:
//! * **Wait State 0:** `0x0800_0000`, this is the "normal" access point.
//! * **Wait State 1:** `0x0A00_0000`.
//! * **Wait State 2:** `0x0C00_0000`.
//!
//! By using the [`WAITCNT_ADDR`] you can modify the wait states required for
//! each of the three ROM mirrors independently.
//!
//! * **Size:** up to 32MB
//! * **Wait states:** variable (default is 4), but always more than zero.
//! * **Bus Size:** 16-bit
//! * **Reads:** 8/16/32

/// ROM base address for wait state 0.
pub const ROM_WAIT0_BASE_ADDR: usize = 0x0800_0000;

/// ROM base address for wait state 1.
pub const ROM_WAIT1_BASE_ADDR: usize = 0x0A00_0000;

/// ROM base address for wait state 2.
pub const ROM_WAIT2_BASE_ADDR: usize = 0x0C00_0000;
