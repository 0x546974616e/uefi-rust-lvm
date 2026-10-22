#![cfg_attr(not(test), no_std)]
pub(crate) mod unicode;

pub use unicode::*;

/// Converts a string literal into a null-terminated UTF-16 string at compile
/// time.
///
/// # Example
///
/// ```rust
/// # use uefi_string::L;
/// # use core::slice::from_raw_parts;
/// let wide_ptr: *const u16 = L!("Hi");
/// let expected = [0x48u16, 0x69u16, 0u16];
/// assert_eq!(unsafe { from_raw_parts(wide_ptr, 3) }, expected)
/// ```
#[macro_export]
macro_rules! L {
  ($STRING:literal) => {{
    const INPUT: &[u8] = $STRING.as_bytes();
    const OUTPUT_LENGTH: usize = $crate::utf16_length_or_panic(INPUT) + 1;
    const OUTPUT: &[u16; OUTPUT_LENGTH] = {
      let mut output = [0u16; OUTPUT_LENGTH];
      $crate::convert_utf8_to_utf16_or_panic(INPUT, {
        let coercion_workaround: &mut [u16] = &mut output;
        coercion_workaround
      });
      &{ output }
    };
    OUTPUT.as_ptr()
  }};
}
