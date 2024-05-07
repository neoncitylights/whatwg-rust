/*!
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
*/

mod components;
mod utils;

pub use crate::components::*;

pub type ParseStringFn<T> = dyn Fn(&str) -> Option<T>;
pub type ParseComponentFn<T> = dyn Fn(&str, &mut usize) -> Option<T>;
