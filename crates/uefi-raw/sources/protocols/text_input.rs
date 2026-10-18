use crate::{BOOLEAN, EFI_EVENT, EFI_INPUT_KEY, EFI_STATUS};

// #[guid]
// pub const EFI_SIMPLE_TEXT_INPUT_PROTOCOL_GUID = {
//   0x387477C1, 0x69C7, 0x11D2,
//     { 0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B }
// }

uefi_function! {
  #[version(2.11, "12.3.2")]
  /// Resets the input device hardware.
  ///
  /// # Parameters
  ///
  /// * `This`                 - Protocol instance pointer.
  /// * `ExtendedVerification` - Indicates that the driver may perform a more
  ///                            exhaustive verification operation of to device
  ///                            during reset.
  ///
  /// # Description
  ///
  /// The implementation of `Reset` is required to clear the contents of any
  /// input queues resident in memory used for buffering keystroke data and put
  /// the input stream in a known empty state.
  ///
  /// As part of initialization process, the firmware/device will make a quick
  /// but reasonable attempt to verify that the device is functioning.  If the
  /// `ExtendedVerification` flag is `TRUE` the firmware may take an extended
  /// amount of time to verify the device is operating on reset.  Otherwise the
  /// reset operation is to occur as quickly as possible.
  ///
  /// The hardware verification process is not defined by this specification and
  /// is left up to the platform firmware or driver to implement.
  ///
  /// # Errors
  ///
  /// * `EFI_SUCCESS`      - The device was reset.
  /// * `EFI_DEVICE_ERROR` - The device is not functioning properly and could
  ///                        not be reset.
  pub fn EFI_INPUT_RESET for EFI_SIMPLE_TEXT_INPUT_PROTOCOL::Reset(
    &This,
    #[in] ExtendedVerification: BOOLEAN,
  ) -> EFI_STATUS;
}

uefi_function! {
  #[version(2.11, "12.3.3")]
  /// Reads the next keystroke from the input device.
  ///
  /// # Parameters
  ///
  /// * `This` - Protocol instance pointer.
  /// * `Key`  - A pointer to a buffer that is filled in with the keystroke
  ///            information for the key that was pressed.
  ///
  /// # Description
  ///
  /// The `ReadKeyStroke()` function reads the next keystroke from the input
  /// device. If there is no pending keystroke the function returns
  /// `EFI_NOT_READY`. If there is a pending keystroke, then `ScanCode` is the
  /// EFI scan code defined in [EFI Scan Codes](#). The `UnicodeChar` is the
  /// actual printable character or is zero if the key does not represent a
  /// printable character (control key, function key, etc.).
  ///
  /// # Errors
  ///
  /// * `EFI_SUCCESS`      - The keystroke information was returned.
  /// * `EFI_NOT_READY`    - There was no keystroke data available.
  /// * `EFI_DEVICE_ERROR` - The keystroke information was not returned due to
  ///                        hardware errors.
  /// * `EFI_UNSUPPORTED`  - The device does not support the ability to read
  ///                        keystroke data.
  pub fn EFI_INPUT_READ_KEY for EFI_SIMPLE_TEXT_INPUT_PROTOCOL::ReadKeyStroke(
    &This,
    #[out] Key: *mut EFI_INPUT_KEY,
  ) -> EFI_STATUS;
}

uefi_struct! {
  #[version(2.11, "12.3.1")]
  /// This protocol is used to obtain input from the *ConsoleIn* device.
  ///
  /// The `EFI_SIMPLE_TEXT_INPUT_PROTOCOL` is used on the *ConsoleIn* device. It
  /// is the minimum required protocol for *ConsoleIn*. The EFI specification
  /// requires that the `EFI_SIMPLE_TEXT_INPUT_PROTOCOL` supports the same
  /// languages as the corresponding `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`.
  pub struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL {
    /// Reset the *ConsoleIn* device.
    Reset: #[function] EFI_INPUT_RESET,

    /// Returns the next input character.
    ReadKeyStroke: #[function] EFI_INPUT_READ_KEY,

    /// Event to use with `WaitForEvent()` to wait for a key to be available.
    pub WaitForKey: EFI_EVENT,
  }
}
