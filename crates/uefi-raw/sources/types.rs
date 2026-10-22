use core::{ffi::c_void, ptr};

uefi_type! {
  #[version(2.11, "2.3.1")]
  /// 4-byte signed value.
  pub type INT32 = i32;
}

uefi_type! {
  #[version(2.11, "2.3.1")]
  /// Unsigned value of native width.
  ///
  /// 4 bytes on supported 32-bit processor instructions, 8 bytes on supported
  /// 64-bit processor instructions, 16 bytes on supported 128-bit processor
  /// instructions.
  pub type UINTN = usize;
}

uefi_type! {
  #[version(2.11, "2.3.1")]
  /// Logical Boolean.
  ///
  /// 1-byte value containing a 0 for **FALSE** or a 1 for **TRUE**. Other
  /// values are undefined.
  pub type BOOLEAN = bool;
}

uefi_type! {
  #[version(2.11, "2.3.1")]
  /// 2-byte Character.
  ///
  /// Unless otherwise specified all characters and strings are stored in the
  /// UCS-2 encoding format as defined by Unicode 2.1 and ISO/IEC 10646
  /// standards.
  pub type CHAR16 = u16;
}

uefi_type! {
  #[version(2.11, "2.3.1")]
  /// Undeclared type.
  pub type VOID = c_void;
}

uefi_type! {
  #[version(2.11, "2.3.1")]
  /// A collection of related interfaces.
  pub type EFI_HANDLE = *const VOID;
}

uefi_type! {
  #[version(2.11, "2.3.1")]
  /// Handle to an event structure.
  pub type EFI_EVENT = *const VOID;
}

/// 😈
// pub const NULL: *const VOID = ptr::null();
#[must_use]
pub const fn NULL<T>() -> *const T {
  ptr::null()
}

uefi_struct! {
  #[version(2.11, "12.3.3")]
  /// The keystroke information for the key that was pressed.
  #[derive(Default)]
  pub struct EFI_INPUT_KEY {
    /// The scan code of the key, used to identify non-printable keys.
    ScanCode: u16,

    /// The Unicode character associated with the key, if applicable.
    UnicodeChar: CHAR16,
  }
}
