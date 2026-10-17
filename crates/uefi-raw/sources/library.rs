#![cfg_attr(not(test), no_std)]

#[macro_use]
mod macros;

mod enums;
mod guid;
mod protocol;
mod status;
mod structs;
mod types;

pub use {enums::*, guid::*, protocol::*, status::*, structs::*, types::*};

pub static mut SYSTEM_TABLE: *const EFI_SYSTEM_TABLE = core::ptr::null();
