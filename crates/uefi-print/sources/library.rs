#![cfg_attr(not(test), no_std)]
#![allow(clippy::collapsible_if)]
pub(crate) mod output;

#[macro_export]
macro_rules! print {
  ($($ARGUMENT:tt)*) => {{
    $crate::_print(core::format_args!($($ARGUMENT)*));
  }};
}

#[macro_export]
macro_rules! println {
  () => {{
    $crate::print!("\n");
  }};

  ($($ARGUMENT:tt)*) => {{
    $crate::_println(core::format_args!($($ARGUMENT)*));
  }};
}

#[macro_export]
macro_rules! eprint {
  ($($ARGUMENT:tt)*) => {{
    $crate::_eprint(core::format_args!($($ARGUMENT)*));
  }};
}

#[macro_export]
macro_rules! eprintln {
  () => {{
    $crate::eprint!("\n");
  }};

  ($($ARGUMENT:tt)*) => {{
    $crate::_eprintln(core::format_args!($($ARGUMENT)*));
  }};
}

pub fn _print(arguments: core::fmt::Arguments<'_>) {
  output::print(|table| table.ConOut, arguments, false);
}

pub fn _println(arguments: core::fmt::Arguments<'_>) {
  output::print(|table| table.ConOut, arguments, true);
}

pub fn _eprint(arguments: core::fmt::Arguments<'_>) {
  output::print(|table| table.StdErr, arguments, false);
}

pub fn _eprintln(arguments: core::fmt::Arguments<'_>) {
  output::print(|table| table.StdErr, arguments, true);
}
