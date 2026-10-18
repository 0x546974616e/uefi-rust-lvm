#![cfg_attr(not(test), no_std)]
#![allow(clippy::doc_overindented_list_items)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
mod macros;

mod guid;
mod status;
mod types;

mod tables {
  pub mod boot;
  pub mod header;
  pub mod runtime;
  pub mod system;
}

mod protocols {
  pub mod text_input;
  pub mod text_output;
  pub mod block_io;
}

pub use {
  guid::*,
  protocols::{text_input::*, text_output::*, block_io::*},
  status::*,
  tables::{boot::*, header::*, runtime::*, system::*},
  types::*,
};

pub static mut SYSTEM_TABLE: *const EFI_SYSTEM_TABLE = core::ptr::null();
