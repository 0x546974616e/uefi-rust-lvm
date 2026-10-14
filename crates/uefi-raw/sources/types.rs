#[allow(non_camel_case_types)]
pub type EFI_STATUS = usize;

// typedef VOID* EFI_EVENT;
#[allow(non_camel_case_types)]
pub type EFI_EVENT = *const core::ffi::c_void;

pub type BOOLEAN = bool;
pub type CHAR16 = u16;

pub type TodoImageHandle = u64;
