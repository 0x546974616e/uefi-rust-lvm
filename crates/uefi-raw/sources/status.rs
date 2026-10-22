use core::fmt;

/// Status code.
///
/// EFI interfaces return an `EFI_STATUS` code. See [`EFI_STATUS` Success
/// Codes (High Bit Clear)](#), [`EFI_STATUS` Error Codes (High Bit Set)](#),
/// [`EFI_STATUS` Warning Codes (High Bit Clear)](#) list these codes for
/// success, errors, and warnings, respectively. The range of status codes
/// that have the highest bit set and the next to highest bit clear are
/// reserved for use by EFI. The range of status codes that have both the
/// highest bit set and the next to highest bit set are reserved for use by
/// OEMs. Success and warning codes have their highest bit clear, so all
/// success and warning codes have positive values. The range of status codes
/// that have both the highest bit clear and the next to highest bit clear are
/// reserved for use by EFI. The range of status code that have the highest
/// bit clear and the next to highest bit set are reserved for use by OEMs.
/// The Table below lists the status code ranges described above.
#[doc = ""]
#[doc = uefi_version!(2.11, "D")]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct EFI_STATUS(pub usize);

impl EFI_STATUS {
  /// The highest bit of the status value, used to identify errors.
  const MAX_BIT: usize = 1 << (usize::BITS - 1);

  /// Creates an error status (high bit set).
  const fn Error(value: usize) -> EFI_STATUS {
    EFI_STATUS(Self::MAX_BIT | value)
  }

  /// Creates a warning status (high bit clear).
  const fn Warning(value: usize) -> EFI_STATUS {
    EFI_STATUS(value)
  }

  /// Returns true if the status indicates success.
  ///
  /// In UEFI, a status value of zero represents success.
  ///
  /// # Example
  ///
  /// ```rust,no_run
  /// # use uefi_raw::{EFI_SYSTEM_TABLE, EFI_INPUT_KEY};
  /// # let system_table: EFI_SYSTEM_TABLE = unimplemented!();
  /// let mut key = EFI_INPUT_KEY::default();
  /// let status = unsafe { system_table.ConIn().ReadKeyStroke(&mut key) };
  /// if status.is_success() {
  ///   // The key was read successfully.
  /// }
  /// ```
  pub const fn is_success(self) -> bool {
    self.0 == 0
  }

  /// Returns true if the status represents a warning.
  ///
  /// A warning has a nonzero value with the highest bit clear. Warnings
  /// indicate that an operation completed with a condition the caller may need
  /// to handle.
  ///
  /// # Example
  ///
  /// ```rust
  /// # use uefi_raw::{EFI_STATUS, EFI_WARN_BUFFER_TOO_SMALL};
  /// let status: EFI_STATUS = EFI_WARN_BUFFER_TOO_SMALL;
  /// assert!(!status.is_success());
  /// assert!(status.is_warning());
  /// assert!(!status.is_error());
  /// ```
  pub const fn is_warning(self) -> bool {
    self.0 > 0 && self.0 < Self::MAX_BIT
  }

  /// Returns true if the status represents an error.
  ///
  /// An error has the highest bit set. The operation may have failed, so the
  /// caller should handle the error before continuing.
  ///
  /// # Example
  ///
  /// ```rust,no_run
  /// # use uefi_raw::EFI_SYSTEM_TABLE;
  /// # let system_table: EFI_SYSTEM_TABLE = unimplemented!();
  /// let status = unsafe { system_table.ConOut().ClearScreen() };
  /// if status.is_error() {
  ///   // Handle the error.
  /// }
  /// ```
  pub const fn is_error(self) -> bool {
    self.0 >= Self::MAX_BIT
  }

  /// Returns if the status indicates success or a warning.
  ///
  /// Because this function may panic, its use is generally discouraged.  Panics
  /// are meant for unrecoverable errors, and [may abort the entire
  /// program][panic-abort].  Instead, prefer to use [`is_success()`],
  /// [`is_warning()`] or [`is_error()`].
  ///
  /// [panic-abort]: https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html
  /// [`is_success()`]: Self::is_success
  /// [`is_warning()`]: Self::is_warning
  /// [`is_error()`]: Self::is_error
  ///
  /// # Panics
  ///
  /// Panics if [`is_error()`] returns `true`. The panic message includes the
  /// human-readable description of the error when the status code is known, or
  /// the numeric status value otherwise.
  ///
  /// # Example
  ///
  /// ```rust
  /// # use uefi_raw::EFI_SUCCESS;
  /// let status = EFI_SUCCESS;
  /// status.unwrap();
  /// ```
  ///
  /// ```rust
  /// # use uefi_raw::EFI_WARN_BUFFER_TOO_SMALL;
  /// let status = EFI_WARN_BUFFER_TOO_SMALL;
  /// status.unwrap();
  /// ```
  ///
  /// ```rust,should_panic
  /// # use uefi_raw::EFI_ACCESS_DENIED;
  /// let status = EFI_ACCESS_DENIED;
  /// status.unwrap();
  /// ```
  #[track_caller]
  pub fn unwrap(self) {
    if self.is_error() {
      panic!("UEFI error: {}", self);
    }
  }
}

macro_rules! uefi_status {
  (
    $(
      $(#[doc = $VARIANT_DOC:literal])+
      $VARIANT_NAME:ident =
        $($VARAINT_METHOD:ident($VARIANT_VALUE:literal))?
        $($VARIANT_LITERAL:literal)?
    ),* $(,)?
  ) => {
    $(
      $(#[doc = $VARIANT_DOC])+
      pub const $VARIANT_NAME: EFI_STATUS =
        $(EFI_STATUS::$VARAINT_METHOD($VARIANT_VALUE);)?
        $(EFI_STATUS($VARIANT_LITERAL);)?
    )*

    impl fmt::Debug for EFI_STATUS {
      /// Formats the status using its constant name and numeric value.
      ///
      /// For known error statuses, the high bit is removed from the displayed
      /// value. All other status values are displayed unchanged.
      ///
      /// # Example
      ///
      /// ```rust
      /// # use uefi_raw::{EFI_SUCCESS, EFI_NOT_READY};
      /// assert_eq!(format!("{EFI_SUCCESS:?}"), "EFI_SUCCESS(0)");
      /// assert_eq!(format!("{EFI_NOT_READY:?}"), "EFI_NOT_READY(6)");
      /// ```
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (variant, value) = match self.0 {
          $(x if x == $VARIANT_NAME.0 => (stringify!($VARIANT_NAME),
            $($VARIANT_VALUE)? $($VARIANT_LITERAL)?),)*
          value => ("EFI_STATUS", value),
        };
        write!(f, "{variant}({value})")
      }
    }

    impl fmt::Display for EFI_STATUS {
      /// Formats the status using its human-readable description.
      ///
      /// Unknown status values are formatted as `EFI_STATUS(value)`.
      ///
      /// # Example
      ///
      /// ```rust
      /// # use uefi_raw::EFI_UNSUPPORTED;
      /// assert_eq!(
      ///   format!("{EFI_UNSUPPORTED}"),
      ///   "The operation is not supported (EFI_UNSUPPORTED)"
      /// );
      /// ```
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (variant, reason) = match self.0 {
          $(x if x == $VARIANT_NAME.0 => (stringify!($VARIANT_NAME),
            concat!($($VARIANT_DOC),+).trim_ascii_start()),)*
          value => return write!(f, "EFI_STATUS({value})"),
        };
        write!(f, "{reason} ({variant})")
      }
    }
  };
}

uefi_status! {
  /// The operation completed successfully
  EFI_SUCCESS = 0,

  /// The image failed to load
  EFI_LOAD_ERROR = Error(1),

  /// A parameter was incorrect
  EFI_INVALID_PARAMETER = Error(2),

  /// The operation is not supported
  EFI_UNSUPPORTED = Error(3),

  /// The buffer was not the proper size for the request
  EFI_BAD_BUFFER_SIZE = Error(4),

  /// The buffer is not large enough to hold the requested data
  EFI_BUFFER_TOO_SMALL = Error(5),

  /// There is no data pending upon return
  EFI_NOT_READY = Error(6),

  /// The physical device reported an error while attempting the operation
  EFI_DEVICE_ERROR = Error(7),

  /// The device cannot be written to
  EFI_WRITE_PROTECTED = Error(8),

  /// A resource has run out
  EFI_OUT_OF_RESOURCES = Error(9),

  /// An inconstancy was detected on the file system causing the operating to fail
  EFI_VOLUME_CORRUPTED = Error(10),

  /// There is no more space on the file system
  EFI_VOLUME_FULL = Error(11),

  /// The device does not contain any medium to perform the operation
  EFI_NO_MEDIA = Error(12),

  /// The medium in the device has changed since the last access
  EFI_MEDIA_CHANGED = Error(13),

  /// The item was not found
  EFI_NOT_FOUND = Error(14),

  /// Access was denied
  EFI_ACCESS_DENIED = Error(15),

  /// The server was not found or did not respond to the request
  EFI_NO_RESPONSE = Error(16),

  /// A mapping to a device does not exist
  EFI_NO_MAPPING = Error(17),

  /// The timeout time expired
  EFI_TIMEOUT = Error(18),

  /// The protocol has not been started
  EFI_NOT_STARTED = Error(19),

  /// The protocol has already been started
  EFI_ALREADY_STARTED = Error(20),

  /// The operation was aborted
  EFI_ABORTED = Error(21),

  /// An ICMP error occurred during the network operation
  EFI_ICMP_ERROR = Error(22),

  /// A TFTP error occurred during the network operation
  EFI_TFTP_ERROR = Error(23),

  /// A protocol error occurred during the network operation
  EFI_PROTOCOL_ERROR = Error(24),

  /// The function encountered an internal version that was incompatible with a
  /// version requested by the caller
  EFI_INCOMPATIBLE_VERSION = Error(25),

  /// The function was not performed due to a security violation
  EFI_SECURITY_VIOLATION = Error(26),

  /// A CRC error was detected
  EFI_CRC_ERROR = Error(27),

  /// Beginning or end of media was reached
  EFI_END_OF_MEDIA = Error(28),

  /// The end of the file was reached
  EFI_END_OF_FILE = Error(31),

  /// The language specified was invalid
  EFI_INVALID_LANGUAGE = Error(32),

  /// The security status of the data is unknown or compromised and the data
  /// must be updated or replaced to restore a valid security status
  EFI_COMPROMISED_DATA = Error(33),

  /// There is an address conflict address allocation
  EFI_IP_ADDRESS_CONFLICT = Error(34),

  /// A HTTP error occurred during the network operation
  EFI_HTTP_ERROR = Error(35),

  /// The string contained one or more characters that the device could not
  /// render and were skipped
  EFI_WARN_UNKNOWN_GLYPH = Warning(1),

  /// The handle was closed, but the file was not deleted
  EFI_WARN_DELETE_FAILURE = Warning(2),

  /// The handle was closed, but the data to the file was not flushed properly
  EFI_WARN_WRITE_FAILURE = Warning(3),

  /// The resulting buffer was too small, and the data was truncated to the
  /// buffer size
  EFI_WARN_BUFFER_TOO_SMALL = Warning(4),

  /// The data has not been updated within the timeframe set by local policy for
  /// this type of data
  EFI_WARN_STALE_DATA = Warning(5),

  /// The resulting buffer contains UEFI-compliant file system
  EFI_WARN_FILE_SYSTEM = Warning(6),

  /// The operation will be processed across a system reset
  EFI_WARN_RESET_REQUIRED = Warning(7),
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn debug_status() {
    assert_eq!(
      format!("{EFI_BUFFER_TOO_SMALL:?}"),
      "EFI_BUFFER_TOO_SMALL(5)",
    );
  }

  #[test]
  fn debug_unknown() {
    assert_eq!(
      format!("{:?}", EFI_STATUS::Warning(1337)),
      "EFI_STATUS(1337)",
    );
  }

  #[test]
  fn display_status() {
    assert_eq!(
      EFI_INVALID_LANGUAGE.to_string(),
      "The language specified was invalid (EFI_INVALID_LANGUAGE)",
    );
  }

  #[test]
  fn display_unknown() {
    assert_eq!(
      EFI_STATUS::Error(1337).to_string(),
      "EFI_STATUS(9223372036854777145)",
    );
  }
}
