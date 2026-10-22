#![no_std]
#![no_main]
#![allow(unused)] // TODO TMP

use core::{
  panic::PanicInfo,
  ptr::{addr_of, addr_of_mut},
  slice,
};

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

use {uefi_print::*, uefi_raw::*, uefi_string::L};

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
  // println!("{:#?}", stdout.Mode());
  println!("{system_table:#X?}");
  // EFI_ACCESS_DENIED.unwrap();

  let mut HandleCount = 0usize;
  let mut HandleBuffer: *const EFI_HANDLE = NULL();
  let status = unsafe {
    boot_services.LocateHandleBuffer(
      EFI_LOCATE_SEARCH_TYPE::ByProtocol,
      &EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID,
      NULL(),
      &mut HandleCount,
      &mut HandleBuffer,
    )
  };

  if status.is_success() {
    for handle in unsafe { slice::from_raw_parts(HandleBuffer, HandleCount) } {
      let mut text_out: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL = NULL();

      // ⚠️ WARNING:
      // First casting to `c_void` and then taking a mutable reference
      // (`&mut (text_out.cast::<VOID>())`) creates a reference to a
      // temporary value. The original `text_out` will be not modified.
      let status = unsafe {
        boot_services.HandleProtocol(
          *handle,
          &EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID,
          addr_of_mut!(text_out).cast(),
        )
      };

      if status.is_success() && text_out == stdout {
        println!("Ok: {text_out:?}");
        unsafe { (*text_out).OutputString(L!("Zażółć gęślą jaźń!")) };
      }
    }

    unsafe { boot_services.FreePool(HandleBuffer.cast()) }.unwrap();
  }

  let mut index = 0usize;
  let mut key = EFI_INPUT_KEY::default();
  let event = system_table.ConIn().WaitForKey;
  print!("\nPress any key to exit...\n");
  unsafe { system_table.ConIn().Reset(false) }.unwrap();
  unsafe { boot_services.WaitForEvent(1, &event, &mut index) }.unwrap();
  unsafe { system_table.ConIn().ReadKeyStroke(&mut key) }.unwrap();

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
