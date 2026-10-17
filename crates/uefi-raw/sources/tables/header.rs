uefi_struct! {
  #[version(2.11, "4.2.1")]
  /// Data structure that precedes all of the standard EFI table types.
  ///
  /// **NOTE**: The capabilities found in the EFI system table, runtime table
  /// and boot services table may change over time. The first field in each of
  /// these tables is an `EFI_TABLE_HEADER`. This header’s `Revision` field is
  /// incremented when new capabilities and functions are added to the functions
  /// in the table. When checking for capabilities, code should verify that
  /// `Revision` is greater than or equal to the revision level of the table at
  /// the point when the capabilities were added to the UEFI specification.
  ///
  /// **NOTE**: Unless otherwise specified, UEFI uses a standard CCITT32 CRC
  /// algorithm with a seed polynomial value of `0x04c11db7` for its CRC
  /// calculations.
  ///
  /// **NOTE**: The size of the system table, runtime services table, and boot
  /// services table may increase over time. It is very important to always use
  /// the `HeaderSize` field of the `EFI_TABLE_HEADER` to determine the size of
  /// these tables.
  pub struct EFI_TABLE_HEADER {
    /// A 64-bit signature that identifies the type of table that follows.
    /// Unique signatures have been generated for the EFI System Table, the EFI
    /// *Boot Services Table*, and the *EFI Runtime Services Table*.
    Signature: u64,

    /// The revision of the EFI Specification to which this table conforms.
    ///
    /// The upper 16 bits of this field contain the major revision value, and
    /// the lower 16 bits contain the minor revision value. The minor revision
    /// value is a decimal value split into two parts, the "*upper decimal*" and
    /// the "*lower decimal*". The upper decimal is calculated dividing the
    /// minor revision value by 10 using integer division. The lower decimal is
    /// calculated by taking the minor revision value modulo 10.
    ///
    /// When printed or displayed UEFI spec revision is referred as {**Major
    /// revision**}.{**Minor revision upper digits**}.{**Minor revision lowest
    /// digit**} or {**Major revision**}.{**Minor revision upper digits**} in
    /// case Minor revision lowest digit is set to 0. For example:
    ///
    /// * Specification revision value `((2<<16) | (10))` would be referred
    ///   to as **2.1**;
    /// * Specification revision value `((2<<16) | (30))` would be referred
    ///   to as **2.3**;
    /// * Specification revision value `((2<<16) | (31))` would be referred
    ///   to as **2.3.1**;
    /// * Specification revision value `((2<<16) | (100))` would be referred
    ///   to as **2.10**;
    /// * Specification revision value `((2<<16) | (101))` would be referred
    ///   to as **2.10.1**.
    ///
    /// Note that EFI **1.10** did not follow this convention and was referred
    /// to as **1.10**, not **1.1**.
    Revision: u32,

    /// The size, in bytes, of the entire table including the `EFI_TABLE_HEADER`.
    HeaderSize: u32,

    /// The 32-bit CRC for the entire table. This value is computed by setting
    /// this field to 0, and computing the 32-bit CRC for `HeaderSize` bytes.
    CRC32: u32,

    /// Reserved field that must be set to `0`.
    Reserved: u32,
  }
}
