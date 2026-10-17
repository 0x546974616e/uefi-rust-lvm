#![no_std]
#![no_main]
#![allow(unused)] // TODO TMP

use core::{ffi::c_void, panic::PanicInfo};

#[cfg(not(target_os = "uefi"))]
fn main() {}

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

pub type Status = usize;

use uefi_raw::*;

//#[no_mangle]
#[unsafe(export_name = "efi_main")]
pub extern "efiapi" fn efi_main(
  handle: TodoImageHandle,
  system_table: *mut EFI_SYSTEM_TABLE,
) -> Status {
  let gST = unsafe { system_table.as_ref_unchecked() };
  let stdout = gST.ConOut();
  unsafe { stdout.ClearScreen() };

  let string = "hellO\n\r";
  for character in string.chars() {
    let mut buffer: [u16; 2] = [0, 0];
    let utf16 = character.encode_utf16(&mut buffer);
    unsafe { stdout.OutputString(utf16.as_ptr()) };
  }

  let string_arr =
    ['h' as u16, 'I' as u16, '!' as u16, '\n' as u16, '\0' as u16];
  unsafe { stdout.OutputString(string_arr.as_ptr()) };

  let _mode = stdout.Mode();

  unsafe {
    let mut key = EFI_INPUT_KEY::default();
    loop {
      let status = gST.ConIn().ReadKeyStroke(&mut key);
      if status == EFI_SUCCESS {
        break;
      }
    }
  };

  unsafe {
    gST.RuntimeServices().ResetSystem(
      EFI_RESET_TYPE::EfiResetShutdown,
      EFI_SUCCESS,
      0,
      NULL,
    )
  };
  // gRT->ResetSystem(EfiResetShutdown, EFI_SUCCESS, 0, NULL);
  // loop{}
  0
}
