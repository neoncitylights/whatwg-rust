# whatwg-datetime

[![License](https://img.shields.io/badge/License-MIT%20%26%20Apache%202.0-blue?style=flat-square)](#license)
[![master docs](https://img.shields.io/github/deployments/neoncitylights/whatwg-rust/github-pages?style=flat-square&label=master%20docs)](https://neoncitylights.github.io/whatwg-rust/whatwg_datetime/index.html)
[![docs.rs](https://img.shields.io/docsrs/whatwg-datetime/latest?style=flat-square&label=docs.rs)](https://docs.rs/whatwg-datetime/)
[![CI](https://img.shields.io/github/actions/workflow/status/neoncitylights/whatwg-rust/.github/workflows/main.yml?style=flat-square)](https://github.com/neoncitylights/whatwg-rust/actions/workflows/main.yml)
[![codecov](https://img.shields.io/codecov/c/github/neoncitylights/whatwg-rust?style=flat-square&logo=codecov&logoColor=%23fff)](https://codecov.io/github/neoncitylights/whatwg-rust)

A Rust crate for parsing the datetime microsyntax, as defined by the WHATWG HTML Standard.

## Install

```shell
cargo add whatwg-datetime
```

## Usage

This library currently implements 8 of the 9 datetime formats defined by the WHATWG HTML Standard. The only format not implemented is the duration format, which is tracked in [issue #29](https://github.com/neoncitylights/whatwg-rust/issues/29).

```rust
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use whatwg_datetime::parse_global_datetime;

assert_eq!(
	parse_global_datetime("2011-11-18T14:54Z"),
	Some(DateTime::<Utc>::from_utc(
		NaiveDateTime::new(
			NaiveDate::from_ymd_opt(2011, 11, 18).unwrap(),
			NaiveTime::from_hms_opt(14, 54, 0).unwrap(),
		),
		Utc,
	))
);
```

## License

Licensed under either of

- Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([`LICENSE-MIT`](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
