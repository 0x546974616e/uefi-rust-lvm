use crate::*;

uefi_struct! {
  #[version(2.11, "12.4.1")]
  /// Mode structure pointed to by `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`.
  pub struct EFI_SIMPLE_TEXT_OUTPUT_MODE {
    /// The number of modes supported by `QueryMode()` and `SetMode()`.
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

uefi_struct! {
  #[version(2.11, "4.2.1")]
  /// Data structure that precedes all of the standard EFI table types.
  ///
  /// **NOTE**: The capabilities found in the EFI system table, runtime table
  /// and boot services table may change over time. The first field in each of
  /// these tables is an `EFI_TABLE_HEADER`. This header’s `Revision` field is
  /// incremented when new capabilities and functions are added to the functions
  /// in the table. When checking for capabilities, code should verify that
  /// `Revision` is greater than or equal to the revision level of the table at
  /// the point when the capabilities were added to the UEFI specification.
  ///
  /// **NOTE**: Unless otherwise specified, UEFI uses a standard CCITT32 CRC
  /// algorithm with a seed polynomial value of `0x04c11db7` for its CRC
  /// calculations.
  ///
  /// **NOTE**: The size of the system table, runtime services table, and boot
  /// services table may increase over time. It is very important to always use
  /// the `HeaderSize` field of the `EFI_TABLE_HEADER` to determine the size of
  /// these tables.
  pub struct EFI_TABLE_HEADER {
    /// A 64-bit signature that identifies the type of table that follows.
    /// Unique signatures have been generated for the EFI System Table, the EFI
    /// *Boot Services Table*, and the *EFI Runtime Services Table*.
    Signature: u64,

    /// The revision of the EFI Specification to which this table conforms. The
    /// upper 16 bits of this field contain the major revision value, and the
    /// lower 16 bits contain the minor revision value. The minor revision value
    /// is a decimal value split into two parts, the "*upper decimal*" and the
    /// "*lower decimal*". The upper decimal is calculated dividing the minor
    /// revision value by 10 using integer division. The lower decimal is
    /// calculated by taking the minor revision value modulo 10.
    ///
    /// When printed or displayed UEFI spec revision is referred as {**Major
    /// revision**}.{**Minor revision upper digits**}.{**Minor revision lowest
    /// digit**} or {**Major revision**}.{**Minor revision upper digits**} in
    /// case Minor revision lowest digit is set to 0. For example:
    ///
    /// * Specification revision value `((2<<16) | (10))` would be referred
    ///   to as **2.1**;
    /// * Specification revision value `((2<<16) | (30))` would be referred
    ///   to as **2.3**;
    /// * Specification revision value `((2<<16) | (31))` would be referred
    ///   to as **2.3.1**;
    /// * Specification revision value `((2<<16) | (100))` would be referred
    ///   to as **2.10**;
    /// * Specification revision value `((2<<16) | (101))` would be referred
    ///   to as **2.10.1**.
    ///
    /// Note that EFI **1.10** did not follow this convention and was referred
    /// to as **1.10**, not **1.1**.
    Revision: u32,

    /// The size, in bytes, of the entire table including the `EFI_TABLE_HEADER`.
    HeaderSize: u32,

    /// The 32-bit CRC for the entire table. This value is computed by setting
    /// this field to 0, and computing the 32-bit CRC for `HeaderSize` bytes.
    CRC32: u32,

    /// Reserved field that must be set to `0`.
    Reserved: u32,
  }
}

uefi_struct! {
  #[version(2.11, "4.3.1")]
  /// Contains pointers to the runtime and boot services tables.
  pub struct EFI_SYSTEM_TABLE {
    /// The table header for the *EFI System Table*. This header contains the
    /// `EFI_SYSTEM_TABLE_SIGNATURE` and `EFI_SYSTEM_TABLE_REVISION` values
    /// along with the size of the `EFI_SYSTEM_TABLE` structure and a 32-bit CRC
    /// to verify that the contents of the *EFI System Table* are valid.
    pub Header: EFI_TABLE_HEADER,

    /// A pointer to a null terminated string that identifies the vendor that
    /// produces the system firmware for the platform.
    FirmwareVendor: #[pointer] pub CHAR16,

    /// A firmware vendor specific value that identifies the revision of the
    /// system firmware for the platform.
    FirmwareRevision: u32,

    /// The handle for the active console input device. This handle must support
    /// `EFI_SIMPLE_TEXT_INPUT_PROTOCOL` and `EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL`.
    /// If there is no active console, these protocols must still be present.
    ConsoleInHandle: #[todo],

    /// A pointer to the `EFI_SIMPLE_TEXT_INPUT_PROTOCOL` interface that is
    /// associated with `ConsoleInHandle`.
    ConIn: #[pointer] pub EFI_SIMPLE_TEXT_INPUT_PROTOCOL,

    /// The handle for the active console output device. This handle must
    /// support the `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`. If there is no active
    /// console, this protocol must still be present.
    ConsoleOutHandle: #[todo],

    /// A pointer to the `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL` interface that is
    /// associated with `ConsoleOutHandle`.
    pub ConOut: #[pointer] pub EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,

    /// The handle for the active standard error console device. This handle
    /// must support the `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`. If there is no
    /// active console, this protocol must still be present.
    StandardErrorHandle: #[todo],

    /// A pointer to the `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL` interface that is
    /// associated with `StandardErrorHandle`.
    pub StdErr: #[pointer] pub EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,

    /// A pointer to the *EFI Runtime Services Table*.
    RuntimeServices: #[pointer] pub EFI_RUNTIME_SERVICES,

    /// A pointer to the *EFI Boot Services Table*.
    BootServices: #[todo], // EFI_BOOT_SERVICES

    /// The number of system configuration tables in the buffer
    /// `ConfigurationTable`.
    NumberOfTableEntries: usize,

    /// A pointer to the system configuration tables. The number of entries in
    /// the table is `NumberOfTableEntries`.
    ConfigurationTable: #[todo],
  }
}

uefi_protocol! {
  #[version(2.11, "4.5.1")]
  /// Contains a table header and pointers to all of the runtime services.
  pub struct EFI_RUNTIME_SERVICES {
    /// The table header for the *EFI Runtime Services Table*. This header
    /// contains the `EFI_RUNTIME_SERVICES_SIGNATURE` and
    /// `EFI_RUNTIME_SERVICES_REVISION` values along with the size of the
    /// `EFI_RUNTIME_SERVICES` structure and a 32-bit CRC to verify that the
    /// contents of the *EFI Runtime Services Table* are valid.
    pub Header: EFI_TABLE_HEADER,

    /// Returns the current time and date, and the time-keeping capabilities of
    /// the platform.
    GetTime: #[todo],

    /// Sets the current local time and date information.
    SetTime: #[todo],

    /// Returns the current wakeup alarm clock setting.
    GetWakeupTime: #[todo],

    /// Sets the system wakeup alarm clock time.
    SetWakeupTime: #[todo],

    /// Used by a UEFI OS loader to convert from physical addressing to virtual
    /// addressing.
    SetVirtualAddressMap: #[todo],

    /// Used by EFI components to convert internal pointers when switching to
    /// virtual addressing.
    ConvertPointer: #[todo],

    /// Returns the value of a variable.
    GetVariable: #[todo],

    /// Enumerates the current variable names.
    GetNextVariableName: #[todo],

    /// Sets the value of a variable.
    SetVariable: #[todo],

    /// Returns the next high 32 bits of the platform’s monotonic counter.
    GetNextHighMonotonicCount: #[todo],

    /// Resets the entire platform.
    ResetSystem: {
      #[version(2.11, "8.5.1.1")]
      /// Resets the entire platform.
      ///
      /// If the platform supports `EFI_RESET_NOTIFICATION_PROTOCOL`, then
      /// prior to completing the reset of the platform, all of the pending
      /// notifications must be called.
      ///
      /// The `ResetSystem()` function resets the entire platform, including all
      /// processors and devices, and reboots the system. This function does not
      /// return.
      ///
      /// # Parameters
      ///
      /// * `ResetType`   - The type of reset to perform.
      /// * `ResetStatus` - The status code for the reset.
      ///
      ///   If the system reset is part of a normal operation, the status
      ///   code would be `EFI_SUCCESS`. If the system reset is due to some
      ///   type of failure the most appropriate EFI Status code would be used.
      ///
      /// * `DataSize`  - The size, in bytes, of `ResetData`.
      /// * `ResetData` - Data according to `EFI_RESET_TYPE`.
      ///
      ///   For a `ResetType` of `EfiResetCold`, `EfiResetWarm`, or
      ///   `EfiResetShutdown` the data buffer starts with a Null-terminated
      ///   string, optionally followed by additional binary data. The string is
      ///   a description that the caller may use to further indicate the reason
      ///   for the system reset. For a `ResetType`  of `EfiResetPlatformSpecific`
      ///   the data buffer also starts with a Null-terminated string that is
      ///   followed by an `EFI_GUID` that describes the specific type of reset
      ///   to perform.
      pub fn EFI_RESET_SYSTEM(
        #[in] ResetType: EFI_RESET_TYPE,
        #[in] ResetStatus: EFI_STATUS,
        #[in] DataSize: usize,
        #[in, optional] ResetData: *const VOID,
      )
    },

    /// Passes capsules to the firmware with both virtual and physical mapping.
    UpdateCapsule: #[todo],

    /// Returns if the capsule can be supported via `UpdateCapsule()`.
    QueryCapsuleCapabilities: #[todo],

    /// Returns information about the EFI variable store.
    QueryVariableInfo: #[todo],
  }
}
