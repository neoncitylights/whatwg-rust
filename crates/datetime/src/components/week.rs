use crate::tokens::{TOKEN_ABBR_WEEK, TOKEN_HYPHEN};
use crate::utils::{collect_ascii_digits, week_number_of_year};

/// A week date consisting of a year and a week number.
///
/// # Examples
/// ```
/// use whatwg_datetime::{parse_week, YearWeek};
///
/// assert_eq!(parse_week("2011-W47"), YearWeek::new_opt(2011, 47));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YearWeek {
	pub(crate) year: i32,
	pub(crate) week: u32,
}

impl YearWeek {
	#[inline]
	pub(crate) const fn new(year: i32, week: u32) -> Self {
		Self { year, week }
	}

	/// Creates a new `YearWeek` from a year and a week number.
	///
	/// This asserts that the year is greater than 0 and that the week number
	/// is in the valid range for the year. Specifically, the week number
	/// must be between 1 and the number of weeks in the year, inclusive.
	///
	/// The number of weeks in a year is described by the algorithm
	/// in [WHATWG HTML Standard § 2.3.5.8 Weeks][whatwg-html-weeks].
	///
	/// # Examples
	/// ```
	/// use whatwg_datetime::YearWeek;
	///
	/// assert!(YearWeek::new_opt(2004, 53).is_some());
	/// assert!(YearWeek::new_opt(2011, 47).is_some());
	/// assert!(YearWeek::new_opt(2011, 53).is_none()); // 2011 only has 52 weeks
	/// assert!(YearWeek::new_opt(1952, 0).is_none()); // Week number must be at least 1
	/// assert!(YearWeek::new_opt(0, 1).is_none()); // Year number must be greater than 0
	/// ```
	pub fn new_opt(year: i32, week: u32) -> Option<Self> {
		if year <= 0 {
			return None;
		}

		if week < 1 || week > week_number_of_year(year)? {
			return None;
		}

		Some(Self::new(year, week))
	}

	/// A year component. This is a number greater than 0.
	///
	/// # Examples
	/// ```
	/// use whatwg_datetime::YearWeek;
	///
	/// let year_week = YearWeek::new_opt(2004, 53).unwrap();
	/// assert_eq!(year_week.year(), 2004);
	/// ```
	#[inline]
	pub const fn year(&self) -> i32 {
		self.year
	}

	/// A week component. This is a number between 1 and the number of weeks
	/// in the year, inclusive.
	///
	/// # Examples
	/// ```
	/// use whatwg_datetime::YearWeek;
	///
	/// let year_week = YearWeek::new_opt(2004, 53).unwrap();
	/// assert_eq!(year_week.week(), 53);
	/// ```
	#[inline]
	pub const fn week(&self) -> u32 {
		self.week
	}
}

/// Parse a week-year number and a week-number
///
/// This follows the rules for [parsing a week string][whatwg-html-parse]
/// per [WHATWG HTML Standard § 2.3.5.8 Weeks][whatwg-html-weeks].
///
/// # Examples
/// ```
/// use whatwg_datetime::{parse_week, YearWeek};
///
/// assert_eq!(parse_week("2004-W53"), YearWeek::new_opt(2004, 53));
/// assert_eq!(parse_week("2011-W47"), YearWeek::new_opt(2011, 47));
/// assert_eq!(parse_week("2011-W53"), None); // 2011 only has 52 weeks
/// ```
///
/// [whatwg-html-weeks]: https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#weeks
/// [whatwg-html-parse]: https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#parse-a-week-string
pub fn parse_week(input: &str) -> Option<YearWeek> {
	// Step 1, 2
	let mut position = 0usize;

	// Step 3, 4
	let year_string = collect_ascii_digits(input, &mut position);
	let year = year_string.parse::<i32>().unwrap();
	if year <= 0 {
		return None;
	}

	// Step 5
	if position > input.len() || input.chars().nth(position) != Some(TOKEN_HYPHEN) {
		return None;
	} else {
		position += 1;
	}

	// Step 6
	if position > input.len() || input.chars().nth(position) != Some(TOKEN_ABBR_WEEK) {
		return None;
	} else {
		position += 1;
	}

	// Step 7
	let parsed_week = collect_ascii_digits(input, &mut position);
	if parsed_week.len() != 2 {
		return None;
	}

	let week = parsed_week.parse::<u32>().unwrap();
	let max_weeks = week_number_of_year(year)?;
	if week < 1 || week > max_weeks {
		return None;
	}

	Some(YearWeek::new(year, week))
}

#[cfg(test)]
mod tests {
	use super::{parse_week, YearWeek};

	#[test]
	fn test_parse_week() {
		assert_eq!(parse_week("2004-W53"), Some(YearWeek::new(2004, 53)));
	}

	#[test]
	fn test_parse_week_fails_year_is_zero() {
		assert_eq!(parse_week("0000-W01"), None);
	}

	#[test]
	fn test_parse_week_fails_invalid_separator() {
		assert_eq!(parse_week("2004_W01"), None);
	}

	#[test]
	fn test_parse_week_fails_invalid_week_abbr() {
		assert_eq!(parse_week("2003-𝓌01"), None);
	}

	#[test]
	fn test_parse_week_fails_invalid_week_length() {
		assert_eq!(parse_week("2004-W1"), None);
		assert_eq!(parse_week("2008-W001"), None);
	}

	#[test]
	fn test_parse_week_fails_invalid_week_num_lower_bound() {
		assert_eq!(parse_week("2022-W00"), None);
		assert_eq!(parse_week("1897-W00"), None);
	}

	#[test]
	fn test_parse_week_fails_invalid_week_num_upper_bound() {
		assert_eq!(parse_week("2004-W54"), None);
		assert_eq!(parse_week("1996-W53"), None);
	}
}
