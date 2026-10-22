use core::{error, fmt};

/// Error type for [`EFI_GUID::try_parse()`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuidParserError {
  /// Input has the wrong length, expected 36 bytes.
  Length(usize),

  /// Input is missing a separator (`-`) at this byte index.
  Separator(usize),

  /// Input contains invalid ASCII hex at this byte index.
  Digit(usize),
}

impl error::Error for GuidParserError {}
impl fmt::Display for GuidParserError {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Debug::fmt(self, formatter)
  }
}

/// Globally-unique identifier.
///
/// # Notes
///
/// GUIDs are mentioned in [Section 2.3.1 -- *Data Types*][S231], and defined in
/// [Section 7.3.2, *InstallProtocolInterface()*][S732]. However, the relevant
/// information is found in [Appendix A -- *GUID and Time Formats*][AppA], of
/// the UEFI Specification, Release 2.11.
///
/// The UEFI Specification describes a GUID as a 128-bit buffer aligned on a
/// 64-bit boundary, but defines its structure, as does the EDK II
/// implementation, as `struct { u32 a; u16 b; u16 c; u8 d[8]; }`, which implies
/// 32-bit alignment. We follow the same layout here for compatibility, as does
/// the Linux Kernel (see `include/linux/efi.h`).
///
/// [S231]: https://uefi.org/specs/UEFI/2.11/02_Overview.html#data-types
/// [S732]: https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-installprotocolinterface
/// [AppA]: https://uefi.org/specs/UEFI/2.10/Apx_A_GUID_and_Time_Formats.html
/// [RFC 4122]: https://datatracker.ietf.org/doc/html/rfc4122
///
/// # Description
///
/// All EFI GUIDs (Globally Unique Identifiers) have the format described in
/// [RFC 4122] and comply with the referenced algorithms for generating GUIDs.
/// It should also be noted that `TimeLow`, `TimeMid`, `TimeHighAndVersion`
/// fields in the EFI are encoded as **little endian**.
///
/// All EFI time information is stored in 64-bit structures that contain the
/// following format: The timestamp is a 60-bit value containing a count of
/// 100-nanosecond intervals since *00:00:00.00, 15 October 1582* (the date of
/// Gregorian reform to the Christian calendar). This time value will not roll
/// over until the year 3400 AD. It is assumed that a future version of the EFI
/// specification can deal with the year-3400 issue by extending this format if
/// necessary.
///
/// This specification also defines a standard text representation of the GUID.
/// This format is also sometimes called the "registry format". It consists of
/// 36 characters, as follows:
///
/// ```txt
/// aabbccdd-eeff-gghh-iijj-kkllmmnnoopp
/// ```
///
/// The pairs *aa* - *pp* are two characters in the range `[0-9A-Fa-f]`, with
/// each pair representing a single byte hexadecimal value. The following table
/// describes the relationship between the text representation and a 16-byte
/// buffer, the structure defined by EDK II (see `edk2/MdePkg/Include/Base.h`)
/// and this `EFI_GUID` structure:
///
/// | String | Buffer |            EFI_GUID            |     EDK II     |
/// |:------:|:------:|--------------------------------|----------------|
/// |   aa   |    3   | `TimeLow[24:31]`               | `Data1[24:31]` |
/// |   bb   |    2   | `TimeLow[16:23]`               | `Data1[16:23]` |
/// |   cc   |    1   | `TimeLow[8:15]`                | `Data1[8:15]`  |
/// |   dd   |    0   | `TimeLow[0:7]`                 | `Data1[0:7]`   |
/// |   ee   |    5   | `TimeMid[8:15]`                | `Data2[8:15]`  |
/// |   ff   |    4   | `TimeMid[0:7]`                 | `Data2[0:7]`   |
/// |   gg   |    7   | `TimeHighAndVersion[8:15]`     | `Data3[8:15]`  |
/// |   hh   |    6   | `TimeHighAndVersion[0:7]`      | `Data3[0:7]`   |
/// |   ii   |    8   | `ClockSeqHighAndReserved[0:7]` | `Data4[0:7]`   |
/// |   jj   |    9   | `ClockSeqLow[0:7]`             | `Data4[8:15]`  |
/// |   kk   |   10   | `Node[0:7]`                    | `Data4[16:23]` |
/// |   ll   |   11   | `Node[8:15]`                   | `Data4[24:31]` |
/// |   mm   |   12   | `Node[16:23]`                  | `Data4[32:39]` |
/// |   nn   |   13   | `Node[24:31]`                  | `Data4[40:47]` |
/// |   oo   |   14   | `Node[32:39]`                  | `Data4[48:55]` |
/// |   pp   |   15   | `Node[40:47]`                  | `Data4[56:63]` |
///
/// # Example
///
/// ```rust
/// # use uefi_raw::EFI_GUID;
/// let guid = EFI_GUID::from_values(
///   (0x12345678, 0x9ABC, 0xDEF0),
///   [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0],
/// );
///
/// assert_eq!(
///   guid,
///   EFI_GUID {
///     TimeLow: [0x78, 0x56, 0x34, 0x12],
///     TimeMid: [0xBC, 0x9A],
///     TimeHighAndVersion: [0xF0, 0xDE],
///     ClockSeqHighAndReserved: 0x12,
///     ClockSeqLow: 0x34,
///     Node: [0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0],
///   },
/// );
/// ```
#[doc = ""]
#[doc = uefi_version!(2.11, "A")]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
#[repr(C, align(4))]
pub struct EFI_GUID {
  /// The low field of the timestamp.
  pub TimeLow: [u8; 4],

  /// The middle field of the timestamp.
  pub TimeMid: [u8; 2],

  /// The high field of the timestamp multiplexed with the version number.
  pub TimeHighAndVersion: [u8; 2],

  /// The high field of the clock sequence multiplexed with the variant.
  pub ClockSeqHighAndReserved: u8,

  /// The low field of the clock sequence.
  pub ClockSeqLow: u8,

  /// The spatially unique node identifier.
  ///
  /// This can be based on any IEEE 802 address obtained from a network card.
  /// If no network card exists in the system, a crypto graphic-quality
  /// random number can be used.
  pub Node: [u8; 6],
}

uefi_assert!(size_of::<EFI_GUID>() == 16);
uefi_assert!(align_of::<EFI_GUID>() == 4);

impl EFI_GUID {
  /// Creates a `EFI_GUID` represented by the all-zero byte-pattern.
  pub const fn zeroed() -> Self {
    unsafe { core::mem::zeroed() }
  }

  /// Creates an `EFI_GUID` from its three timestamp fields and eight-byte data
  /// field.
  ///
  /// # Parameters
  ///
  /// * `timestamp` - The `TimeLow`, `TimeMid`, and `TimeHighAndVersion` fields,
  ///   respectively.
  /// * `data`      - The eight bytes containing the clock sequence and node
  ///   identifier, in order.
  ///
  /// # Description
  ///
  /// The `timestamp` fields are converted to **little-endian** byte arrays,
  /// while `data` is copied directly into the clock sequence and node fields.
  ///
  /// # Example
  ///
  /// ```rust
  /// # use uefi_raw::EFI_GUID;
  /// let guid = EFI_GUID::from_values(
  ///   (0x12345678, 0x9ABC, 0xDEF0),
  ///   [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0],
  /// );
  ///
  /// assert_eq!(guid.to_string(), "12345678-9ABC-DEF0-1234-56789ABCDEF0");
  /// ```
  pub const fn from_values(timestamp: (u32, u16, u16), data: [u8; 8]) -> Self {
    let [ClockSeqHighAndReserved, ClockSeqLow, Node @ ..] = data;

    EFI_GUID {
      TimeLow: timestamp.0.to_le_bytes(),
      TimeMid: timestamp.1.to_le_bytes(),
      TimeHighAndVersion: timestamp.2.to_le_bytes(),
      ClockSeqHighAndReserved,
      ClockSeqLow,
      Node,
    }
  }

  /// Creates an `EFI_GUID` from a `u128` value.
  ///
  /// # Description
  ///
  /// The most significant 32 bits, followed by the next two 16-bit fields, are
  /// used for the timestamp fields and converted to **little-endian** byte
  /// arrays. The least significant 64 bits are converted to **big-endian**
  /// bytes and used for the clock sequence and node fields.
  ///
  /// # Example
  ///
  /// ```rust
  /// # use uefi_raw::EFI_GUID;
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
    EFI_GUID::from_values(
      (
        (uuid >> 96) as u32,
        ((uuid >> 80) & 0xffff) as u16,
        ((uuid >> 64) & 0xffff) as u16,
      ),
      (uuid as u64).to_be_bytes(),
    )
  }

  /// Parses a GUID string in the standard 36-character format.
  ///
  /// # Description
  ///
  /// The input must be in `xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx` format, where
  /// each `x` is any hexadecimal digit. The first three fields are stored in
  /// **little-endian** byte order. The remaining eight bytes are stored in the
  /// order they appear in the string.
  ///
  /// Returns [`GuidParserError`] if the string has an invalid length,
  /// separators, or hexadecimal characters.
  ///
  /// # Example
  ///
  /// ```rust
  /// # use uefi_raw::EFI_GUID;
  /// let guid = EFI_GUID::try_parse("12345678-9ABC-DEF0-1234-56789ABCDEF0");
  /// assert_eq!(guid, Ok(0x123456789ABCDEF0123456789ABCDEF0.into()));
  /// ```
  pub const fn try_parse(string: &str) -> Result<Self, GuidParserError> {
    inner::parse_guid(string)
  }

  /// Parses a GUID from a string, panicking on failure.
  ///
  /// # Description
  ///
  /// The input must be in `xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx` format, where
  /// each `x` is any hexadecimal digit (any of `0-9`, `a-f`, or `A-F`). The
  /// first three fields are stored in **little-endian** byte order. The
  /// remaining eight bytes are stored in the order they appear in the string.
  ///
  /// # Panics
  ///
  /// This function will panic if the input does not match the format described
  /// above. Specifically, it will panic if the input is not exactly 36 bytes
  /// long, if the separators are missing or misplaced, or if any of the
  /// remaining characters are not valid hexadecimal digits.
  ///
  /// # Example
  ///
  /// ```rust,should_panic
  /// # use uefi_raw::EFI_GUID;
  /// let _guid = EFI_GUID::parse_or_panic("1234");
  /// ```
  #[track_caller]
  pub const fn parse_or_panic(string: &str) -> Self {
    match EFI_GUID::try_parse(string) {
      Ok(guid) => guid,
      Err(GuidParserError::Length(_)) => {
        panic!("GUID string has wrong length (expected 36 bytes)");
      }
      Err(GuidParserError::Separator(_)) => {
        panic!("GUID string is missing one or more separators (`-`)");
      }
      Err(GuidParserError::Digit(_)) => {
        panic!("GUID string contains one or more invalid characters");
      }
    }
  }
}

impl fmt::Display for EFI_GUID {
  /// Formats the GUID uppercase hexadecimal digits.
  ///
  /// # Example
  ///
  /// ```rust
  /// use uefi_raw::EFI_GUID;
  /// // See `EFI_GUID::from_u128()` for details.
  /// let guid: EFI_GUID = 0x123456789ABCDEF0123456789ABCDEF0.into();
  /// assert_eq!(format!("{guid:?}"), "12345678-9ABC-DEF0-1234-56789ABCDEF0");
  /// ```
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(
      formatter,
      "{:08X?}-{:04X?}-{:04X?}-{:02X?}{:02X?}-{:02X?}{:02X?}{:02X?}{:02X?}{:02X?}{:02X?}",
      u32::from_le_bytes(self.TimeLow),
      u16::from_le_bytes(self.TimeMid),
      u16::from_le_bytes(self.TimeHighAndVersion),
      self.ClockSeqHighAndReserved,
      self.ClockSeqLow,
      self.Node[0],
      self.Node[1],
      self.Node[2],
      self.Node[3],
      self.Node[4],
      self.Node[5]
    )
  }
}

impl fmt::Debug for EFI_GUID {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    fmt::Display::fmt(self, formatter)
  }
}

impl From<u128> for EFI_GUID {
  fn from(value: u128) -> Self {
    EFI_GUID::from_u128(value)
  }
}

mod inner {
  use super::*;

  const SEPARATOR: u8 = b'-';

  /// Unwraps a result or propagates its error.
  ///
  /// # Notes
  /// *(2026-10-19)*
  ///
  /// - `Try` and `FromResidual` is not yet stable as a const trait.
  ///
  /// - `?` is not allowed on `Result<u8, guid::GuidParserError>` in constant
  ///   functions calls in constant functions are limited to constant functions,
  ///   tuple structs and tuple variants. see issue [#143874] for more
  ///   information.
  ///
  /// [#143874]: https://github.com/rust-lang/rust/issues/143874
  macro_rules! _try {
    ($EXPRESSION:expr) => {
      match $EXPRESSION {
        Ok(value) => value,
        Err(error) => return Err(error),
      }
    };
  }

  /// Returns the value of an ASCII hexadecimal digit, or `None`.
  const fn hex_digit(byte: u8) -> Option<u8> {
    Some(match byte {
      b'0'..=b'9' => byte - b'0',
      b'a'..=b'f' => byte - b'a' + 10,
      b'A'..=b'F' => byte - b'A' + 10,
      _ => return None,
    })
  }

  /// Parses two hexadecimal digits at the given string byte index.
  const fn hex_byte(bytes: &[u8], index: usize) -> Result<u8, GuidParserError> {
    let Some(high) = hex_digit(bytes[index]) else {
      return Err(GuidParserError::Digit(index));
    };

    let Some(low) = hex_digit(bytes[index + 1]) else {
      return Err(GuidParserError::Digit(index + 1));
    };

    Ok((high << 4) | low)
  }

  /// Parses a GUID string in the standard 36-character format.
  pub const fn parse_guid(string: &str) -> Result<EFI_GUID, GuidParserError> {
    let bytes = string.as_bytes();
    if bytes.len() != 36 {
      return Err(GuidParserError::Length(bytes.len()));
    }

    if bytes[8] != SEPARATOR {
      return Err(GuidParserError::Separator(8));
    }
    if bytes[13] != SEPARATOR {
      return Err(GuidParserError::Separator(13));
    }
    if bytes[18] != SEPARATOR {
      return Err(GuidParserError::Separator(18));
    }
    if bytes[23] != SEPARATOR {
      return Err(GuidParserError::Separator(23));
    }

    Ok(EFI_GUID {
      // Reverse the first three fields for little-endian storage.
      TimeLow: [
        _try!(hex_byte(bytes, 6)),
        _try!(hex_byte(bytes, 4)),
        _try!(hex_byte(bytes, 2)),
        _try!(hex_byte(bytes, 0)),
      ],
      TimeMid: [
        _try!(hex_byte(bytes, 11)), //
        _try!(hex_byte(bytes, 9)),
      ],
      TimeHighAndVersion: [
        _try!(hex_byte(bytes, 16)),
        _try!(hex_byte(bytes, 14)),
      ],

      // The remaining fields retain their string order.
      ClockSeqHighAndReserved: _try!(hex_byte(bytes, 19)),
      ClockSeqLow: _try!(hex_byte(bytes, 21)),
      Node: [
        _try!(hex_byte(bytes, 24)),
        _try!(hex_byte(bytes, 26)),
        _try!(hex_byte(bytes, 28)),
        _try!(hex_byte(bytes, 30)),
        _try!(hex_byte(bytes, 32)),
        _try!(hex_byte(bytes, 34)),
      ],
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  macro_rules! error {
    ($GUID:literal, $ERROR:ident($VALUE:literal)) => {
      assert_eq!(
        inner::parse_guid($GUID),
        Err(GuidParserError::$ERROR($VALUE))
      );
    };
  }

  #[test]
  fn guid_parser_length_error() {
    error!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX35", Length(35));
    error!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX37", Length(37));
  }

  #[test]
  fn guid_parser_separator_error() {
    error!("00000000_000000000000000000000000000", Separator(8));
    error!("00000000-0000_0000000000000000000000", Separator(13));
    error!("00000000-0000-0000_00000000000000000", Separator(18));
    error!("00000000-0000-0000-0000_000000000000", Separator(23));
  }

  #[test]
  fn guid_parser_digit_error() {
    error!("01Ab?Bc8-?sdU-,/wa-f~Hn-dsgEC29}:wzm", Digit(4));
    // [...]
  }

  #[test]
  fn parse_guid_string() {
    let guids = [
      "12345678-9ABC-DEF0-1234-56789ABCDEF0", // UU
      "12345678-9abc-def0-1234-56789abcdef0", // ll
      "12345678-9ABC-DEF0-1234-56789abcdef0", // Ul
    ];

    for guid in guids {
      assert_eq!(
        inner::parse_guid(guid),
        Ok(EFI_GUID {
          TimeLow: [0x78, 0x56, 0x34, 0x12],
          TimeMid: [0xBC, 0x9A],
          TimeHighAndVersion: [0xF0, 0xDE],
          ClockSeqHighAndReserved: 0x12,
          ClockSeqLow: 0x34,
          Node: [0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0],
        })
      );
    }
  }
}
