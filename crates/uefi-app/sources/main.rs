#![no_std]
#![no_main]
#![allow(unused)] // TODO TMP

use core::panic::PanicInfo;

#[cfg(not(target_os = "uefi"))]
fn main() {}

#[cfg_attr(test, allow(unused))]
#[cfg_attr(not(test), panic_handler)]
fn panic_impl(info: &PanicInfo) -> ! {
  println!("PANIC {info}.");
  loop {
    core::hint::spin_loop();
  }
}

use {uefi_print::*, uefi_raw::*};

/// # Safety
/// UEFI entry pointer
// #[no_mangle]
#[unsafe(export_name = "efi_main")]
pub unsafe extern "efiapi" fn efi_main(
  handle: TodoImageHandle,
  system_table: *const EFI_SYSTEM_TABLE,
) -> EFI_STATUS {
  unsafe { SYSTEM_TABLE = system_table };
  unsafe { uefi_raw::SYSTEM_TABLE = system_table };
  let st = unsafe { system_table.as_ref_unchecked() };
  let stdout = st.ConOut();
  unsafe { stdout.ClearScreen() };

  println!("Hello, World! \u{1F600}");
  println!("{EFI_INCOMPATIBLE_VERSION}");
  let _mode = stdout.Mode();
  //println!("{mode:#?}");
  println!("{st:#?}");

  // let _ = None::<usize>.unwrap();

  unsafe {
    let mut key = EFI_INPUT_KEY::default();
    loop {
      let status = st.ConIn().ReadKeyStroke(&mut key);
      if status == EFI_SUCCESS {
        break;
      }
    }
  };

  unsafe {
    st.RuntimeServices().ResetSystem(
      EFI_RESET_TYPE::EfiResetShutdown,
      EFI_SUCCESS,
      0usize,
      NULL,
    )
  };

  EFI_SUCCESS
}
