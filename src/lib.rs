//! Mitochondria is the powerhouse of the `Cell`.
//!
//! This crate provides additional mutable containers for use cases not
//! covered by the triumvirate of `Cell`, `RefCell` and `UnsafeCell`.

#![deny(missing_docs)]

mod cloning;
mod once;

pub use cloning::CloningCell;
pub use once::OnceCell;
