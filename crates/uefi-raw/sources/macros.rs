#[macro_export]
macro_rules! uefi_version {
  ($VERSION:literal) => {
    $crate::uefi_version!(@impl "Release " $VERSION)
  };

  ($VERSION:literal, $SECTION:literal) => {
    $crate::uefi_version!(@impl "Release " $VERSION ", Section " $SECTION)
  };

  (@impl $($LITERAL:literal)*) => {
    concat!("-- <i>UEFI Specification, ", $($LITERAL,)* ".</i>")
  };
}

#[macro_export]
macro_rules! uefi_type {
  (
    #[version($($TYPE_VERSION:literal),+)]
    $(#[$TYPE_ATTRIBUTE:meta])*
    $TYPE_VISIBILITY:vis type $TYPE_NAME:ident = $TYPE_TYPE:ty;
  ) => {
    $(#[$TYPE_ATTRIBUTE])*
    #[doc = ""]
    #[doc = $crate::uefi_version!($($TYPE_VERSION),+)]
    $TYPE_VISIBILITY type $TYPE_NAME = $TYPE_TYPE;
  };
}

/// Defines a UEFI enumeration.
///
/// Constructing a Rust enum with an invalid discriminant is undefined behavior.
/// Therefore, to safely handle unknown values returned by the firmware,
/// enumerations must be represented using the (transparent) newtype pattern.
///
/// This macro generates a transparent newtype around a `u32`, as specified by
/// the UEFI Specification (Release 2.11, Section 2.3.1 -- *Data Types*):
///
/// - **\<Enumerated Type\>**
///
///   Element of a standard ANSI C enum type declaration. Type `INT32` or
///   `UINT32`. ANSI C does not define the size of sign of an enum so they
///   should never be used in structures. ANSI C integer promotion rules make
///   `INT32` or `UINT32` interchangeable when passed as an argument to a
///   function.
///
/// # Example
///
/// ```rust
/// # use uefi_raw::uefi_enum;
/// uefi_enum! {
///   #[version(2026.10)]
///   /// Nonante (90)
///   enum Country {
///     /// Brussels.
///     Belgium = 1,
///     /// Bern.
///     Switzerland = 2,
///   }
/// }
///
/// // Simulate an unexpected value.
/// fn get_country() -> Country {
///   Country(1337)
/// }
///
/// // Although the enumeration contains only two variants, we must handle
/// // unexpected values, as the firmware may return an invalid value or a value
/// // introduced in a newer version that our binary does not yet support.
/// let found = match get_country() {
///   Country::Belgium => true,
///   Country::Switzerland => true,
///   _ => false,
/// };
///
/// assert!(!found);
/// ```
#[macro_export]
macro_rules! uefi_enum {
  (
    #[version($($ENUM_VERSION:literal),+)]
    $(#[$ENUM_ATTRIBUTE:meta])*
    $ENUM_VISIBILITY:vis enum $ENUM_NAME:ident {
      $(
        $(#[$VARIANT_ATTRIBUTE:meta])*
        $VARIANT_NAME:ident = $VARIANT_VALUE:literal
      ),* $(,)?
    }
  ) => {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    $(#[$ENUM_ATTRIBUTE])*
    #[doc = ""]
    #[doc = $crate::uefi_version!($($ENUM_VERSION),+)]
    $ENUM_VISIBILITY struct $ENUM_NAME(pub u32);

    impl $ENUM_NAME {
      $(
        $(#[$VARIANT_ATTRIBUTE])*
        $ENUM_VISIBILITY const $VARIANT_NAME: $ENUM_NAME
          = $ENUM_NAME($VARIANT_VALUE);
      )*
    }
  };
}

/// Defines a UEFI protocol function.
///
/// This macro declares an EFI function pointer type and implements a method
/// on the associated protocol structure that invokes the function pointer.
///
/// # Example
///
/// ```rust,no_run
/// # use uefi_raw::{BOOLEAN, EFI_STATUS, uefi_function};
/// uefi_function! {
///   #[version(2026.10)]
///   /// An example of EFI function.
///   fn EFI_EXAMPLE for Example::f(
///     &This,
///     #[in] Extra: BOOLEAN,
///     #[out(optional)] Done: *mut BOOLEAN,
///   ) -> EFI_STATUS;
/// }
///
/// #[repr(C)]
/// struct Example {
///   f: EFI_EXAMPLE,
/// }
///
/// fn call_f(example: &Example) {
///   let mut done = false;
///   // `&self` is automatically forwarded to `This`.
///   let status = unsafe { example.f(true, &mut done) };
///   if status.is_success() && done {
///     // Do something else.
///   }
/// }
/// ```
#[macro_export]
macro_rules! uefi_function {
  (
    #[version($($FUNCTION_VERSION:literal),+)]
    $(#[$FUNCTION_ATTRIBUTE:meta])*
    $FUNCTION_VISIBILITY:vis fn $FUNCTION_NAME:ident
    for $PROTOCOL_NAME:ident::$PROTOCOL_FIELD:ident (
      $(&$PARAMETER_SELF:ident,)?
      $($(#[$(in)? $(out)? $(inout)? $((optional))?])?
        $PARAMETER_NAME:ident: $PARAMETER_TYPE:ty
      ),* $(,)?
    ) $(-> $FUNCTION_RETURN:ty)?;
  ) => {
    $(#[$FUNCTION_ATTRIBUTE])*
    #[doc = ""]
    #[doc = $crate::uefi_version!($($FUNCTION_VERSION),+)]
    $FUNCTION_VISIBILITY type $FUNCTION_NAME =
      unsafe extern "efiapi" fn(
        $($PARAMETER_SELF: *const $PROTOCOL_NAME,)?
        $($PARAMETER_NAME: $PARAMETER_TYPE),*
      ) $(-> $FUNCTION_RETURN)?;

    impl $PROTOCOL_NAME {
      $(#[$FUNCTION_ATTRIBUTE])*
      /// # Safety
      /// The caller must ensure that the protocol instance and all
      /// arguments satisfy the requirements of the UEFI specification.
      #[doc = ""]
      #[doc = $crate::uefi_version!($($FUNCTION_VERSION),+)]
      $FUNCTION_VISIBILITY unsafe fn $PROTOCOL_FIELD (
        &self, $($PARAMETER_NAME: $PARAMETER_TYPE),*
      )  $(-> $FUNCTION_RETURN)? {
        unsafe {
          (self.$PROTOCOL_FIELD)(
            $({ let $PARAMETER_SELF = self; $PARAMETER_SELF },)?
            $($PARAMETER_NAME),*
          )
        }
      }
    }
  };
}

/// Defines a C-compatible UEFI structure.
///
/// Field type annotations control how field types are generated:
///
/// - `#[todo]`: Uses `*const ()` as a placeholder for an unspecified type.
/// - `#[pointer]`: Uses `*const T` and generates a getter returning `&T`.
/// - `#[function]`: Uses the specified type directly, typically for function
///   pointers declared with [`uefi_function!`].
/// - No annotation: Uses the specified type unchanged.
///
/// # Example
///
/// ```rust,no_run
/// # use uefi_raw::{UINTN, uefi_struct};
/// uefi_struct! {
///   #[version(2026.10)]
///   /// Structure example.
///   pub struct Example {
///     Data: #[pointer] UINTN,
///     Function: #[todo],
///     Value: UINTN,
///   }
/// }
///
/// fn print_data(example: &Example) {
///   println!("{:X} {}", example.Data(), example.Value);
/// }
/// ```
#[macro_export]
macro_rules! uefi_struct {
  (
    #[version($($STRUCT_VERSION:literal),+)]
    $(#[$STRUCT_ATTRIBUTE:meta])*
    $STRUCT_VISIBILITY:vis struct $STRUCT_NAME:ident {
      $(
        $(#[$FIELD_ATTRIBUTE:meta])*
        $FIELD_VISIBILITY:vis $FIELD_NAME:ident:
          $(#[$TYPE_KIND:tt])? $TYPE_VISIBILITY:vis $($FIELD_TYPE:ty)?
      ),* $(,)?
    }
  ) => {
    #[repr(C)]
    #[derive(Debug)]
    $(#[$STRUCT_ATTRIBUTE])*
    #[doc = ""]
    #[doc = $crate::uefi_version!($($STRUCT_VERSION),+)]
    $STRUCT_VISIBILITY struct $STRUCT_NAME {
      $(
        $(#[$FIELD_ATTRIBUTE])*
        $FIELD_VISIBILITY $FIELD_NAME:
          $crate::uefi_struct! {
            @type $(#[$TYPE_KIND])?
              $($FIELD_TYPE)?
          }
      ),*
    }

    impl $STRUCT_NAME {
      $(
        $crate::uefi_struct! {
          @fn $(#[$TYPE_KIND])?
          $(#[$FIELD_ATTRIBUTE])*
            $TYPE_VISIBILITY $FIELD_NAME()
              $(-> $FIELD_TYPE;)?
        }
      )*
    }
  };

  (@type #[todo]) => { *const () };
  (@type #[pointer] $TYPE:ty) => { *const $TYPE };
  (@type #[function] $TYPE:ty) => { $TYPE };
  (@type $TYPE:ty) => { $TYPE };

  (@fn #[pointer]
    $(#[$ATTRIBUTE:meta])*
    $VISIBILITY:vis $NAME:ident() -> $TYPE:ty;
  ) => {
    $(#[$ATTRIBUTE])*
    $VISIBILITY fn $NAME(&self) -> &$TYPE {
      assert!(!self.$NAME.is_null());
      assert!(self.$NAME.is_aligned());
      // SAFETY: The pointer must be valid, aligned, readable, and
      // remain valid for the lifetime of the returned reference.
      unsafe { self.$NAME.as_ref() }.unwrap()
    }
  };

  (@fn $($TT:tt)*) => {
    // Nothing to do here.
  };
}
