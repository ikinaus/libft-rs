pub fn ft_isalpha(c: u8) -> bool {
	matches!(c, b'A'..=b'Z' | b'a'..=b'z')
}

pub fn ft_isdigit(c: u8) -> bool {
	matches!(c, b'0'..=b'9')
}

pub fn ft_isalnum(c: u8) -> bool {
	ft_isalpha(c) || ft_isdigit(c)
}

pub fn ft_isascii(c: u8) -> bool {
	matches!(c, 0..128)
}

pub fn ft_isprint(c: u8) -> bool {
	matches!(c, 32..=126)
}

pub fn ft_toupper(c: u8) -> u8 {
	if matches!(c, b'a'..=b'z') {
		c - b'a' + b'A'
	} else {
		c
	}
}

pub fn ft_tolower(c: u8) -> u8 {
	if matches!(c, b'a'..=b'z') {
		c - b'A' + b'a'
	} else {
		c
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_isalpha() {
		for c in 0..=255u8 {
			assert_eq!(ft_isalpha(c), c.is_ascii_alphabetic());
		}
	}

	#[test]
	fn test_isdigit() {
		for c in 0..=255u8 {
			assert_eq!(ft_isdigit(c), c.is_ascii_digit());
		}
	}

	#[test]
	fn test_isalnum() {
		for c in 0..=255u8 {
			assert_eq!(ft_isalnum(c), c.is_ascii_alphanumeric());
		}
	}

	#[test]
	fn test_isascii() {
		for c in 0..=255u8 {
			assert_eq!(ft_isascii(c), c.is_ascii());
		}
	}

	#[test]
	fn test_isprint() {
		for c in 0..=255u8 {
			assert_eq!(ft_isprint(c), c.is_ascii_graphic() || c == b' ');
		}
	}

	#[test]
	fn test_toupper() {
		for c in 0..=255u8 {
			assert_eq!(ft_toupper(c), c.to_ascii_uppercase());
		}
	}

	#[test]
	fn test_tolower() {
		for c in 0..=255u8 {
			assert_eq!(ft_tolower(c), c.to_ascii_lowercase());
		}
	}
}
