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
  handle: EFI_HANDLE,
  system_table: *const EFI_SYSTEM_TABLE,
) -> EFI_STATUS {
  unsafe { uefi_raw::SYSTEM_TABLE = system_table };
  let system_table = unsafe { system_table.as_ref_unchecked() };
  let boot_services = system_table.BootServices();

  let stdout = system_table.ConOut();
  unsafe { stdout.ClearScreen() };

  println!("Hello, World! 😀 = \u{1F600}");
  println!("{EFI_INCOMPATIBLE_VERSION}");
  let _mode = stdout.Mode();
  //println!("{mode:#?}");
  println!("{system_table:#X?}");

  let mut index = 0usize;
  let mut key = EFI_INPUT_KEY::default();
  let event = system_table.ConIn().WaitForKey;
  let status = unsafe { boot_services.WaitForEvent(1, &event, &mut index) };
  let status = unsafe { system_table.ConIn().ReadKeyStroke(&mut key) };

  // TODO: gBS->Stall();

  unsafe {
    system_table.RuntimeServices().ResetSystem(
      EFI_RESET_TYPE::EfiResetShutdown,
      EFI_SUCCESS,
      0usize,
      NULL(),
    )
  };

  EFI_SUCCESS
}
