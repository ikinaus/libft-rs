pub fn ft_isalpha(c: u8) -> bool {
	matches!(c, b'A'..=b'Z' | b'a'..=b'z')
}

pub fn ft_isdigit(c: u8) -> bool {
	matches!(c, b'0'..=b'9')
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
}
