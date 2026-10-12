#![no_std]
#![no_main]
#![allow(unused)] // TODO TMP

use core::ffi::c_void;
use core::panic::PanicInfo;

#[cfg_attr(test, allow(unused))]
#[cfg_attr(not(test), panic_handler)]
fn panic_impl(_info: &PanicInfo) -> ! {
  // TODO: EH Rewind ?
  loop {}
}

macro_rules! printf {
  ($($arg:tt)*) => {{
    $crate::print_args(format_args!($($arg)*));
  }};
}

pub fn print_args(_args: core::fmt::Arguments<'_>) {
  unsafe {
    // let st = SYSTEM_TABLE;
    // let mut console = Console::new((*st).con_out);
    // let _ = console.write_fmt(args);
  }
}

pub type ImageHandle = u64;
pub type Status = usize;


macro_rules! uefi_version {
  ($VERSION:literal) => {
    concat!(
      "-- <i>UEFI Specification, Release ",
        stringify!($VERSION),
      ".</i>"
    )
  };

  ($VERSION:literal, $SECTION:literal) => {
    concat!(
      "-- <i>UEFI Specification, Release ",
        stringify!($VERSION),
      ", Section ",
        $SECTION,
      ".</i>"
    )
  };
}

macro_rules! uefi_struct {
  (
    $(#[$STRUCT_ATTRIBUTE:meta])*
    $STRUCT_VISIBILITY:vis struct $STRUCT_NAME:ident {
      $(
        $(#[$FIELD_ATTRIBUTE:meta])*
        $FIELD_VISIBILITY:vis $FIELD_NAME:ident:
          $(#[$TYPE_ATTRIBUTE:tt])? $($FIELD_TYPE:ty)?
      ),* $(,)?
    }
  ) => {
    #[repr(C)]
    $(#[$STRUCT_ATTRIBUTE])*
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    $STRUCT_VISIBILITY struct $STRUCT_NAME {
      $(
        $(#[$FIELD_ATTRIBUTE])*
        $FIELD_VISIBILITY $FIELD_NAME:
          uefi_struct! {
            @type $(#[$TYPE_ATTRIBUTE])?
              $($FIELD_TYPE)?
          }
      ),*
    }

    impl $STRUCT_NAME {
      $(
        uefi_struct! {
          @fn $(#[$TYPE_ATTRIBUTE])?
            $(#[$FIELD_ATTRIBUTE])*
            $FIELD_VISIBILITY $FIELD_NAME()
              -> $($FIELD_TYPE)?;
        }
      )*
    }
  };

  (@type #[todo]) => { *const () };
  (@type #[pointer] $TYPE:ty) => { *const $TYPE };
  (@type $TYPE:ty) => { $TYPE };

  (@fn #[pointer]
    $(#[$FIELD_ATTRIBUTE:meta])*
    $VISIBILITY:vis $NAME:ident() -> $TYPE:ty;
  ) => {
    $(#[$FIELD_ATTRIBUTE])*
    #[allow(non_snake_case)]
    $VISIBILITY fn $NAME(&self) -> &$TYPE {
      // SAFETY: Callers know what they are doing...
      unsafe { self.$NAME.as_ref() }.unwrap()
    }
  };

  (@fn $($TT:tt)*) => {
    // Nothing to do here.
  };
}

uefi_struct! {
  #[repr(align(8))]
  /// 128-bit buffer containing a unique identifier value.
  ///
  /// Unless otherwise specified, aligned on a 64-bit boundary.
  pub struct EFI_GUID {
    pub Data1: u32,
    pub Data2: u16,
    pub Data3: u16,
    pub Data4: [u8; 8],
  }
}

/// Définit un protocol UEFI.
///
/// ```rust
/// ```
macro_rules! uefi_protocol {
  (
    $(
      #[guid]
      $GUID_VISIBILITY:vis const $GUID_NAME:ident = {
        $GUID_DATA_1:literal, $GUID_DATA_2:literal, $GUID_DATA_3:literal,
        { $($GUID_DATA_4:literal),* $(,)? } $(,)?
      } $(;)?
    )?

    #[version($($PROTOCOL_VERSION:literal),+)]
    $(#[$PROTOCOL_ATTRIBUTE:meta])*
    $PROTOCOL_VISIBILITY:vis struct $PROTOCOL_NAME:ident {
      $(
        $(#[$FIELD_ATTRIBUTE:meta])*
        $FIELD_VISIBILITY:vis $FIELD_NAME:ident
          $(:#[todo$($FIELD_TODO:tt)?])?
          $(:#[pointer] $FIELD_POINTER:ty)?
          $(:$FIELD_TYPE:ty)?
          $(:{
            #[version($($FUNCTION_VERSION:literal),+)]
            $( #[$FUNCTION_ATTRIBUTE:meta] )*
            $FUNCTION_VISIBILITY:vis fn $FUNCTION_NAME:ident(
              $(&$PARAMETER_SELF:ident)? $(,)?
              $($(#[in])? $(#[out])? $(#[optional])?
                $PARAMETER_NAME:ident: $PARAMETER_TYPE:ty
              ),* $(,)?
            ) $(-> $FUNCTION_RETURN:ty)?
          })?
      ),* $(,)?
    }
  ) => {
    $(
      $GUID_VISIBILITY const $GUID_NAME: EFI_GUID = EFI_GUID {
        Data1: $GUID_DATA_1, Data2: $GUID_DATA_2, Data3: $GUID_DATA_3,
        Data4: [ $($GUID_DATA_4),* ],
      };
    )?

    // NOTE:
    // The metavariable `$FUNCTION_NAME` is an `ident` and cannot be passed
    // to `uefi_struct!{}`, which only accepts `$FIELD_TYPE:ty`. We therefore
    // copy the struct declaration here...

    #[repr(C)]
    $(#[$PROTOCOL_ATTRIBUTE])*
    #[doc = ""]
    #[doc = uefi_version!($($PROTOCOL_VERSION),+)]
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    $PROTOCOL_VISIBILITY struct $PROTOCOL_NAME {
      $(
        $(#[$FIELD_ATTRIBUTE])*
        $FIELD_VISIBILITY $FIELD_NAME:
          $(*const ($($FIELD_TODO)?))?
          $(*const $FIELD_POINTER)?
          $($FUNCTION_NAME)?
          $($FIELD_TYPE)?
      ),*
    }

    $(
      $(
        $(#[$FUNCTION_ATTRIBUTE])*
        #[allow(non_snake_case)]
        #[allow(non_camel_case_types)]
        #[doc = ""]
        #[doc = uefi_version!($($FUNCTION_VERSION),+)]
        $FUNCTION_VISIBILITY type $FUNCTION_NAME =
          unsafe extern "efiapi" fn(
            $($PARAMETER_SELF: *const $PROTOCOL_NAME,)?
            $($PARAMETER_NAME: $PARAMETER_TYPE),*
          ) $(-> $FUNCTION_RETURN)?;
      )?
    )*

    impl $PROTOCOL_NAME {
      $(
        $(
          $(#[$FUNCTION_ATTRIBUTE])*
          #[allow(non_snake_case)]
          #[doc = ""]
          #[doc = uefi_version!($($FUNCTION_VERSION),+)]
          $FUNCTION_VISIBILITY fn $FIELD_NAME(
            &self, $($PARAMETER_NAME: $PARAMETER_TYPE),*
          ) $(-> $FUNCTION_RETURN)? {
            unsafe {
              (self.$FIELD_NAME)(
                $({ let $PARAMETER_SELF = self; $PARAMETER_SELF },)?
                $($PARAMETER_NAME),*
              )
            }
          }
        )?

        uefi_struct! {
          @fn #[pointer]
          $(#[$FIELD_ATTRIBUTE])*
          $FIELD_VISIBILITY $FIELD_NAME() -> $($FIELD_POINTER)?;
        }
      )*
    }
  };
}

#[allow(non_camel_case_types)]
pub type TR_TODO_FUNCTION_POINTER = *const ();

#[allow(non_camel_case_types)]
pub type EFI_STATUS = usize;

// typedef VOID* EFI_EVENT;
#[allow(non_camel_case_types)]
pub type EFI_EVENT = *const core::ffi::c_void;

type BOOLEAN = bool;

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

pub type CHAR16 = u16;

uefi_protocol! {
  #[guid]
  const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID = {
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
    pub Mode: #[pointer] EFI_SIMPLE_TEXT_OUTPUT_MODE,
  }
}

///  @par Data Structure Description:
///  Mode Structure pointed to by Simple Text Out protocol.
#[repr(C)]
#[allow(nonstandard_style)]
#[derive(Debug)]
pub struct EFI_SIMPLE_TEXT_OUTPUT_MODE {
  /// The number of modes supported by QueryMode () and SetMode ().
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

uefi_struct! {
  pub struct SystemTable {
    header: [u8; 24],
    firmware_vendor: u64,
    firmware_revision: u32,
    input_handle: ImageHandle,
    /// dadadada
    pub input: #[pointer] EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    output_handle: ImageHandle,
    pub output: #[pointer] EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    error_handle: ImageHandle,
    error: #[todo],
    runtime: #[todo],
    boot: #[todo],
    no_of_entries: usize,
    config_table: #[todo],
  }
}

#[repr(C)]
#[derive(Default)]
pub struct EFI_INPUT_KEY {
  ScanCode: u16,    // UINT16;
  UnicodeChar: u16, // CHAR16;
}

//#[no_mangle]
#[unsafe(export_name = "efi_main")]
pub extern "efiapi" fn efi_main(
  handle: ImageHandle,
  system_table: *mut SystemTable,
) -> Status {
  let gST = unsafe { system_table.as_ref_unchecked() };
  let stdout = gST.output();
  stdout.ClearScreen();

  let string = "hellO\n\r";
  for character in string.chars() {
    let mut buffer: [u16; 1] = [0];
    let utf16 = character.encode_utf16(&mut buffer);
    stdout.OutputString(utf16.as_ptr());
  }

  let string_arr =
    ['h' as u16, 'I' as u16, '!' as u16, '\n' as u16, '\0' as u16];
  stdout.OutputString(string_arr.as_ptr());

  let _mode = stdout.Mode();

  unsafe {
    let mut key = EFI_INPUT_KEY::default();
    loop {
      let status = gST.input().ReadKeyStroke(&mut key);
      if status == 0 {
        break;
      }
    }
  };

  // loop{}
  0
}
