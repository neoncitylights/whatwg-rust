# whatwg-infra

[![License](https://img.shields.io/badge/License-MIT%20%26%20Apache%202.0-blue?style=flat-square)](#license)
[![master docs](https://img.shields.io/github/deployments/neoncitylights/whatwg-rust/github-pages?style=flat-square&label=master%20docs)](https://neoncitylights.github.io/whatwg-rust/whatwg_infra/index.html)
[![docs.rs](https://img.shields.io/docsrs/whatwg-infra/latest?style=flat-square&label=docs.rs)](https://docs.rs/whatwg-infra/)
[![CI](https://img.shields.io/github/actions/workflow/status/neoncitylights/whatwg-rust/.github/workflows/main.yml?style=flat-square)](https://github.com/neoncitylights/whatwg-rust/actions/workflows/main.yml)
[![codecov](https://img.shields.io/codecov/c/github/neoncitylights/whatwg-rust?style=flat-square&logo=codecov&logoColor=%23fff)](https://codecov.io/github/neoncitylights/whatwg-rust)

A tiny Rust crate that implements parts of the WHATWG Infra Standard. Specifically, it implements the following:

- [4.5. Code points](https://infra.spec.whatwg.org/#code-points)
- [4.6. Strings](https://infra.spec.whatwg.org/#strings)

It exposes a small set of primitives that are useful for parsing text into machine-readable data.

## Install

```shell
cargo add whatwg-infra
```

## Usage

You can import individual functions:

```rust
use whatwg_infra::{
	is_ascii_tab_newline,
	is_c0_control,
	is_c0_control_space,
	is_noncharacter
};

assert!(is_ascii_tab_newline('\t'));
assert!(is_c0_control('\u{0000}'));
assert!(is_c0_control_space('\u{0020}'));

```

You can also import the traits to get all the functionality, and execute the methods on the types directly.

```rust
use whatwg_infra::{InfraScalarValue, InfraStr, InfraUtf16Surrogate};

assert_eq!('a'.is_ascii_tab_newline(), false);
assert_eq!('\u{001E}'.is_c0_control(), true);
assert_eq!('\n'.is_c0_control_space(), true);
assert_eq!('\u{CFFFF}'.is_noncharacter(), true);
```

## no_std

This crate does not depend on libstd, and can be used in `#![no_std]` environments.

## License

Licensed under either of

* Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([`LICENSE-MIT`](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
