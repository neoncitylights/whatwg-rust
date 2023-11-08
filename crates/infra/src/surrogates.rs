/// Methods from the WHATWG Infra Standard for UTF-16 surrogates
#[allow(clippy::wrong_self_convention)]
pub trait InfraUtf16Surrogate {
	/// The minimum UTF-16 codepoint that can be represented as a leading surrogate
	const LEADING_SURROGATE_MIN: u16;
	/// The minimum UTF-16 codepoint that can be represented as a leading surrogate
	const LEADING_SURROGATE_MAX: u16;

	/// The minimum UTF-16 codepoint that can be represented as a trailing surrogate
	const TRAILING_SURROGATE_MIN: u16;
	/// The maximum UTF-16 codepoint that can be represented as a trailing surrogate
	const TRAILING_SURROGATE_MAX: u16;

	/// The minimum UTF-16 codepoint that can be represented as a surrogate
	const SURROGATE_MIN: u16;
	/// The maximum UTF-16 codepoint that can be represented as a surrogate
	const SURROGATE_MAX: u16;

	/// See the documentation for [`is_surrogate_utf16()`]
	fn is_surrogate_utf16(self) -> bool;
	/// See the documentation for [`is_leading_surrogate_utf16()`]
	fn is_leading_surrogate_utf16(self) -> bool;
	/// See the documentation for [`is_trailing_surrogate_utf16()`]
	fn is_trailing_surrogate_utf16(self) -> bool;
}

impl InfraUtf16Surrogate for u16 {
	const LEADING_SURROGATE_MIN: u16 = 0xD800u16;
	const LEADING_SURROGATE_MAX: u16 = 0xDBFFu16;
	const TRAILING_SURROGATE_MIN: u16 = 0xDC00u16;
	const TRAILING_SURROGATE_MAX: u16 = 0xDFFFu16;
	const SURROGATE_MIN: u16 = Self::LEADING_SURROGATE_MIN;
	const SURROGATE_MAX: u16 = Self::TRAILING_SURROGATE_MAX;

	fn is_surrogate_utf16(self) -> bool {
		is_surrogate_utf16(self)
	}

	fn is_leading_surrogate_utf16(self) -> bool {
		is_leading_surrogate_utf16(self)
	}

	fn is_trailing_surrogate_utf16(self) -> bool {
		is_trailing_surrogate_utf16(self)
	}
}

/// Checks if a `u16` is a UTF-16 codepoint defined in the range of
/// U+D800 to U+DFFF, inclusive.
///
/// See also: [WHATWG Infra Standard definition][whatwg-infra-dfn]
///
/// **Note**: This function substitutes the nightly-gated feature in Rust,
/// [`utf16_extra`][utf16-extra-feat] ([issue #94919][utf16-extra-gh]).
///
/// [whatwg-infra-dfn]: https://infra.spec.whatwg.org/#surrogate
/// [utf16-extra-gh]: https://github.com/rust-lang/rust/issues/94919
/// [utf16-extra-feat]: https://doc.rust-lang.org/unstable-book/library-features/utf16-extra.html
///
/// # Examples
/// ```
/// use whatwg_infra::surrogates::is_surrogate_utf16;
///
/// assert_eq!(is_surrogate_utf16(0xD799u16), false);
/// assert_eq!(is_surrogate_utf16(0xD809u16), true);
/// assert_eq!(is_surrogate_utf16(0xDB99u16), true);
/// assert_eq!(is_surrogate_utf16(0xDFFFu16), true);
/// assert_eq!(is_surrogate_utf16(0xE000u16), false);
/// ```
#[allow(clippy::wrong_self_convention)]
#[must_use]
#[inline]
pub const fn is_surrogate_utf16(c: u16) -> bool {
	//matches!(c, 0xD800u16..=0xDFFFu16)
	matches!(c, u16::SURROGATE_MIN..=u16::SURROGATE_MAX)
}

/// Checks if a `u16` is a UTF-16 codepoint defined in the range of
/// U+D800 to U+DBFF, inclusive.
///
/// See also: [WHATWG Infra Standard definition][whatwg-infra-dfn]
///
/// [whatwg-infra-dfn]: https://infra.spec.whatwg.org/#leading-surrogate
///
/// # Examples
/// ```
/// use whatwg_infra::surrogates::is_leading_surrogate_utf16;
///
/// assert_eq!(is_leading_surrogate_utf16(0xD799u16), false);
/// assert_eq!(is_leading_surrogate_utf16(0xD800u16), true);
/// assert_eq!(is_leading_surrogate_utf16(0xDBFFu16), true);
/// assert_eq!(is_leading_surrogate_utf16(0xDC00u16), false);
/// ```
#[allow(clippy::wrong_self_convention)]
#[must_use]
#[inline]
pub const fn is_leading_surrogate_utf16(c: u16) -> bool {
	matches!(c, u16::LEADING_SURROGATE_MIN..=u16::LEADING_SURROGATE_MAX)
}

/// Checks if a `u16` is a UTF-16 codepoint defined in the range of
/// U+DC00 to U+DFFF, inclusive.
///
/// See also: [WHATWG Infra Standard definition][whatwg-infra-dfn]
///
/// [whatwg-infra-dfn]: https://infra.spec.whatwg.org/#trailing-surrogate
///
/// # Examples
/// ```
/// use whatwg_infra::surrogates::is_trailing_surrogate_utf16;
///
/// assert_eq!(is_trailing_surrogate_utf16(0xDB99u16), false);
/// assert_eq!(is_trailing_surrogate_utf16(0xDC00u16), true);
/// assert_eq!(is_trailing_surrogate_utf16(0xDFFFu16), true);
/// assert_eq!(is_trailing_surrogate_utf16(0xE000u16), false);
/// ```
#[allow(clippy::wrong_self_convention)]
#[must_use]
#[inline]
pub const fn is_trailing_surrogate_utf16(c: u16) -> bool {
	matches!(c, u16::TRAILING_SURROGATE_MIN..=u16::TRAILING_SURROGATE_MAX)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_is_surrogate_utf16() {
		assert_eq!(is_surrogate_utf16(0xD799u16), false);
		assert_eq!(is_surrogate_utf16(0xD809u16), true);
		assert_eq!(is_surrogate_utf16(0xDB99u16), true);
		assert_eq!(is_surrogate_utf16(0xDFFFu16), true);
		assert_eq!(0xE000u16.is_surrogate_utf16(), false);
	}

	#[test]
	fn test_is_leading_surrogate_utf16() {
		assert_eq!(is_leading_surrogate_utf16(0xD799u16), false);
		assert_eq!(is_leading_surrogate_utf16(0xD800u16), true);
		assert_eq!(is_leading_surrogate_utf16(0xDBFFu16), true);
		assert_eq!(0xDC00u16.is_leading_surrogate_utf16(), false);
	}

	#[test]
	fn test_is_trailing_surrogate_utf16() {
		assert_eq!(is_trailing_surrogate_utf16(0xDB99u16), false);
		assert_eq!(is_trailing_surrogate_utf16(0xDC00u16), true);
		assert_eq!(is_trailing_surrogate_utf16(0xDFFFu16), true);
		assert_eq!(0xE000u16.is_trailing_surrogate_utf16(), false);
	}
}
