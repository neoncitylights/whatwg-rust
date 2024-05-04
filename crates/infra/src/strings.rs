extern crate alloc;
use alloc::{borrow::ToOwned, string::String};

/// Methods from the WHATWG Infra Standard for strings
pub trait InfraStr {
	/// See the documentation for [`normalize_newlines()`]
	fn normalize_newlines(&self) -> String;
	/// See the documentation for [`strip_newlines()`]
	fn strip_newlines(&self) -> String;
	/// See the documentation for [`trim_ascii_whitespace()`]
	fn trim_ascii_whitespace(&self) -> &str;
	/// See the documentation for [`trim_collapse_ascii_whitespace()`]
	fn trim_collapse_ascii_whitespace(&self) -> String;
	/// See the documentation for [`collect_codepoints()`]
	fn collect_codepoints<P>(&self, position: &mut usize, predicate: P) -> String
	where
		P: FnMut(char) -> bool;
	/// See the documentation for [`skip_codepoints()`]
	fn skip_codepoints<P>(&self, position: &mut usize, predicate: P)
	where
		P: FnMut(char) -> bool;
}

impl InfraStr for str {
	fn normalize_newlines(&self) -> String {
		normalize_newlines(self)
	}

	fn strip_newlines(&self) -> String {
		strip_newlines(self)
	}

	fn trim_ascii_whitespace(&self) -> &str {
		trim_ascii_whitespace(self)
	}

	fn trim_collapse_ascii_whitespace(&self) -> String {
		trim_collapse_ascii_whitespace(self)
	}

	fn collect_codepoints<P>(&self, position: &mut usize, predicate: P) -> String
	where
		P: FnMut(char) -> bool,
	{
		collect_codepoints(self, position, predicate)
	}

	fn skip_codepoints<P>(&self, position: &mut usize, predicate: P)
	where
		P: FnMut(char) -> bool,
	{
		skip_codepoints(self, position, predicate)
	}
}

/// Replaces every U+000D U+000A pair of codepoints with a single U+000A
/// codepoint, and any remaining U+000D codepoint with a U+000A codepoint.
///
/// See also: [WHATWG Infra Standard definition][whatwg-infra-dfn]
///
/// [whatwg-infra-dfn]: https://infra.spec.whatwg.org/#normalize-newlines
///
/// # Examples
/// ```
/// use whatwg_infra::normalize_newlines;
///
/// let s = "\ralice\r\n\r\nbob\r";
/// assert_eq!(normalize_newlines(s), String::from("\nalice\n\nbob\n"));
/// ```
#[must_use]
#[inline]
pub fn normalize_newlines(s: &str) -> String {
	s.replace("\u{000D}\u{000A}", "\u{000A}")
		.as_str()
		.replace('\u{000D}', "\u{000A}")
}

/// A string without any U+000A LINE FEED (LF) or U+000D CARIAGE RETURN (CR)
/// codepoints.
///
/// See also: [WHATWG Infra Standard definition][whatwg-infra-dfn]
///
/// [whatwg-infra-dfn]: https://infra.spec.whatwg.org/#strip-newlines
///
/// # Examples
/// ```
/// use whatwg_infra::strip_newlines;
///
/// let s = "Alice\n\rBob";
/// assert_eq!(strip_newlines(s), String::from("AliceBob"));
///
/// let empty = "\r\r\n\n\r\n";
/// assert_eq!(strip_newlines(empty), String::from(""));
/// ```
#[must_use]
#[inline]
pub fn strip_newlines(s: &str) -> String {
	let mut result = String::with_capacity(s.len());
	let mut stripped_codepoints = 0usize;

	for c in s.chars() {
		if c != '\u{000A}' && c != '\u{000D}' {
			result.push(c);
			stripped_codepoints += 1usize;
		}
	}

	if result.len() != s.len() {
		result.shrink_to(s.len() - stripped_codepoints);
	}

	result
}

/// Removes ASCII whitespace from before and after a string.
///
/// See also: [WHATWG Infra Standard definition][whatwg-infra-dfn]
///
/// [whatwg-infra-dfn]: https://infra.spec.whatwg.org/#strip-leading-and-trailing-ascii-whitespace
///
/// # Examples
/// ```
/// use whatwg_infra::trim_ascii_whitespace;
///
/// let s1 = "     ";
/// assert_eq!(trim_ascii_whitespace(s1), String::from(""));
///
/// let s2 = "  cats and dogs  ";
/// assert_eq!(trim_ascii_whitespace(s2), String::from("cats and dogs"));
/// ```
#[must_use]
pub fn trim_ascii_whitespace(s: &str) -> &str {
	s.trim_matches(|c: char| c.is_ascii_whitespace())
}

/// Removes ASCII whitespace from before and after a string, and collapses
/// runs of ASCII whitespaces by replacing them with a single U+0020 SPACE codepoint.
///
/// See also: [WHATWG Infra Standard definition][whatwg-infra-dfn]
///
/// [whatwg-infra-dfn]: https://infra.spec.whatwg.org/#strip-and-collapse-ascii-whitespace
///
/// # Examples
/// ```
/// use whatwg_infra::trim_collapse_ascii_whitespace;
///
/// let s = "\r  \n  cat dog  hamster";
/// assert_eq!(trim_collapse_ascii_whitespace(s), String::from("cat dog hamster"));
/// ```
#[must_use]
pub fn trim_collapse_ascii_whitespace(s: &str) -> String {
	let mut result = String::with_capacity(s.len());
	let mut last_seen_whitespace = false;

	for c in s.chars() {
		if c.is_ascii_whitespace() {
			if !last_seen_whitespace {
				last_seen_whitespace = true;
				result.push('\u{0020}');
				continue;
			}
		} else {
			last_seen_whitespace = false;
			result.push(c);
		}
	}

	trim_ascii_whitespace(result.as_str()).to_owned()
}

/// Collects a sequence of Unicode codepoints given a predicate function
/// and position to move forward.
///
/// See also: [WHATWG Infra Standard definition][whatwg-infra-dfn]
///
/// [whatwg-infra-dfn]: https://infra.spec.whatwg.org/#collect-a-sequence-of-code-points
///
/// # Examples
/// ```
/// use whatwg_infra::collect_codepoints;
///
/// let value = "test1";
/// let mut position = 0usize;
/// let collected = collect_codepoints(value, &mut position, |c| c.is_ascii_alphabetic());
///
/// assert_eq!(collected, String::from("test"));
/// assert_eq!(position, 4);
/// ```
pub fn collect_codepoints<P>(s: &str, position: &mut usize, predicate: P) -> String
where
	P: FnMut(char) -> bool,
{
	if s.is_empty() || position >= &mut s.len() {
		return String::new();
	}

	let mut result = String::with_capacity(s.len() - *position);
	let starting_position = *position;

	skip_codepoints(s, position, predicate);

	result.push_str(&s[starting_position..*position]);
	if result.len() < s.len() - *position {
		result.shrink_to_fit();
	}

	result
}

/// A non-allocating version of [`collect_codepoints()`] for skipping/ignoring
/// a series of codepoints that match a certain predicate.
///
/// ```
/// use whatwg_infra::skip_codepoints;
///
/// let s = "alice_bob";
/// let mut position = 0usize;
///
/// skip_codepoints(s, &mut position, |c| c.is_ascii_alphabetic());
///
/// assert_eq!(position, 5);
/// assert_eq!(&s[position..], "_bob");
/// ```
pub fn skip_codepoints<P>(s: &str, position: &mut usize, mut predicate: P)
where
	P: FnMut(char) -> bool,
{
	if s.is_empty() || position >= &mut s.len() {
		return;
	}

	let rest = s.chars().skip(*position);
	for c in rest {
		if position < &mut s.len() && predicate(c) {
			*position += 1;
		} else {
			break;
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_normalize_newlines() {
		assert_eq!(
			"\ralice\r\n\r\nbob\r".normalize_newlines(),
			String::from("\nalice\n\nbob\n")
		);
	}

	#[test]
	fn test_strip_newlines_empty() {
		assert_eq!("\r\r\n\n\r\n".strip_newlines(), String::from(""));
	}

	#[test]
	fn test_strip_newlines_empty2() {
		assert_eq!("".strip_newlines(), String::new());
	}

	#[test]
	fn test_strip_newlines_strings1() {
		assert_eq!("Alice\n\rBob".strip_newlines(), String::from("AliceBob"));
	}

	#[test]
	fn test_trim_ascii_whitespace_empty() {
		assert_eq!("     ".trim_ascii_whitespace(), String::from(""));
	}

	#[test]
	fn test_trim_ascii_whitespace_strings1() {
		assert_eq!(
			"  cats and dogs  ".trim_ascii_whitespace(),
			String::from("cats and dogs")
		);
	}

	#[test]
	fn test_trim_collapse_ascii_whitespace() {
		assert_eq!(
			"\r  \n  cat dog  hamster".trim_collapse_ascii_whitespace(),
			String::from("cat dog hamster")
		);
	}

	#[test]
	fn test_collect_codepoints_empty() {
		let mut position = 0usize;
		let collected = "".collect_codepoints(&mut position, |c| c.is_ascii_whitespace());

		assert_eq!(collected, String::new());
	}

	#[test]
	fn test_collect_codepoints_high_position() {
		let mut position = 15usize;
		let collected = "alice".collect_codepoints(&mut position, |c| c.is_alphabetic());

		assert_eq!(collected, String::new());
	}

	#[test]
	fn test_collect_codepoints_string2() {
		let test = "test!!!!!";
		let mut position = 0usize;
		let collected = test.collect_codepoints(&mut position, |c| c.is_ascii_alphabetic());
		assert_eq!(collected, String::from("test"));
		assert_eq!(position, 4);
	}

	#[test]
	fn test_collect_codepoints_either() {
		let value = "Apple    Banana    Orange";
		let mut position = 0usize;
		let collected = collect_codepoints(value, &mut position, |c| {
			c.is_alphabetic() || c.is_whitespace()
		});

		assert_eq!(collected, String::from("Apple    Banana    Orange"));
	}

	#[test]
	fn skip_codepoints() {
		let s = "1234test";
		let mut position = 0usize;

		s.skip_codepoints(&mut position, |c| c.is_ascii_digit());

		assert_eq!(position, 4);
		assert_eq!(&s[position..], "test");
	}

	#[test]
	fn skip_codepoints_no_matches_early_exit() {
		let s = "1234test";
		let mut position = 0usize;
		s.skip_codepoints(&mut position, |c| c.is_ascii_alphabetic());

		assert_eq!(position, 0);
		assert_eq!(&s[position..], "1234test");
	}

	#[test]
	fn skip_codepoints_match_until_end() {
		let s = "123456789";
		let mut position = 0usize;

		s.skip_codepoints(&mut position, |c| c.is_ascii_digit());

		assert_eq!(position, 9);
		assert_eq!(&s[position..], "");
	}

	#[test]
	fn skip_codepoints_empty_str() {
		let s = "";
		let mut position = 0usize;

		s.skip_codepoints(&mut position, |c| c.is_ascii_digit());

		assert_eq!(position, 0);
		assert_eq!(&s[position..], "");
	}
}
