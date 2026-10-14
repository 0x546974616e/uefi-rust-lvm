use crate::*;

///  @par Data Structure Description:
///  Mode Structure pointed to by Simple Text Out protocol.
#[repr(C)]
#[allow(nonstandard_style)]
#[derive(Debug)]
pub struct EFI_SIMPLE_TEXT_OUTPUT_MODE {
  /// The number of modes supported by QueryMode () and SetMode ().
  MaxMode: i32,

  /// The text mode of the output device(s).
  Mode: i32,

  /// The current character output attribute.
  Attribute: i32,

  /// The cursor's column.
  CursorColumn: i32,

  /// The cursor's row.
  CursorRow: i32,
  /// The cursor is currently visible or not.
  CursorVisible: BOOLEAN,
}

#[repr(C)]
#[derive(Default)]
pub struct EFI_INPUT_KEY {
  ScanCode: u16,    // UINT16;
  UnicodeChar: u16, // CHAR16;
}

uefi_struct! {
  /// 128-bit buffer containing a unique identifier value.
  ///
  /// Unless otherwise specified, aligned on a 64-bit boundary.
  pub struct EFI_GUID {
    pub Data1: u32,
    pub Data2: u16,
    pub Data3: u16,
    pub Data4: [u8; 8],
  }
}

uefi_struct! {
  pub struct SystemTable {
    header: [u8; 24],
    firmware_vendor: u64,
    firmware_revision: u32,
    input_handle: TodoImageHandle,
    /// dadadada
    input: #[pointer] pub EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    output_handle: TodoImageHandle,
    output: #[pointer] pub EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    error_handle: TodoImageHandle,
    error: #[todo],
    runtime: #[todo],
    boot: #[todo],
    no_of_entries: usize,
    config_table: #[todo],
  }
}
