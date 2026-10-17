uefi_enum! {
  #[version(2.11, "8.5.1.1")]
  /// The type of reset to perform.
  pub enum EFI_RESET_TYPE {
    /// Calling `EFI_RESET_SYSTEM` with `ResetType` of `EfiResetCold` causes a
    /// system-wide reset. This sets all circuitry within the system to its
    /// initial state. This type of reset is asynchronous to system operation
    /// and operates without regard to cycle boundaries. `EfiResetCold` is
    /// tantamount to a system power cycle.
    EfiResetCold,

    /// Calling `EFI_RESET_SYSTEM` with `ResetType` of `EfiResetWarm` causes a
    /// system-wide initialization. The processors are set to their initial
    /// state, and pending cycles are not corrupted. If the system does not
    /// support this reset type, then an `EfiResetCold` must be performed.
    EfiResetWarm,

    /// Calling `EFI_RESET_SYSTEM` with `ResetType` of `EfiResetShutdown` causes
    /// the system to enter a power state equivalent to the ACPI G2/S5 or G3
    /// states. If the system does not support this reset type, then when the
    /// system is rebooted, it should exhibit the `EfiResetCold` attributes.
    EfiResetShutdown,

    /// Calling `EFI_RESET_SYSTEM` with `ResetType` of `EfiResetPlatformSpecific`
    /// causes a system-wide reset. The exact type of the reset is defined by
    /// the `EFI_GUID` that follows the Null-terminated Unicode string passed
    /// into `ResetData`. If the platform does not recognize the `EFI_GUID` in
    /// `ResetData` the platform must pick a supported reset type to perform.
    /// The platform may optionally log the parameters from any non-normal reset
    /// that occurs.
    EfiResetPlatformSpecific,
  }
}
