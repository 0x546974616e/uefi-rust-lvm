//! Unicode utilities.
//!
//! [`str::encode_utf16()`] and [`str::chars()`] are not `const` (yet?).
use core::{error, fmt};

pub type Utf8Result<T> = Result<T, Utf8Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Utf8Error {
  EmptyInput,
  IncompleteSequence,
  InvalidLeadingByte,
  InvalidContinuationByte,
  OverlongEncoding,
  SurrogateCodePoint,
  CodePointOutOfRange,
}

impl error::Error for Utf8Error {}
impl fmt::Display for Utf8Error {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Debug::fmt(self, formatter)
  }
}

/// Decodes the first Unicode code point from a UTF-8 byte slice.
///
/// # Description
///
/// Returns the decoded Unicode code point and the number of bytes consumed.
/// The function validates UTF-8 encoding rules, including continuation bytes,
/// overlong encodings, UTF-16 surrogate code points, and the Unicode code point
/// range (`U+0000..=U+10FFFF`).
///
/// # Example
///
/// ```rust
/// # use uefi_string::decode_utf8_char;
/// assert_eq!(decode_utf8_char("😀".as_bytes(), 0), Ok('😀'));
/// ```
pub const fn decode_utf8_char(
  bytes: &[u8],
  position: usize,
) -> Utf8Result<char> {
  if bytes.len() < position + 1 {
    return Err(Utf8Error::EmptyInput);
  }

  // 1-byte UTF-8: U+0000-U+007F
  // ┌──────────┐
  // │ 0xxxxxxx │
  // └──────────┘
  //  0x00..0x7F
  let c0 = bytes[position] as u32;
  if c0 <= 0x7F {
    return Ok(unsafe { char::from_u32_unchecked(c0) });
  }

  // 2-byte UTF-8: U+0080-U+07FF
  // ┌──────────┬──────────┐
  // │ 110XXXxx │ 10yyyyyy │
  // └──────────┴──────────┘
  //  0xC0..0xDF 0x80..0xBF
  if matches!(c0, 0xC0..=0xDF) {
    if bytes.len() < position + 2 {
      return Err(Utf8Error::IncompleteSequence);
    }

    let c1 = bytes[position + 1] as u32;
    if (c1 & 0xC0) != 0x80 {
      return Err(Utf8Error::InvalidContinuationByte);
    }

    // Reject overlong encodings:
    // ┌──────────┬──────────┐
    // │ 1100000x │ 10yyyyyy │
    // └──────────┴──────────┘
    //  0xC0..0xC1
    if c0 <= 0xC1 {
      return Err(Utf8Error::OverlongEncoding);
    }

    // ┌──────────┬──────────┬──────────┬──────────┐
    // │ ........ │ ........ │ .....111 │ 11...... │ C0 = 0x1F << 6
    // │ ........ │ ........ │ ........ │ ..111111 │ C1 = 0x3F
    // ├──────────┼──────────┼──────────┼──────────┤
    // │ ........ │ ........ │ .....XXX │ xxyyyyyy │
    // └──────────┴──────────┴──────────┴──────────┘
    return Ok(unsafe {
      char::from_u32_unchecked(((c0 & 0x1F) << 6) | (c1 & 0x3F))
    });
  }

  // 3-byte UTF-8: U+0800-U+FFFF
  // ┌──────────┬──────────┬──────────┐
  // │ 1110xxxx │ 10YYYYyy │ 10zzzzzz │
  // └──────────┴──────────┴──────────┘
  //  0xE0..0xEF 0x80..0xBF 0x80..0xBF
  if matches!(c0, 0xE0..=0xEF) {
    if bytes.len() < position + 3 {
      return Err(Utf8Error::IncompleteSequence);
    }

    let c1 = bytes[position + 1] as u32;
    let c2 = bytes[position + 2] as u32;
    if (c1 & 0xC0) != 0x80 || (c2 & 0xC0) != 0x80 {
      return Err(Utf8Error::InvalidContinuationByte);
    }

    // Reject overlong encodings:
    // ┌──────────┬──────────┬──────────┐
    // │ 11100000 │ 100YYYyy │ 10zzzzzz │
    // └──────────┴──────────┴──────────┘
    //  0xE0..0xE0 0x80..0x9F
    if c0 == 0xE0 && c1 <= 0x9F {
      return Err(Utf8Error::OverlongEncoding);
    }

    // Reject UTF-8 encodings of UTF-16 surrogates:
    // (UTF-16 surrogates = U+D800..U+DFFF)
    // ┌──────────┬──────────┬──────────┐
    // │ 11101101 │ 10YYYYyy │ 10zzzzzz │
    // └──────────┴──────────┴──────────┘
    //  0xED..0xED 0xA0..0xBF
    if c0 == 0xED && c1 >= 0xA0 {
      return Err(Utf8Error::SurrogateCodePoint);
    }

    // ┌──────────┬──────────┬──────────┬──────────┐
    // │ ........ │ ........ │ 1111.... │ ........ │ C0 = 0x0F << 12
    // │ ........ │ ........ │ ....1111 │ 11...... │ C1 = 0x3F << 6
    // │ ........ │ ........ │ ........ │ ..111111 │ C2 = 0x3F
    // ├──────────┼──────────┼──────────┼──────────┤
    // │ ........ │ ........ │ xxxxYYYY │ yyzzzzzz │
    // └──────────┴──────────┴──────────┴──────────┘
    return Ok(unsafe {
      char::from_u32_unchecked(
        ((c0 & 0x0F) << 12) | ((c1 & 0x3F) << 6) | (c2 & 0x3F),
      )
    });
  }

  // 4-byte UTF-8: U+10000-U+10FFFF
  // ┌──────────┬──────────┬──────────┬──────────┐
  // │ 11110WWW │ 10XXxxxx │ 10YYYYyy │ 10zzzzzz │
  // └──────────┴──────────┴──────────┴──────────┘
  //  0xF0..0xF4 0x80..0xBF 0x80..0xBF 0x80..0xBF
  if matches!(c0, 0xF0..=0xF4) {
    if bytes.len() < position + 4 {
      return Err(Utf8Error::IncompleteSequence);
    }

    let c1 = bytes[position + 1] as u32;
    let c2 = bytes[position + 2] as u32;
    let c3 = bytes[position + 3] as u32;
    if (c1 & 0xC0) != 0x80 || (c2 & 0xC0) != 0x80 || (c3 & 0xC0) != 0x80 {
      return Err(Utf8Error::InvalidContinuationByte);
    }

    // Reject overlong encodings:
    // ┌──────────┬──────────┬──────────┬──────────┐
    // │ 11110000 │ 1000xxxx │ 10YYYYyy │ 10zzzzzz │
    // └──────────┴──────────┴──────────┴──────────┘
    //  0xF0..0xF0 0x80..0x8F
    if c0 == 0xF0 && c1 <= 0x8F {
      return Err(Utf8Error::OverlongEncoding);
    }

    // Reject code points above U+10FFFF:
    // ┌──────────┬──────────┬──────────┬──────────┐
    // │ 11110100 │ 10001111 │ 10YYYYyy │ 10zzzzzz │
    // └──────────┴──────────┴──────────┴──────────┘
    //  0xF4..0xF4 0x8F..0x8F
    if c0 == 0xF4 && c1 > 0x8F {
      return Err(Utf8Error::CodePointOutOfRange);
    }

    // ┌──────────┬──────────┬──────────┬──────────┐
    // │ ........ │ ...111.. │ ........ │ ........ │ C0 = 0x07 << 18
    // │ ........ │ ......11 │ 1111.... │ ........ │ C1 = 0x3F << 12
    // │ ........ │ ........ │ ....1111 │ 11...... │ C2 = 0x3F << 6
    // │ ........ │ ........ │ ........ │ ..111111 │ C3 = 0x3F
    // ├──────────┼──────────┼──────────┼──────────┤
    // │ ........ │ ...WWWXX │ xxxxYYYY │ yyzzzzzz │
    // └──────────┴──────────┴──────────┴──────────┘
    return Ok(unsafe {
      char::from_u32_unchecked(
        ((c0 & 0x07) << 18)
          | ((c1 & 0x3F) << 12)
          | ((c2 & 0x3F) << 6)
          | (c3 & 0x3F),
      )
    });
  }

  Err(Utf8Error::InvalidLeadingByte)
}

/// Decodes the first Unicode code point, panicking on failure.
///
/// # Description
///
/// Returns the decoded Unicode code point as a `u32` and the number of bytes
/// consumed (1–4). Any remaining bytes are left unconsumed.
///
/// # Panics
///
/// Panics if the input is empty, the UTF-8 sequence is incomplete, a leading or
/// continuation byte is invalid, the encoding is overlong, the sequence encodes
/// a UTF-16 surrogate code point, or the decoded value exceeds `U+10FFFF`. The
/// panic message identifies the reason for the failure.
///
/// # Example
///
/// ```rust,should_panic
/// # use uefi_string::decode_utf8_char_or_panic;
/// decode_utf8_char_or_panic(&[0xFF], 0);
/// ```
#[track_caller]
pub const fn decode_utf8_char_or_panic(bytes: &[u8], position: usize) -> char {
  match decode_utf8_char(bytes, position) {
    Ok(value) => value,
    Err(Utf8Error::EmptyInput) => {
      panic!("The input slice is empty.")
    }
    Err(Utf8Error::IncompleteSequence) => panic!(
      "The input does not contain enough bytes to complete the UTF-8 sequence."
    ),
    Err(Utf8Error::InvalidLeadingByte) => {
      panic!("The first byte cannot start a valid UTF-8 sequence.")
    }
    Err(Utf8Error::InvalidContinuationByte) => {
      panic!("A continuation byte is invalid.")
    }
    Err(Utf8Error::OverlongEncoding) => {
      panic!("The code point is encoded using more bytes than necessary.")
    }
    Err(Utf8Error::SurrogateCodePoint) => {
      panic!("The sequence encodes a UTF-16 surrogate code point.")
    }
    Err(Utf8Error::CodePointOutOfRange) => {
      panic!("The sequence encodes a value greater than U+10FFFF.")
    }
  }
}

/// Returns the number of UTF-16 code units required to represent a UTF-8 byte
/// slice.
///
/// # Panics
///
/// Panics if bytes contains invalid or incomplete UTF-8.
///
/// # Examples
///
/// ```rust
/// # use uefi_string::utf16_length_or_panic;
/// assert_eq!(utf16_length_or_panic("Hello".as_bytes()), 5);
/// assert_eq!(utf16_length_or_panic("😀".as_bytes()), 2);
/// assert_eq!(utf16_length_or_panic("".as_bytes()), 0);
/// ```
#[track_caller]
pub const fn utf16_length_or_panic(bytes: &[u8]) -> usize {
  let (mut utf16_length, mut position) = (0usize, 0usize);

  while position < bytes.len() {
    let codepoint = decode_utf8_char_or_panic(bytes, position);
    // Characters outside the BMP are represented using a UTF-16 surrogate pair.
    utf16_length += codepoint.len_utf16();
    position += codepoint.len_utf8();
  }

  utf16_length
}

/// Converts a UTF-8 byte slice into UTF-16 code units.
///
/// # Description
///
/// Writes the UTF-16 representation of the input into the provided output
/// buffer. This function does not append a null terminator.
///
/// # Panics
///
/// Panics if the input contains invalid or incomplete UTF-8, or if the output
/// buffer is too small to hold the UTF-16 representation of the input.
///
/// # Example
///
/// ```rust
/// # use uefi_string::*;
/// let input = "😀".as_bytes();
/// let mut output = [0u16; 2];
///
/// assert_eq!(utf16_length_or_panic(input), 2);
/// convert_utf8_to_utf16_or_panic(input, &mut output);
///
/// assert_eq!(output, [0xD83D, 0xDE00]);
/// ```
#[track_caller]
pub const fn convert_utf8_to_utf16_or_panic(input: &[u8], output: &mut [u16]) {
  let (mut input_position, mut output_position) = (0usize, 0usize);
  let mut utf16_buffer = [0x0u16; char::MAX_LEN_UTF16];

  while input_position < input.len() {
    let codepoint = decode_utf8_char_or_panic(input, input_position);
    let (mut i, bytes) = (0, codepoint.encode_utf16(&mut utf16_buffer));
    while i < bytes.len() {
      output[output_position] = bytes[i];
      output_position += 1;
      i += 1;
    }

    input_position += codepoint.len_utf8();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn decodes_single_byte_utf8() {
    assert_eq!(decode_utf8_char("AA".as_bytes(), 1), Ok('A'));
  }

  #[test]
  fn decodes_two_byte_utf8() {
    assert_eq!(decode_utf8_char("éé".as_bytes(), 2), Ok('é'));
  }

  #[test]
  fn decodes_three_byte_utf8() {
    assert_eq!(decode_utf8_char("小小".as_bytes(), 3), Ok('小'));
  }

  #[test]
  fn decodes_four_byte_utf8() {
    assert_eq!(decode_utf8_char("😀😀".as_bytes(), 4), Ok('😀'));
  }

  #[test]
  fn rejects_empty_input() {
    assert_eq!(decode_utf8_char(&[], 0), Err(Utf8Error::EmptyInput));
  }

  #[test]
  fn rejects_invalid_leading_byte() {
    assert_eq!(
      decode_utf8_char(&[0xFF], 0),
      Err(Utf8Error::InvalidLeadingByte)
    );
  }

  #[test]
  fn rejects_incomplete_sequence() {
    assert_eq!(
      decode_utf8_char(&[0xC2], 0),
      Err(Utf8Error::IncompleteSequence)
    );
  }

  #[test]
  fn rejects_invalid_continuation_byte() {
    assert_eq!(
      decode_utf8_char(&[0xC2, 0x20], 0),
      Err(Utf8Error::InvalidContinuationByte)
    );
  }

  #[test]
  fn rejects_overlong_encoding() {
    assert_eq!(
      decode_utf8_char(&[0xE0, 0x80, 0x80], 0),
      Err(Utf8Error::OverlongEncoding)
    );
  }

  #[test]
  fn rejects_surrogate_code_point() {
    assert_eq!(
      decode_utf8_char(&[0xED, 0xA0, 0x80], 0),
      Err(Utf8Error::SurrogateCodePoint)
    );
  }

  #[test]
  fn rejects_code_point_out_of_range() {
    assert_eq!(
      decode_utf8_char(&[0xF4, 0x90, 0x80, 0x80], 0),
      Err(Utf8Error::CodePointOutOfRange)
    );
  }
}
