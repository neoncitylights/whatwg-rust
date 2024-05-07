# Changelog

## Unreleased (YYYY-MM-DD)

- Implemented `InfraStr` trait for the `String` type (previously only implemented for `str`).
- Added `skip_codepoints()`, a non-allocating alternative of `collect_codepoints()`.

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
