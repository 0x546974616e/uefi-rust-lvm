use core::{ptr, ffi::c_void};

// #[repr(transparent)]
// #[allow(non_camel_case_types)]
// pub type EFI_STATUS = usize;
// pub struct EFI_STATUS(pub usize);

// typedef VOID* EFI_EVENT;
#[allow(non_camel_case_types)]
pub type EFI_EVENT = *const c_void;

pub type VOID = c_void;
pub type BOOLEAN = bool;
pub type CHAR16 = u16;

pub type TodoImageHandle = u64;

pub const NULL: *const c_void = ptr::null();