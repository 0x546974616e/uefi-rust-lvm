use core::{ffi::c_void, ptr};

#[allow(non_camel_case_types)]
pub type EFI_EVENT = *const VOID;

pub type VOID = c_void;
pub type BOOLEAN = bool;
pub type CHAR16 = u16;

pub type TodoImageHandle = u64;

/// 😈
pub const NULL: *const VOID = ptr::null();
