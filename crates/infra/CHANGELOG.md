# Changelog

## 1.1.0 (2024-11-03)

- Added `skip_ascii_whitespace()`, a convenience function and implementation of [4.6. Strings § skip ASCII whitespace](https://infra.spec.whatwg.org/#skip-ascii-whitespace).

## 1.0.0 (2024-05-07)

- Implemented `InfraStr` trait for the `String` type (previously only implemented for `str`).
- Added `skip_codepoints()`, a non-allocating alternative of `collect_codepoints()`.
- Simplifies the function signatures of the following:
  - `collect_codepoints()` now takes a generic parameter `P` that must satisfy `Fn(char) -> bool` (previously `FnMut(char) -> bool`)
  - `skip_codepoints()` now takes a generic parameter `P` that must satisfy `Fn(char) -> bool` (previously `FnMut(char) -> bool`)
  - `skip_codepoints()` no longer takes a mutable parameter for `predicate` of `P` (now `predicate: P` instead of `mut predicate: P`)
  - `InfraStr` trait follows suit with the same changes mentioned above

## 0.2.2 (2023-11-08)

- Fixed typos in documentation for [`trim_collapse_ascii_whitespace()`] and [`is_surrogate_utf16()`]
- Fixed/updated repository URLs

## 0.2.1 (2023-05-15)

- Removed `#[must_use]` attribute from `collect_codepoints()`, to allow for use cases for skipping certain codepoints (e.g skipping ASCII whitespace)

## 0.2.0 (2023-04-30)

- Added 3 new traits:
  - `InfraScalarValue` trait for `char` type
  - `InfraStr` trait for `str` type
  - `InfraUtf16Surrogate` trait for `u16` type
- Predicate functions that take a `char` no longer take a reference. This is since `char` types are cheap to copy, and to stay consistent with the Rust's standard library function signatures.
  - `whatwg_infra::is_ascii_tab_newline()`
  - `whatwg_infra::is_c0_control()`
  - `whatwg_infra::is_c0_control_space()`
  - `whatwg_infra::is_noncharacter()`
- Add `trim_collapse_ascii_whitespace()` to trim and remove consecutive ASCII whitespace
- Add more extensive unit tests and documentation examples
- Organize code into modules (`scalar`, `strings`, and `surrogates`) and re-export

## 0.1.0 (2023-03-20)

- Initial release of the whatwg-infra Rust library
