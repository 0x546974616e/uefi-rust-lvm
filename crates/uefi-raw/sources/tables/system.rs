use crate::{
  CHAR16, EFI_BOOT_SERVICES, EFI_RUNTIME_SERVICES,
  EFI_SIMPLE_TEXT_INPUT_PROTOCOL, EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
  EFI_TABLE_HEADER,
};

uefi_struct! {
  #[version(2.11, "4.3.1")]
  /// Contains pointers to the runtime and boot services tables.
  pub struct EFI_SYSTEM_TABLE {
    /// The table header for the *EFI System Table*.
    ///
    /// This header contains the `EFI_SYSTEM_TABLE_SIGNATURE` and
    /// `EFI_SYSTEM_TABLE_REVISION` values along with the size of the
    /// `EFI_SYSTEM_TABLE` structure and a 32-bit CRC to verify that the
    /// contents of the *EFI System Table* are valid.
    pub Header: EFI_TABLE_HEADER,

    /// A pointer to a null terminated string that identifies the vendor that
    /// produces the system firmware for the platform.
    FirmwareVendor: #[pointer] pub CHAR16,

    /// A firmware vendor specific value that identifies the revision of the
    /// system firmware for the platform.
    FirmwareRevision: u32,

    /// The handle for the active console input device.
    ///
    /// This handle must support `EFI_SIMPLE_TEXT_INPUT_PROTOCOL` and
    /// `EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL`. If there is no active console,
    /// these protocols must still be present.
    ConsoleInHandle: #[todo],

    /// A pointer to the `EFI_SIMPLE_TEXT_INPUT_PROTOCOL` interface that is
    /// associated with `ConsoleInHandle`.
    ConIn: #[pointer] pub EFI_SIMPLE_TEXT_INPUT_PROTOCOL,

    /// The handle for the active console output device.
    ///
    /// This handle must support the `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`. If there
    /// is no active console, this protocol must still be present.
    ConsoleOutHandle: #[todo],

    /// A pointer to the `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL` interface that is
    /// associated with `ConsoleOutHandle`.
    pub ConOut: #[pointer] pub EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,

    /// The handle for the active standard error console device.
    ///
    /// This handle must support the `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`. If there
    /// is no active console, this protocol must still be present.
    StandardErrorHandle: #[todo],

    /// A pointer to the `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL` interface that is
    /// associated with `StandardErrorHandle`.
    pub StdErr: #[pointer] pub EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,

    /// A pointer to the *EFI Runtime Services Table*.
    RuntimeServices: #[pointer] pub EFI_RUNTIME_SERVICES,

    /// A pointer to the *EFI Boot Services Table*.
    BootServices: #[pointer] pub EFI_BOOT_SERVICES,

    /// The number of system configuration tables in the buffer
    /// `ConfigurationTable`.
    NumberOfTableEntries: usize,

    /// A pointer to the system configuration tables. The number of entries in
    /// the table is `NumberOfTableEntries`.
    ConfigurationTable: #[todo],
  }
}
