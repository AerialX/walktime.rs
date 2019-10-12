#![cfg_attr(feature = "unstable", feature(lang_items, core_intrinsics, never_type))]
#![no_std]

mod panic;
mod entry;
mod termination;

pub use termination::{ExitStatus, Never, Termination};
pub use entry::{start, exit, abort};
#[cfg(feature = "constructors")]
pub use entry::{InitContext, CONSTRUCTORS};

// Tests that entry point works properly
#[cfg(test)]
fn main() { }
