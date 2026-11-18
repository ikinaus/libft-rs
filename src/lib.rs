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
	if matches!(c, b'A'..=b'Z') {
		c - b'A' + b'a'
	} else {
		c
	}
}

pub fn ft_strlen(s: &[u8]) -> usize {
	let mut i: usize = 0;
	while (i < s.len()) && (s[i] != b'\0') {
		i += 1;
	}
	i
}

pub fn ft_memset(s: &mut [u8], c: u8) {
	let mut i: usize = 0;
	while i < s.len() {
		s[i] = c;
		i += 1;
	}
}

pub fn ft_bzero(s: &mut [u8]) {
	ft_memset(s, 0);
}

pub fn ft_memcpy(dst: &mut [u8], src: &[u8]) {
	let mut i: usize = 0;
	while i < src.len() {
		dst[i] = src[i];
		i += 1;
	}
}

pub fn ft_memmove(buf: &mut [u8], dst: usize, src: usize, n: usize) {
	let mut i: isize = 0;
	let mut incr: isize = 1;

	if dst > src {
		i = n as isize - 1;
		incr = -1;
	}

	while (i >= 0) && (i < (n as isize)) {
		buf[dst + (i as usize)] = buf[src + (i as usize)];
		i += incr;
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

	#[test]
	fn test_strlen() {
		assert_eq!(ft_strlen(b"hello"), 5);
		assert_eq!(ft_strlen(b"hi\0xy"), 2);
		assert_eq!(ft_strlen(b""), 0);
		assert_eq!(ft_strlen(b"\0"), 0);
	}

	#[test]
	fn test_memset() {
		let mut buf = [0u8; 5];
		ft_memset(&mut buf, b'x');
		assert_eq!(buf, [b'x'; 5]);
		buf = [0; 5];
		ft_memset(&mut buf[1..3], b'x');
		assert_eq!(buf, [0, b'x', b'x', 0, 0]);
		buf = [0; 5];
		ft_memset(&mut buf[..0], b'x');
		assert_eq!(buf, [0; 5]);
	}

	#[test]
	fn test_bzero() {
		let mut buf = [1u8; 5];
		ft_bzero(&mut buf);
		assert_eq!(buf, [0; 5]);
	}

	#[test]
	fn test_memcpy() {
		let mut dst = [0u8; 5];
		ft_memcpy(&mut dst, b"abcde");
		assert_eq!(dst, *b"abcde");
		let mut dst = [0u8; 5];
		ft_memcpy(&mut dst[1..3], b"ab");
		assert_eq!(dst, [0, b'a', b'b', 0, 0]);
		let mut dst = [0u8; 5];
		ft_memcpy(&mut dst, b"");
		assert_eq!(dst, [0u8; 5]);
		// let mut dst = [0u8; 5];
		// ft_memcpy(&mut dst, b"asdasdasdas");
		// assert_eq!(dst, [0u8; 5]);
	}

	#[test]
	fn test_memmove() {
		let mut buf = [1u8, 2, 3, 4, 5];
		ft_memmove(&mut buf, 1, 0, 4);
		assert_eq!(buf, [1u8, 1, 2, 3, 4]);
	}
}
