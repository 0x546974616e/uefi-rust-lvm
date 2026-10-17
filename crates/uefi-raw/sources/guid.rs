use core::fmt;

/// 128-bit buffer containing a unique identifier value.
///
/// Unless otherwise specified, aligned on a 64-bit boundary.
#[doc = ""]
#[doc = crate::uefi_version!(2.11, "2.3.1")]
#[repr(C)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct EFI_GUID {
  /// The first eight hexadecimal digits.
  pub Data1: u32,

  /// The second group of four hexadecimal digits.
  pub Data2: u16,

  /// The third group of four hexadecimal digits.
  pub Data3: u16,

  /// The final two groups of four and twelve hexadecimal digits.
  pub Data4: [u8; 8],
}

impl EFI_GUID {
  /// Creates a `EFI_GUID` represented by the all-zero byte-pattern.
  pub const fn zeroed() -> Self {
    EFI_GUID {
      Data1: 0,
      Data2: 0,
      Data3: 0,
      Data4: [0, 0, 0, 0, 0, 0, 0, 0],
    }
  }

  /// Creates a `EFI_GUID` with the given constant values.
  ///
  /// # Example
  ///
  /// ```rust
  /// use uefi_raw::EFI_GUID;
  /// let _guid = EFI_GUID::from_values(
  ///   (0x12345678, 0x9ABC, 0xDEF0),
  ///   [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0],
  /// );
  /// ```
  pub const fn from_values(data: (u32, u16, u16), data4: [u8; 8]) -> Self {
    EFI_GUID {
      Data1: data.0,
      Data2: data.1,
      Data3: data.2,
      Data4: data4,
    }
  }

  /// Creates an `EFI_GUID` from a `u128` value.
  ///
  /// The most significant bits populate `Data1`, `Data2`, and `Data3`. The
  /// remaining 64 bits populate `Data4` in big-endian byte order.
  ///
  /// # Example
  ///
  /// ```rust
  /// use uefi_raw::EFI_GUID;
  /// let guid = EFI_GUID::from_u128(0x123456789ABCDEF0123456789ABCDEF0);
  ///
  /// assert_eq!(
  ///   guid,
  ///   EFI_GUID::from_values(
  ///     (0x12345678, 0x9ABC, 0xDEF0),
  ///     [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0],
  ///   ),
  /// );
  /// ```
  pub const fn from_u128(uuid: u128) -> Self {
    EFI_GUID {
      Data1: (uuid >> 96) as u32,
      Data2: ((uuid >> 80) & 0xffff) as u16,
      Data3: ((uuid >> 64) & 0xffff) as u16,
      Data4: (uuid as u64).to_be_bytes(),
    }
  }

  /// Converts an `EFI_GUID` to a `u128` value.
  ///
  /// # Example
  ///
  /// ```rust
  /// use uefi_raw::EFI_GUID;
  /// let guid = EFI_GUID::from_values(
  ///   (0x12345678, 0x9ABC, 0xDEF0),
  ///   [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0],
  /// );
  ///
  /// assert_eq!(guid.to_u128(), 0x123456789ABCDEF0123456789ABCDEF0);
  /// ```
  pub const fn to_u128(&self) -> u128 {
    ((self.Data1 as u128) << 96)
      | ((self.Data2 as u128) << 80)
      | ((self.Data3 as u128) << 64)
      | u64::from_be_bytes(self.Data4) as u128
  }
}

impl fmt::Debug for EFI_GUID {
  /// Formats the GUID as an uppercase hexadecimal string.
  ///
  /// # Example
  ///
  /// ```rust
  /// use uefi_raw::EFI_GUID;
  /// let guid: EFI_GUID = 0x123456789ABCDEF0123456789ABCDEF0.into();
  /// assert_eq!(format!("{guid:?}"), "12345678-9ABC-DEF0-1234-56789ABCDEF0");
  /// ```
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(
      formatter,
      "{:08X?}-{:04X?}-{:04X?}-{:02X?}{:02X?}-{:02X?}{:02X?}{:02X?}{:02X?}{:02X?}{:02X?}",
      self.Data1,
      self.Data2,
      self.Data3,
      self.Data4[0],
      self.Data4[1],
      self.Data4[2],
      self.Data4[3],
      self.Data4[4],
      self.Data4[5],
      self.Data4[6],
      self.Data4[7]
    )
  }
}

impl fmt::Display for EFI_GUID {
  /// Formats the GUID with curly braces.
  ///
  /// # Example
  ///
  /// ```rust
  /// use uefi_raw::EFI_GUID;
  /// let guid: EFI_GUID = 0x123456789ABCDEF0123456789ABCDEF0.into();
  /// assert_eq!(guid.to_string(), "{12345678-9ABC-DEF0-1234-56789ABCDEF0}");
  /// ```
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(formatter, "{{{self:?}}}")
  }
}

impl From<u128> for EFI_GUID {
  fn from(value: u128) -> Self {
    EFI_GUID::from_u128(value)
  }
}

impl From<EFI_GUID> for u128 {
  fn from(value: EFI_GUID) -> Self {
    value.to_u128()
  }
}
