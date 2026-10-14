#![allow(clippy::doc_overindented_list_items)]
use super::{structs::*, types::*};

uefi_protocol! {
  #[guid]
  pub const EFI_SIMPLE_TEXT_INPUT_PROTOCOL_GUID = {
    0x387477C1, 0x69C7, 0x11D2,
      { 0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B }
  }

  #[version(2.11, "12.3.1")]
  /// This protocol is used to obtain input from the *ConsoleIn* device.
  ///
  /// The `EFI_SIMPLE_TEXT_INPUT_PROTOCOL` is used on the *ConsoleIn* device. It
  /// is the minimum required protocol for *ConsoleIn*. The EFI specification
  /// requires that the `EFI_SIMPLE_TEXT_INPUT_PROTOCOL` supports the same
  /// languages as the corresponding `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`.
  pub struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL {
    /// Reset the *ConsoleIn* device.
    Reset: {
      #[version(2.11, "12.3.2")]
      /// Resets the input device hardware.
      ///
      /// The implementation of `Reset` is required to clear the contents of any
      /// input queues resident in memory used for buffering keystroke data and
      /// put the input stream in a known empty state.
      ///
      /// As part of initialization process, the firmware/device will make a
      /// quick but reasonable attempt to verify that the device is functioning.
      /// If the `ExtendedVerification` flag is `TRUE` the firmware may take an
      /// extended amount of time to verify the device is operating on reset.
      /// Otherwise the reset operation is to occur as quickly as possible.
      ///
      /// The hardware verification process is not defined by this specification
      /// and is left up to the platform firmware or driver to implement.
      ///
      /// # Parameters
      ///
      /// * `This`                 - Protocol instance pointer.
      /// * `ExtendedVerification` - Indicates that the driver may perform a
      ///                            more exhaustive verification operation of
      ///                            the device during reset.
      ///
      /// # Errors
      ///
      /// * `EFI_SUCCESS`      - The device was reset.
      /// * `EFI_DEVICE_ERROR` - The device is not functioning properly and
      ///                        could not be reset.
      pub fn EFI_INPUT_RESET(
        &This,
        #[in] ExtendedVerification: BOOLEAN,
      ) -> EFI_STATUS
    },

    /// Returns the next input character.
    ReadKeyStroke: {
      #[version(2.11, "12.3.3")]
      /// Reads the next keystroke from the input device.
      ///
      /// The `ReadKeyStroke()` function reads the next keystroke from the input
      /// device. If there is no pending keystroke the function returns
      /// `EFI_NOT_READY`. If there is a pending keystroke, then `ScanCode` is
      /// the EFI scan code defined in [EFI Scan Codes][#]. The `UnicodeChar` is
      /// the actual printable character or is zero if the key does not
      /// represent a printable character (control key, function key, etc.).
      ///
      /// # Parameters
      ///
      /// * `This` - Protocol instance pointer.
      /// * `Key`  - A pointer to a buffer that is filled in with the keystroke
      ///            information for the key that was pressed.
      ///
      /// # Errors
      ///
      /// * `EFI_SUCCESS`      - The keystroke information was returned.
      /// * `EFI_NOT_READY`    - There was no keystroke data available.
      /// * `EFI_DEVICE_ERROR` - The keystroke information was not returned due
      ///                        to hardware errors.
      /// * `EFI_UNSUPPORTED`  - The device does not support the ability to read
      ///                        keystroke data.
      pub fn EFI_INPUT_READ_KEY(
        &This,
        #[out] Key: *mut EFI_INPUT_KEY,
      ) -> EFI_STATUS
    },

    /// Event to use with `WaitForEvent()` to wait for a key to be available.
    pub WaitForKey: EFI_EVENT,
  }
}

uefi_protocol! {
  #[guid]
  pub const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID = {
    0x387477C2, 0x69C7, 0x11D2,
      { 0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3b }
  }

  #[version(2.11, "12.4.1")]
  /// This protocol is used to control text-based output devices.
  ///
  /// The *Simple Text Output protocol* defines the minimum requirements for a
  /// text-based *ConsoleOut* device. The EFI specification requires that the
  /// `EFI_SIMPLE_TEXT_INPUT_PROTOCOL` support the same languages as the
  /// corresponding `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`.
  pub struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    /// Reset the ConsoleOut device.
    Reset: #[todo],

    /// Displays the string on the device at the current cursor location.
    OutputString: {
      #[version(2.11, "12.4.3")]
      /// Writes a string to the output device.
      ///
      /// The `OutputString()` function writes a string to the output device.
      /// This is the most basic output mechanism on an output device. The
      /// String is displayed at the current cursor location on the output
      /// device(s) and the cursor is advanced according to the rules listed in
      /// [EFI Cursor Location/Advance Rules][#].
      ///
      /// # Parameters
      ///
      /// * `This`   - The protocol instance pointer.
      /// * `String` - The NULL-terminated string to be displayed on the output
      ///              device(s). All output devices must also support the
      ///              Unicode drawing character codes defined in this file.
      ///
      /// # Errors
      ///
      /// * `EFI_SUCCESS`            - The string was output to the device.
      /// * `EFI_DEVICE_ERROR`       - The device reported an error while
      ///                              attempting to output the text.
      /// * `EFI_UNSUPPORTED`        - The output device's mode is not currently
      ///                              in a defined text mode.
      /// * `EFI_WARN_UNKNOWN_GLYPH` - This warning code indicates that some of
      ///                              the characters in the string could not be
      ///                              rendered and were skipped.
      pub fn EFI_TEXT_STRING(
        &This,
        #[in] String: *const CHAR16,
      ) -> EFI_STATUS
    },

    /// Tests to see if the *ConsoleOut* device supports this string.
    TestOutput: #[todo],

    /// Queries information concerning the output device's supported text mode.
    query_mode: #[todo],

    /// Sets the current mode of the output device.
    SetMode: #[todo],

    /// Sets the foreground and background color of the text that is output.
    SetAttribute: #[todo],

    /// Clears the screen with the currently set background color.
    ClearScreen: {
      #[version(2.11, "12.4.8")]
      /// Clears the output device(s) display to the currently selected
      /// background color.
      ///
      /// The `ClearScreen()` function clears the output device(s) display to
      /// the currently selected background color. The cursor position is set
      /// to (0, 0).
      ///
      /// # Parameters
      ///
      /// * `This` - The protocol instance pointer.
      ///
      /// # Errors
      ///
      /// * `EFI_SUCCESS`      - The operation completed successfully.
      /// * `EFI_DEVICE_ERROR` - The device had an error and could not complete
      ///                        the request.
      /// * `EFI_UNSUPPORTED`  - The output device is not in a valid text mode.
      pub fn ClearScreen(&This) -> EFI_STATUS
    },

    /// Sets the current cursor position.
    SetCursorPosition: #[todo],

    /// Turns the visibility of the cursor on/off.
    EnableCursor: #[todo],

    /// Pointer to `EFI_SIMPLE_TEXT_OUTPUT_MODE` data.
    Mode: #[pointer] pub EFI_SIMPLE_TEXT_OUTPUT_MODE,
  }
}
