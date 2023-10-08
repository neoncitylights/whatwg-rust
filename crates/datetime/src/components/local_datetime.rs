use crate::tokens::{TOKEN_SPACE, TOKEN_T};
use crate::{parse_date_component, parse_time_component};
use chrono::NaiveDateTime;

/// Parse a [proleptic-Gregorian date][proleptic-greg] consisting
/// of a date, time, with no time-zone information
///
/// This follows the rules for [parsing a local datetime string][whatwg-html-parse]
/// per [WHATWG HTML Standard § 2.3.5.5 Local dates and times][whatwg-html-local-datetime].
///
/// # Examples
/// ```
/// use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
/// use whatwg_datetime::parse_local_datetime;
///
/// // Parse a local datetime string with a date,
/// // a T delimiter, anda  time with fractional seconds
/// assert_eq!(
///     parse_local_datetime("2011-11-18T14:54:39.929"),
///     Some(NaiveDateTime::new(
///         NaiveDate::from_ymd_opt(2011, 11, 18).unwrap(),
///         NaiveTime::from_hms_milli_opt(14, 54, 39, 929).unwrap(),
///     ))
/// );
/// ```
///
/// [proleptic-greg]: https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#proleptic-gregorian-date
/// [whatwg-html-local-datetime]: https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#local-dates-and-times
/// [whatwg-html-parse]: https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#parse-a-local-date-and-time-string
pub fn parse_local_datetime(s: &str) -> Option<NaiveDateTime> {
	let mut position = 0usize;
	let date = parse_date_component(s, &mut position)?;

	let last_char = s.chars().nth(position);
	if position > s.len() || !matches!(last_char, Some(TOKEN_T) | Some(TOKEN_SPACE)) {
		return None;
	} else {
		position += 1;
	}

	let time = parse_time_component(s, &mut position)?;
	if position < s.len() {
		return None;
	}

	Some(NaiveDateTime::new(date, time))
}

#[cfg(test)]
mod tests {
	use super::parse_local_datetime;
	use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

	#[test]
	pub fn test_parse_local_datetime_delimited_t_date_hm() {
		assert_eq!(
			parse_local_datetime("2004-12-31T12:31"),
			Some(NaiveDateTime::new(
				NaiveDate::from_ymd_opt(2004, 12, 31).unwrap(),
				NaiveTime::from_hms_opt(12, 31, 0).unwrap(),
			))
		);
	}

	#[test]
	pub fn test_parse_local_datetime_delimited_t_date_hms() {
		assert_eq!(
			parse_local_datetime("2004-12-31T12:31:59"),
			Some(NaiveDateTime::new(
				NaiveDate::from_ymd_opt(2004, 12, 31).unwrap(),
				NaiveTime::from_hms_opt(12, 31, 59).unwrap(),
			))
		);
	}

	#[test]
	pub fn test_parse_local_datetime_delimited_t_date_hms_milliseconds() {
		assert_eq!(
			parse_local_datetime("2011-11-18T14:54:39.929"),
			Some(NaiveDateTime::new(
				NaiveDate::from_ymd_opt(2011, 11, 18).unwrap(),
				NaiveTime::from_hms_milli_opt(14, 54, 39, 929).unwrap(),
			))
		)
	}

	#[test]
	pub fn test_parse_local_datetime_delimited_space_date_hm() {
		assert_eq!(
			parse_local_datetime("2011-11-18 14:54"),
			Some(NaiveDateTime::new(
				NaiveDate::from_ymd_opt(2011, 11, 18).unwrap(),
				NaiveTime::from_hms_opt(14, 54, 0).unwrap(),
			))
		)
	}

	#[test]
	pub fn test_parse_local_datetime_delimited_space_date_hms() {
		assert_eq!(
			parse_local_datetime("2011-11-18 14:54:39"),
			Some(NaiveDateTime::new(
				NaiveDate::from_ymd_opt(2011, 11, 18).unwrap(),
				NaiveTime::from_hms_opt(14, 54, 39).unwrap(),
			))
		)
	}

	#[test]
	pub fn test_parse_local_datetime_delimited_space_date_hms_milliseconds() {
		assert_eq!(
			parse_local_datetime("2011-11-18 14:54:39.929"),
			Some(NaiveDateTime::new(
				NaiveDate::from_ymd_opt(2011, 11, 18).unwrap(),
				NaiveTime::from_hms_milli_opt(14, 54, 39, 929).unwrap(),
			))
		)
	}

	#[test]
	pub fn test_parse_local_datetime_fails_invalid_delimiter() {
		assert_eq!(parse_local_datetime("2011-11-18W14-54-39"), None);
	}

	#[test]
	pub fn test_parse_local_datetime_fails_invalid_date() {
		assert_eq!(parse_local_datetime("2011/11/18T14:54:39.929"), None);
	}

	#[test]
	pub fn test_parse_local_datetime_fails_invalid_time() {
		assert_eq!(parse_local_datetime("2011-11-18T14/54/39"), None);
	}
}
