/// Transforms
///
/// ```rust
/// # use uefi_raw::uefi_version;
/// ```
#[macro_export]
macro_rules! uefi_version {
  ($VERSION:literal) => {
    concat!("-- <i>UEFI Specification, Release ", $VERSION, ".</i>")
  };

  ($VERSION:literal, $SECTION:literal) => {
    concat!(
      "-- <i>UEFI Specification, Release ",
      $VERSION,
      ", Section ",
      $SECTION,
      ".</i>"
    )
  };
}

#[macro_export]
macro_rules! uefi_enum {
  (
    #[version($($ENUM_VERSION:literal),+)]
    $(#[$ENUM_ATTRIBUTE:meta])*
    $ENUM_VISIBILITY:vis enum $ENUM_NAME:ident {
      $(
        $(#[$VARIANT_ATTRIBUTE:meta])*
        $VARIANT_NAME:ident $(= $VARIANT_VALUE:literal)?
      ),* $(,)?
    }
  ) => {
    #[repr(C)]
    #[derive(Debug)]
    $(#[$ENUM_ATTRIBUTE])*
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    #[doc = ""]
    #[doc = $crate::uefi_version!($($ENUM_VERSION),+)]
    $ENUM_VISIBILITY enum $ENUM_NAME {
      $(
        $(#[$VARIANT_ATTRIBUTE])*
        $VARIANT_NAME $(= $VARIANT_VALUE)?
      ),*
    }
  };
}

#[macro_export]
macro_rules! uefi_struct {
  (@type #[todo]) => { *const () };
  (@type #[pointer] $TYPE:ty) => { *const $TYPE };
  (@type $TYPE:ty) => { $TYPE };

  (
    #[version($($STRUCT_VERSION:literal),+)]
    $(#[$STRUCT_ATTRIBUTE:meta])*
    $STRUCT_VISIBILITY:vis struct $STRUCT_NAME:ident {
      $(
        $(#[$FIELD_ATTRIBUTE:meta])*
        $FIELD_VISIBILITY:vis $FIELD_NAME:ident:
          $(#[$TYPE_ATTRIBUTE:tt])? $TYPE_VISIBILITY:vis
            $($FIELD_TYPE:ty)?
      ),* $(,)?
    }
  ) => {
    #[repr(C)]
    #[derive(Debug)]
    $(#[$STRUCT_ATTRIBUTE])*
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    #[doc = ""]
    #[doc = $crate::uefi_version!($($STRUCT_VERSION),+)]
    $STRUCT_VISIBILITY struct $STRUCT_NAME {
      $(
        $(#[$FIELD_ATTRIBUTE])*
        $FIELD_VISIBILITY $FIELD_NAME:
          $crate::uefi_struct! {
            @type $(#[$TYPE_ATTRIBUTE])?
              $($FIELD_TYPE)?
          }
      ),*
    }

    impl $STRUCT_NAME {
      $(
        $crate::uefi_struct! {
          @fn $(#[$TYPE_ATTRIBUTE])?
            $(#[$FIELD_ATTRIBUTE])*
            $TYPE_VISIBILITY $FIELD_NAME()
              -> $($FIELD_TYPE)?;
        }
      )*
    }
  };

  (@fn #[pointer]
    $(#[$FIELD_ATTRIBUTE:meta])*
    $VISIBILITY:vis $NAME:ident() -> $TYPE:ty;
  ) => {
    $(#[$FIELD_ATTRIBUTE])*
    #[allow(non_snake_case)]
    $VISIBILITY fn $NAME(&self) -> &$TYPE {
      // SAFETY: Callers know what they are doing...
      unsafe { self.$NAME.as_ref() }.unwrap()
    }
  };

  (@fn $($TT:tt)*) => {
    // Nothing to do here.
  };
}

#[macro_export]
macro_rules! uefi_protocol {
  (
    $(
      #[guid]
      $GUID_VISIBILITY:vis const $GUID_NAME:ident = {
        $GUID_DATA_1:literal, $GUID_DATA_2:literal, $GUID_DATA_3:literal,
        { $($GUID_DATA_4:literal),* $(,)? } $(,)?
      } $(;)?
    )?

    #[version($($PROTOCOL_VERSION:literal),+)]
    $(#[$PROTOCOL_ATTRIBUTE:meta])*
    $PROTOCOL_VISIBILITY:vis struct $PROTOCOL_NAME:ident {
      $(
        $(#[$FIELD_ATTRIBUTE:meta])*
        $FIELD_VISIBILITY:vis $FIELD_NAME:ident
          $(:#[todo$($FIELD_TODO:tt)?])?
          $(:#[pointer] $POINTER_VISIBILITY:vis $FIELD_POINTER:ty)?
          $(:$FIELD_TYPE:ty)?
          $(:{
            #[version($($FUNCTION_VERSION:literal),+)]
            $( #[$FUNCTION_ATTRIBUTE:meta] )*
            $FUNCTION_VISIBILITY:vis fn $FUNCTION_NAME:ident(
              $(&$PARAMETER_SELF:ident)? $(,)?
              $( $(#[$(in $(,)?)? $(out $(,)?)? $(optional)?])?
                $PARAMETER_NAME:ident: $PARAMETER_TYPE:ty
              ),* $(,)?
            ) $(-> $FUNCTION_RETURN:ty)?
          })?
      ),* $(,)?
    }
  ) => {
    $(
      $GUID_VISIBILITY const $GUID_NAME: EFI_GUID = EFI_GUID {
        Data1: $GUID_DATA_1, Data2: $GUID_DATA_2, Data3: $GUID_DATA_3,
        Data4: [ $($GUID_DATA_4),* ],
      };
    )?

    // NOTE:
    // The metavariable `$FUNCTION_NAME` is an `ident` and cannot be passed
    // to `uefi_struct!{}`, which only accepts `$FIELD_TYPE:ty`. We therefore
    // copy the struct declaration here...

    #[repr(C)]
    #[derive(Debug)]
    $(#[$PROTOCOL_ATTRIBUTE])*
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    #[doc = ""]
    #[doc = $crate::uefi_version!($($PROTOCOL_VERSION),+)]
    $PROTOCOL_VISIBILITY struct $PROTOCOL_NAME {
      $(
        $(#[$FIELD_ATTRIBUTE])*
        $FIELD_VISIBILITY $FIELD_NAME:
          $(*const ($($FIELD_TODO)?))?
          $(*const $FIELD_POINTER)?
          $($FUNCTION_NAME)?
          $($FIELD_TYPE)?
      ),*
    }

    $(
      $(
        $(#[$FUNCTION_ATTRIBUTE])*
        #[allow(non_snake_case)]
        #[allow(non_camel_case_types)]
        #[doc = ""]
        #[doc = $crate::uefi_version!($($FUNCTION_VERSION),+)]
        $FUNCTION_VISIBILITY type $FUNCTION_NAME =
          unsafe extern "efiapi" fn(
            $($PARAMETER_SELF: *const $PROTOCOL_NAME,)?
            $($PARAMETER_NAME: $PARAMETER_TYPE),*
          ) $(-> $FUNCTION_RETURN)?;
      )?
    )*

    impl $PROTOCOL_NAME {
      $(
        $(
          $(#[$FUNCTION_ATTRIBUTE])*
          #[allow(non_snake_case)]
          #[doc = ""]
          #[doc = $crate::uefi_version!($($FUNCTION_VERSION),+)]
          $FUNCTION_VISIBILITY unsafe fn $FIELD_NAME(
            &self, $($PARAMETER_NAME: $PARAMETER_TYPE),*
          ) $(-> $FUNCTION_RETURN)? {
            unsafe {
              (self.$FIELD_NAME)(
                $({ let $PARAMETER_SELF = self; $PARAMETER_SELF },)?
                $($PARAMETER_NAME),*
              )
            }
          }
        )?

        $crate::uefi_struct! {
          @fn #[pointer]
            $(#[$FIELD_ATTRIBUTE])*
            $($POINTER_VISIBILITY)? $FIELD_NAME()
              -> $($FIELD_POINTER)?;
        }
      )*
    }
  };
}
