//! //! A tiny Rust crate that implements parts of the WHATWG Infra Standard. Specifically, it implements the following:
//!
//! - [4.5. Code points](https://infra.spec.whatwg.org/#code-points)
//! - [4.6. Strings](https://infra.spec.whatwg.org/#strings)
//!
//! It exposes a small set of primitives that are useful for parsing text into machine-readable data.
//!
//! ## Install
//!
//! ```shell
//! cargo add whatwg-infra
//! ```
//!
//! ## Usage
//!
//! You can import individual functions:
//!
//! ```rust
//! use whatwg_infra::{
//!     is_ascii_tab_newline,
//!     is_c0_control,
//!     is_c0_control_space,
//!     is_noncharacter
//! };
//!
//! assert!(is_ascii_tab_newline('\t'));
//! assert!(is_c0_control('\u{0000}'));
//! assert!(is_c0_control_space('\u{0020}'));
//! ```
//!
//! You can also import the traits to get all the functionality, and execute the methods on the types directly.
//!
//! ```rust
//! use whatwg_infra::{InfraScalarValue, InfraStr, InfraUtf16Surrogate};
//!
//! assert_eq!('a'.is_ascii_tab_newline(), false);
//! assert_eq!('\u{001E}'.is_c0_control(), true);
//! assert_eq!('\n'.is_c0_control_space(), true);
//! assert_eq!('\u{CFFFF}'.is_noncharacter(), true);
//! ```
//!
//! ## no_std
//!
//! This crate does not depend on libstd, and can be used in `#![no_std]` environments.
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
