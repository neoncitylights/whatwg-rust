/// Methods from the WHATWG Infra Standard for Unicode codepoints
#[allow(clippy::wrong_self_convention)]
pub trait InfraScalarValue {
	/// See the documentation for [`is_ascii_tab_newline()`]
	fn is_ascii_tab_newline(self) -> bool;
	/// See the documentation for [`is_c0_control()`]
	fn is_c0_control(self) -> bool;
	/// See the documentation for [`is_c0_control_space()`]
	fn is_c0_control_space(self) -> bool;
	/// See the documentation for [`is_noncharacter()`]
	fn is_noncharacter(self) -> bool;
}

impl InfraScalarValue for char {
	fn is_ascii_tab_newline(self) -> bool {
		is_ascii_tab_newline(self)
	}

	fn is_c0_control(self) -> bool {
		is_c0_control(self)
	}

	fn is_c0_control_space(self) -> bool {
		is_c0_control_space(self)
	}

	fn is_noncharacter(self) -> bool {
		is_noncharacter(self)
	}
}

/// Asserts a codepoint is a "noncharacter" based on a certain range of
/// Unicode codepoints.
///
/// > A noncharacter is a codepoint that is in the range U+FDD0 to U+FDEF,
/// > inclusive, or U+FFFE, U+FFFF, U+1FFFE, U+1FFFF, U+2FFFE, U+2FFFF, U+3FFFE,
/// > U+3FFFF, U+4FFFE, U+4FFFF, U+5FFFE, U+5FFFF, U+6FFFE, U+6FFFF, U+7FFFE,
/// > U+7FFFF, U+8FFFE, U+8FFFF, U+9FFFE, U+9FFFF, U+AFFFE, U+AFFFF, U+BFFFE,
/// > U+BFFFF, U+CFFFE, U+CFFFF, U+DFFFE, U+DFFFF, U+EFFFE, U+EFFFF, U+FFFFE,
/// > U+FFFFF, U+10FFFE, or U+10FFFF.
///
/// Essentially, a noncharacter includes:
///  - the 36 codepoints from U+FDD0 to U+FDEF,
///  - the U+..FFFE U+..FFFF codepoints in all 17 Unicode planes which are
///    guaranteed to never encode as anything, per the Unicode Standard
///    (in [Section 3.2, Conformance Requirements][unicode-s3-2] and
///    [Section 3.4, Characters and Encoding][unicode-s3-4]).
///
/// See also: [WHATWG Infra Standard definition][whatwg-infra-dfn]
///
/// [whatwg-infra-dfn]: https://infra.spec.whatwg.org/#noncharacter
/// [unicode-s3-2]: https://www.unicode.org/versions/Unicode15.0.0/ch03.pdf#page=8
/// [unicode-s3-4]: https://www.unicode.org/versions/Unicode15.0.0/ch03.pdf#page=19
///
/// # Examples
/// ```
/// use whatwg_infra::scalar::is_noncharacter;
///
/// assert!(is_noncharacter('\u{FDD0}'));
/// assert!(is_noncharacter('\u{FDD1}'));
/// assert!(is_noncharacter('\u{FFFE}'));
/// assert!(is_noncharacter('\u{10FFFF}'));
/// ```
#[allow(clippy::wrong_self_convention)]
#[rustfmt::skip]
#[must_use]
#[inline]
pub const fn is_noncharacter(c: char) -> bool {
	matches!(c,
		| '\u{FDD0}'..='\u{FDEF}'
		| '\u{FFFE}' | '\u{FFFF}' | '\u{1FFFE}' | '\u{1FFFF}'
		| '\u{2FFFE}' | '\u{2FFFF}' | '\u{3FFFE}' | '\u{3FFFF}'
		| '\u{4FFFE}' | '\u{4FFFF}' | '\u{5FFFE}' | '\u{5FFFF}'
		| '\u{6FFFE}' | '\u{6FFFF}' | '\u{7FFFE}' | '\u{7FFFF}'
		| '\u{8FFFE}' | '\u{8FFFF}' | '\u{9FFFE}' | '\u{9FFFF}'
		| '\u{AFFFE}' | '\u{AFFFF}' | '\u{BFFFE}' | '\u{BFFFF}'
		| '\u{CFFFE}' | '\u{CFFFF}' | '\u{DFFFE}' | '\u{DFFFF}'
		| '\u{EFFFE}' | '\u{EFFFF}' | '\u{FFFFE}' | '\u{FFFFF}'
		| '\u{10FFFE}' | '\u{10FFFF}'
	)
}

/// Checks if a character is a **C0 control**, as originally defined
/// by the ANSI X3.4 standard, and redefined by the
/// [WHATWG Infra Standard][whatwg-infra-dfn].
///
/// Any character is a C0 control if it's within the inclusive range
/// of U+0000 NULL or U+001F INFORMATION SEPARATOR ONE.
///
/// This method is subtly different than [`char::is_ascii_control()`] and
/// [`u8::is_ascii_control()`] which also checks for U+007F DELETE.
///
/// See also: [WHATWG Infra Standard definition][whatwg-infra-dfn]
///
/// [whatwg-infra-dfn]: https://infra.spec.whatwg.org/#c0-control
///
/// # Examples
/// ```
/// use whatwg_infra::scalar::is_c0_control;
///
/// assert!(is_c0_control('\u{0000}'));
/// assert!(is_c0_control('\u{001E}'));
/// assert!(is_c0_control('\u{001F}'));
/// ```
#[allow(clippy::wrong_self_convention)]
#[must_use]
#[inline]
pub const fn is_c0_control(c: char) -> bool {
	c <= '\u{001F}'
}

/// Checks if a character is a **C0 control** or space (U+0020 SPACE).
///
/// See also: [WHATWG Infra Standard definition][whatwg-infra-dfn]
///
/// [whatwg-infra-dfn]: https://infra.spec.whatwg.org/#c0-control-or-space
///
/// # Examples
/// ```
/// use whatwg_infra::scalar::is_c0_control_space;
///
/// assert!(is_c0_control_space(' '));
/// assert!(is_c0_control_space('\u{0019}'));
/// ```
#[allow(clippy::wrong_self_convention)]
#[must_use]
#[inline]
pub const fn is_c0_control_space(c: char) -> bool {
	c <= '\u{0020}'
}

/// Checks if a codepoint is equivalent to one of three ASCII whitespace codepoints
/// * U+0009 TAB
/// * U+000A LINE FEED (LF)
/// * U+000D CARRIAGE RETURN (CR)
///
/// See also: [WHATWG Infra Standard definition][whatwg-infra-dfn]
///
/// [whatwg-infra-dfn]: https://infra.spec.whatwg.org/#ascii-tab-or-newline
///
/// # Examples
/// ```
/// use whatwg_infra::scalar::is_ascii_tab_newline;
///
/// assert!(is_ascii_tab_newline('\t'));
/// assert!(is_ascii_tab_newline('\r'));
/// assert!(is_ascii_tab_newline('\n'));
/// assert!(!is_ascii_tab_newline('a'));
/// ```
#[allow(clippy::wrong_self_convention)]
#[must_use]
#[inline]
pub const fn is_ascii_tab_newline(c: char) -> bool {
	matches!(c, '\u{0009}' | '\u{000A}' | '\u{000D}')
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_is_noncharacter() {
		assert!(is_noncharacter('\u{FDD0}'));
		assert!(is_noncharacter('\u{FDD1}'));
		assert!(is_noncharacter('\u{FFFE}'));
		assert!('\u{10FFFF}'.is_noncharacter());
	}

	#[test]
	fn test_is_c0_control() {
		assert!('\u{0000}'.is_c0_control());
		assert!('\u{001E}'.is_c0_control());
		assert!(is_c0_control('\u{001F}'));
	}

	#[test]
	fn test_is_c0_control_space() {
		assert!(is_c0_control_space(' '));
		assert!('\u{0019}'.is_c0_control_space());
	}

	#[test]
	fn test_is_ascii_tab_newline() {
		assert!(is_ascii_tab_newline('\t'));
		assert!(is_ascii_tab_newline('\r'));
		assert!('\n'.is_ascii_tab_newline());
		assert!(!is_ascii_tab_newline('a'));
	}
}
