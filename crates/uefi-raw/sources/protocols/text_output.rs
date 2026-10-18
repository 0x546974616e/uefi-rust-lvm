use crate::{BOOLEAN, CHAR16, EFI_STATUS, INT32};

// #[guid]
// pub const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID = {
//   0x387477C2, 0x69C7, 0x11D2,
//     { 0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3b }
// }

uefi_struct! {
  #[version(2.11, "12.4.1")]
  /// Mode structure pointed to by `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`.
  pub struct EFI_SIMPLE_TEXT_OUTPUT_MODE {
    /// The number of modes supported by `QueryMode()` and `SetMode()`.
    MaxMode: INT32,

    /// The text mode of the output device(s).
    Mode: INT32,

    /// The current character output attribute.
    Attribute: INT32,

    /// The cursor's column.
    CursorColumn: INT32,

    /// The cursor's row.
    CursorRow: INT32,

    /// The cursor is currently visible or not.
    CursorVisible: BOOLEAN,
  }
}

uefi_function! {
  #[version(2.11, "12.4.3")]
  /// Writes a string to the output device.
  ///
  /// # Parameters
  ///
  /// * `This`   - The protocol instance pointer.
  /// * `String` - The NULL-terminated string to be displayed on the output
  ///              device(s). All output devices must also support the
  ///              Unicode drawing character codes defined in this file.
  ///
  /// # Description
  ///
  /// The `OutputString()` function writes a string to the output device. This
  /// is the most basic output mechanism on an output device. The String is
  /// displayed at the current cursor location on the output device(s) and the
  /// cursor is advanced according to the rules listed in [EFI Cursor
  /// Location/Advance Rules](#).
  ///
  /// # Errors
  ///
  /// * `EFI_SUCCESS`            - The string was output to the device.
  /// * `EFI_DEVICE_ERROR`       - The device reported an error while attempting
  ///                              to output the text.
  /// * `EFI_UNSUPPORTED`        - The output device's mode is not currently in
  ///                              a defined text mode.
  /// * `EFI_WARN_UNKNOWN_GLYPH` - This warning code indicates that some of the
  ///                              characters in the string could not be
  ///                              rendered and were skipped.
  pub fn EFI_TEXT_STRING for EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL::OutputString(
    &This,
    #[in] String: *const CHAR16,
  ) -> EFI_STATUS;
}

uefi_function! {
  #[version(2.11, "12.4.8")]
  /// Clears the output device(s) display to the currently selected background
  /// color.
  ///
  /// # Parameters
  ///
  /// * `This` - The protocol instance pointer.
  ///
  /// # Description
  ///
  /// The `ClearScreen()` function clears the output device(s) display to the
  /// currently selected background color. The cursor position is set to (0, 0).
  ///
  /// # Errors
  ///
  /// * `EFI_SUCCESS`      - The operation completed successfully.
  /// * `EFI_DEVICE_ERROR` - The device had an error and could not complete
  ///                        the request.
  /// * `EFI_UNSUPPORTED`  - The output device is not in a valid text mode.
  pub fn EFI_TEXT_CLEAR_SCREEN for EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL::ClearScreen(
    &This,
  ) -> EFI_STATUS;
}

uefi_struct! {
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
    OutputString: #[function] EFI_TEXT_STRING,

    /// Tests to see if the *ConsoleOut* device supports this string.
    TestOutput: #[todo],

    /// Queries information concerning the output device's supported text mode.
    query_mode: #[todo],

    /// Sets the current mode of the output device.
    SetMode: #[todo],

    /// Sets the foreground and background color of the text that is output.
    SetAttribute: #[todo],

    /// Clears the screen with the currently set background color.
    ClearScreen: #[function] EFI_TEXT_CLEAR_SCREEN,

    /// Sets the current cursor position.
    SetCursorPosition: #[todo],

    /// Turns the visibility of the cursor on/off.
    EnableCursor: #[todo],

    /// Pointer to `EFI_SIMPLE_TEXT_OUTPUT_MODE` data.
    Mode: #[pointer] pub EFI_SIMPLE_TEXT_OUTPUT_MODE,
  }
}
