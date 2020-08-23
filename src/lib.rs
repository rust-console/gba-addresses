#![no_std]
#![warn(missing_docs)]

//! Exports address values of the GBA memory map locations.
//!
//! Also, for some memory regions that are "blocks", const indexing functions
//! are provided.
//!
//! ## Crate Layout
//!
//! All info is exported at the crate root, but the various modules provide some
//! notes about the general usage of that portion of memory.

const fn const_bound_check(index: usize, bound: usize) -> usize {
  const ARRAY: [&str; 1] = ["index out of bounds"];
  ARRAY[(index >= bound) as usize];
  index
}

macro_rules! const_assert {
  ($expr:expr) => {
    const _: () = {
      #[allow(dead_code)]
      const FAILED_ASSERTION: [&str; 1] =
        [concat!("failed assertion: '", stringify!($expr), "' is false",)];
      #[allow(dead_code)]
      const CONDITION: bool = $expr;
      #[allow(dead_code)]
      const ASSERTION: &str = FAILED_ASSERTION[(!CONDITION) as usize];
    };
  };

  // This will let the user see a message
  ($expr:expr, $msg:literal) => {
    const _: () = {
      #[allow(dead_code)]
      const FAILED_ASSERTION: [&str; 1] = [concat!(
        "failed assertion: '",
        stringify!($expr),
        "' is false: ",
        $msg,
      )];
      #[allow(dead_code)]
      const CONDITION: bool = $expr;
      #[allow(dead_code)]
      const ASSERTION: &str = FAILED_ASSERTION[(!CONDITION) as usize];
    };
  };
}

pub mod ewram;
pub use ewram::*;

pub mod iwram;
pub use iwram::*;

pub mod io;
pub use io::*;

pub mod palram;
pub use palram::*;

pub mod vram;
pub use vram::*;

pub mod oam;
pub use oam::*;

pub mod rom;
pub use rom::*;

pub mod sram;
pub use sram::*;
