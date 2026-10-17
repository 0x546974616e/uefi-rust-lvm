use {
  core::fmt,
  uefi_raw::{CHAR16, EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL},
};

pub fn print(
  output_protocol: impl FnOnce(
    &uefi_raw::EFI_SYSTEM_TABLE,
  ) -> *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
  arguments: core::fmt::Arguments<'_>,
  // NOTE: core::format_args_nl!() is unstable.
  newline: bool,
) {
  use core::fmt::Write;
  if let Some(system_table) = unsafe { uefi_raw::SYSTEM_TABLE.as_ref() } {
    if let Some(console) = unsafe { output_protocol(system_table).as_ref() } {
      let mut output = BufferedOutput::new(console);
      let _result = output
        .write_fmt(arguments)
        .and_then(|_| if newline { output.write_char('\n') } else { Ok(()) })
        .and_then(|_| output.flush());
    }
  }
}

/// Buffer capacity, **including** the `NUL` terminator.
const BUFFER_SIZE: usize = 256;

/// Buffered writer for the `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`.
///
/// Text is buffered until the buffer is full or [`BufferedOutput::flush()`] is
/// called. The buffer has a fixed capacity and does not allocate memory.
pub struct BufferedOutput<'st> {
  protocol: &'st EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
  buffer: [CHAR16; BUFFER_SIZE],
  position: usize,
}

impl<'st> BufferedOutput<'st> {
  pub fn new(protocol: &'st EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL) -> Self {
    BufferedOutput {
      buffer: [0u16; BUFFER_SIZE],
      position: 0usize,
      protocol,
    }
  }

  /// Appends one character, replacing non-BMP characters with `?`.
  pub fn push_char(&mut self, mut character: char) -> fmt::Result {
    // UEFI text output traditionally uses UCS-2, which cannot represent
    // characters outside the Basic Multilingual Plane (BMP), i.e., characters
    // above U+FFFF. Supplementary characters belong to the Supplementary Planes
    // and require surrogate pairs in UTF-16. Such characters are therefore
    // replaced with a question mark. (See 2.11 - 33.2.6 Strings for more.)
    if character as u32 > CHAR16::MAX as u32 {
      character = '?';
    }

    // Reserve one slot for the terminating `NUL`.
    if self.position >= BUFFER_SIZE - 1usize {
      self.flush()?;
    }

    *self.buffer.get_mut(self.position).ok_or(fmt::Error)? =
      character as CHAR16;
    self.position += 1;
    Ok(())
  }

  /// Writes the buffered text to the firmware.
  ///
  /// Does nothing if the buffer is empty. Otherwise, appends a `NUL` terminator
  /// and passes the buffer to `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.OutputString()`.
  /// The buffer is cleared only if the firmware call succeeds.
  pub fn flush(&mut self) -> fmt::Result {
    if self.position == 0usize {
      return Ok(());
    }

    // Check the buffer bounds defensively before writing.
    *self.buffer.get_mut(self.position).ok_or(fmt::Error)? = 0x0000u16;

    // SAFETY: The buffer is valid, NUL-terminated and remains alive.
    if unsafe { self.protocol.OutputString(self.buffer.as_ptr()) }.is_error() {
      return Err(fmt::Error);
    }

    self.position = 0;
    Ok(())
  }
}

impl<'st> fmt::Write for BufferedOutput<'st> {
  /// Appends a character to the buffer.
  ///
  /// Embedded `\0` characters are ignored and line feeds (`\n`) are translated
  /// to CRLF (`\r\n`). BMP characters are emitted directly, while non-BMP
  /// characters are replaced with `?` rather than encoded as UTF-16 surrogate
  /// pairs (strings in the UEFI environment are defined using UCS-2).
  fn write_char(&mut self, character: char) -> fmt::Result {
    match character {
      '\0' => Ok(()),
      '\n' => {
        self.push_char('\r')?;
        self.push_char('\n')
      }
      x => self.push_char(x),
    }
  }

  /// Encodes a Rust UTF-8 string into the UCS-2 buffer.
  fn write_str(&mut self, string: &str) -> fmt::Result {
    for character in string.chars() {
      self.write_char(character)?;
    }
    Ok(())
  }
}
