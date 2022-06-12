#![feature(const_convert, const_trait_impl, const_option)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
// Safety-critical application lints
#![deny(
    clippy::pedantic,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::unwrap_used
)]
#![allow(
    dead_code,
    clippy::equatable_if_let,
    clippy::enum_glob_use,
    clippy::implicit_return,
    clippy::iter_nth_zero,
    clippy::match_bool,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::wildcard_imports
)]
// To use the `unsafe` keyword, do not remove the `unsafe_code` attribute entirely.
// Instead, change it to `#![allow(unsafe_code)]` or preferably `#![deny(unsafe_code)]` + opt-in
// with local `#[allow(unsafe_code)]`'s on a case-by-case basis, if practical.
#![forbid(unsafe_code)]
#![forbid(bare_trait_objects)]
// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing
// license files and more
// #![allow(clippy::blanket_clippy_restriction_lints)]
// #![warn(clippy::cargo, clippy::restriction, missing_docs, dead_code, warnings)]
// #![allow(clippy::implicit_return)]

mod consts;
mod cpu;
mod error;
mod memory;
mod traits;
mod utils;

pub use cpu::{Cpu, VectorTable};
pub use error::{Error, Result};
pub use memory::{Address, Memory, ZeroPageAddress};
pub use traits::IAddress;
use traits::ISealed;
#[cfg(test)]
use utils::catch_unwind_silent;
