use crate::{EFI_STATUS, EFI_TABLE_HEADER, UINTN, VOID};

uefi_enum! {
  #[version(2.11, "8.5.1.1")]
  /// The type of reset to perform.
  pub enum EFI_RESET_TYPE {
    /// Calling `EFI_RESET_SYSTEM` with `ResetType` of `EfiResetCold` causes a
    /// system-wide reset. This sets all circuitry within the system to its
    /// initial state. This type of reset is asynchronous to system operation
    /// and operates without regard to cycle boundaries. `EfiResetCold` is
    /// tantamount to a system power cycle.
    EfiResetCold = 0,

    /// Calling `EFI_RESET_SYSTEM` with `ResetType` of `EfiResetWarm` causes a
    /// system-wide initialization. The processors are set to their initial
    /// state, and pending cycles are not corrupted. If the system does not
    /// support this reset type, then an `EfiResetCold` must be performed.
    EfiResetWarm = 1,

    /// Calling `EFI_RESET_SYSTEM` with `ResetType` of `EfiResetShutdown` causes
    /// the system to enter a power state equivalent to the ACPI G2/S5 or G3
    /// states. If the system does not support this reset type, then when the
    /// system is rebooted, it should exhibit the `EfiResetCold` attributes.
    EfiResetShutdown = 2,

    /// Calling `EFI_RESET_SYSTEM` with `ResetType` of `EfiResetPlatformSpecific`
    /// causes a system-wide reset. The exact type of the reset is defined by
    /// the `EFI_GUID` that follows the Null-terminated Unicode string passed
    /// into `ResetData`. If the platform does not recognize the `EFI_GUID` in
    /// `ResetData` the platform must pick a supported reset type to perform.
    /// The platform may optionally log the parameters from any non-normal reset
    /// that occurs.
    EfiResetPlatformSpecific = 3,
  }
}

uefi_function! {
  #[version(2.11, "8.5.1.1")]
  /// Resets the entire platform.
  ///
  /// # Parameters
  ///
  /// * `ResetType`   - The type of reset to perform.
  /// * `ResetStatus` - The status code for the reset.
  ///
  ///   If the system reset is part of a normal operation, the status code would
  ///   be `EFI_SUCCESS`. If the system reset is due to some type of failure the
  ///   most appropriate EFI Status code would be used.
  ///
  /// * `DataSize`  - The size, in bytes, of `ResetData`.
  /// * `ResetData` - Data according to `EFI_RESET_TYPE`.
  ///
  ///   For a `ResetType` of `EfiResetCold`, `EfiResetWarm`, or
  ///   `EfiResetShutdown` the data buffer starts with a Null-terminated string,
  ///   optionally followed by additional binary data. The string is a
  ///   description that the caller may use to further indicate the reason for
  ///   the system reset. For a `ResetType`  of `EfiResetPlatformSpecific` the
  ///   data buffer also starts with a Null-terminated string that is followed
  ///   by an `EFI_GUID` that describes the specific type of reset to perform.
  ///
  /// # Description
  ///
  /// If the platform supports `EFI_RESET_NOTIFICATION_PROTOCOL`, then prior to
  /// completing the reset of the platform, all of the pending notifications
  /// must be called.
  ///
  /// The `ResetSystem()` function resets the entire platform, including all
  /// processors and devices, and reboots the system. This function does not
  /// return.
  pub fn EFI_RESET_SYSTEM for EFI_RUNTIME_SERVICES::ResetSystem(
    #[in] ResetType: EFI_RESET_TYPE,
    #[in] ResetStatus: EFI_STATUS,
    #[in] DataSize: UINTN,
    #[in(optional)] ResetData: *const VOID,
  );
}

uefi_struct! {
  #[version(2.11, "4.5.1")]
  /// Contains a table header and pointers to all of the runtime services.
  ///
  /// UEFI uses the *EFI Runtime Services Table*, which contains a table header
  /// and pointers to all of the runtime services. Except for the table header,
  /// all elements in the *EFI Runtime Services Tables* are prototypes of
  /// function pointers to functions as defined in [Chapter 8: Services -
  /// Runtime Services](#).  Unlike the *EFI Boot Services Table*, this table,
  /// and the function pointers it contains are valid after the UEFI OS loader
  /// and OS have taken control of the platform with a call to
  /// `EFI_BOOT_SERVICES.ExitBootServices()`. If a call to
  /// `SetVirtualAddressMap()` is made by the OS, then the function pointers in
  /// this table are fixed up to point to the new virtually mapped entry points.
  pub struct EFI_RUNTIME_SERVICES {
    /// The table header for the *EFI Runtime Services Table*.
    ///
    /// This header contains the `EFI_RUNTIME_SERVICES_SIGNATURE` and
    /// `EFI_RUNTIME_SERVICES_REVISION` values along with the size of the
    /// `EFI_RUNTIME_SERVICES` structure and a 32-bit CRC to verify that the
    /// contents of the *EFI Runtime Services Table* are valid.
    pub Header: EFI_TABLE_HEADER,

    ////////////////////////////// Time Services ///////////////////////////////

    /// Returns the current time and date, and the time-keeping capabilities of
    /// the platform.
    GetTime: #[todo],

    /// Sets the current local time and date information.
    SetTime: #[todo],

    /// Returns the current wakeup alarm clock setting.
    GetWakeupTime: #[todo],

    /// Sets the system wakeup alarm clock time.
    SetWakeupTime: #[todo],

    ///////////////////////// Virtual Memory Services //////////////////////////

    /// Used by a UEFI OS loader to convert from physical addressing to virtual
    /// addressing.
    SetVirtualAddressMap: #[todo],

    /// Used by EFI components to convert internal pointers when switching to
    /// virtual addressing.
    ConvertPointer: #[todo],

    //////////////////////////// Variable Services /////////////////////////////

    /// Returns the value of a variable.
    GetVariable: #[todo],

    /// Enumerates the current variable names.
    GetNextVariableName: #[todo],

    /// Sets the value of a variable.
    SetVariable: #[todo],

    ////////////////////////// Miscellaneous Services //////////////////////////

    /// Returns the next high 32 bits of the platform’s monotonic counter.
    GetNextHighMonotonicCount: #[todo],

    /// Resets the entire platform.
    ResetSystem: #[function] EFI_RESET_SYSTEM,

    //////////////////////// UEFI 2.0 Capsule Services /////////////////////////

    /// Passes capsules to the firmware with both virtual and physical mapping.
    UpdateCapsule: #[todo],

    /// Returns if the capsule can be supported via `UpdateCapsule()`.
    QueryCapsuleCapabilities: #[todo],

    ////////////////////// Miscellaneous UEFI 2.0 Service //////////////////////

    /// Returns information about the EFI variable store.
    QueryVariableInfo: #[todo],
  }
}
