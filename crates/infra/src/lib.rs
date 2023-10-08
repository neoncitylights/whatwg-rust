#![doc = include_str!("../README.md")]
#![no_std]

/// Detection of UTF-16 surrogate codepoints for `u16`
///
/// This module exposes predicate functions for detecting surrogates,
/// including trailing and leading surrogates.
pub mod surrogates;
pub use crate::surrogates::*;

/// Module for Unicode scalar values
pub mod scalar;
pub use crate::scalar::*;

/// Module for Unicode strings
pub mod strings;
pub use crate::strings::*;
