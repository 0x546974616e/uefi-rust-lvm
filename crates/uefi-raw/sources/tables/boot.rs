use crate::{EFI_EVENT, EFI_GUID, EFI_HANDLE, EFI_STATUS, EFI_TABLE_HEADER, UINTN, VOID};

uefi_function! {
  #[version(2.11, "7.1.5")]
  /// Stops execution until an event is signaled.
  ///
  /// # Parameters
  ///
  /// * `NumberOfEvents` - The number of events in the `Event` array.
  /// * `Event`          - An array of `EFI_EVENT`.
  /// * `Index`          - Pointer to the index of the event which satisfied
  ///                      the wait condition.
  ///
  /// # Description
  ///
  /// This function must be called at priority level `TPL_APPLICATION`. If an
  /// attempt is made to call it at any other priority level, `EFI_UNSUPPORTED`
  /// is returned.
  ///
  /// The list of events in the `Event` array are evaluated in order from first
  /// to last, and this evaluation is repeated until an event is signaled or an
  /// error is detected. The following checks are performed on each event in the
  /// `Event` array.
  ///
  /// - If an event is of type `EVT_NOTIFY_SIGNAL`, then `EFI_INVALID_PARAMETER`
  ///   is returned and `Index` indicates the event that caused the failure.
  ///
  /// - If an event is in the signaled state, the signaled state is cleared and
  ///   `EFI_SUCCESS` is returned, and `Index` indicates the event that was
  ///   signaled.
  ///
  /// - If an event is not in the signaled state but does have a notification
  ///   function, the notification function is queued at the event's
  ///   notification task priority level. If the execution of the event's
  ///   notification function causes the event to be signaled, then the signaled
  ///   state is cleared, `EFI_SUCCESS` is returned, and `Index` indicates the
  ///   event that was signaled.
  ///
  /// To wait for a specified time, a timer event must be included in the
  /// `Event` array.
  ///
  /// To check if an event is signaled without waiting, an already signaled
  /// event can be used as the last event in the list being checked, or the
  /// `CheckEvent()` interface may be used.
  ///
  /// # Errors
  ///
  /// * `EFI_SUCCESS`           - The event indicated by `Index` was signaled.
  /// * `EFI_INVALID_PARAMETER` - `NumberOfEvents` is 0.
  /// * `EFI_INVALID_PARAMETER` - The event indicated by `Index` is of
  ///                             type `EVT_NOTIFY_SIGNAL`.
  /// * `EFI_UNSUPPORTED`       - The current TPL is not `TPL_APPLICATION`.
  pub fn EFI_WAIT_FOR_EVENT for EFI_BOOT_SERVICES::WaitForEvent(
    #[in] NumberOfEvents: UINTN,
    #[in] Event: *const EFI_EVENT,
    #[out] Index: *mut UINTN,
  ) -> EFI_STATUS;
}

uefi_enum! {
  #[version(2.11, "7.3.6")]
  /// Enumeration of *EFI Locate Search Types*.
  pub enum EFI_LOCATE_SEARCH_TYPE {
    /// Retrieve all the handles in the handle database.
    ///
    /// In `EFI_LOCATE_HANDLE` and `EFI_LOCATE_HANDLE_BUFFER`, `Protocol` and
    /// `SearchKey` are ignored and the function returns an array of every
    /// handle in the system.
    AllHandles = 0,

    /// Retrieve the next handle fron a `RegisterProtocolNotify()` event.
    ///
    /// SearchKey supplies the Registration value returned by
    /// `EFI_BOOT_SERVICES.RegisterProtocolNotify()`. The function returns the
    /// next handle that is new for the registration. Only one handle is
    /// returned at a time, starting with the first, and the caller must loop
    /// until no more handles are returned. Protocol is ignored for this search
    /// type.
    ByRegisterNotify = 1,

    /// Retrieve the set of handles from the handle database that support a
    /// specified protocol.
    ///
    /// All handles that support `Protocol` are returned. `SearchKey` is ignored
    /// for this search type.
    ByProtocol = 2,
  }
}

uefi_function! {
  #[version(2.11, "7.3.15")]
  /// Returns an array of handles that support the requested protocol in a
  /// buffer allocated from pool.
  ///
  /// # Parameters
  ///
  /// * `SearchType` - Specifies which handle(s) are to be returned.
  /// * `Protocol`   - Provides the protocol to search by. This parameter is
  ///                  only valid for a `SearchType` of `ByProtocol`.
  /// * `SearchKey`  - Supplies the search key depending on the `SearchType`.
  /// * `NoHandles`  - The number of handles returned in `Buffer`.
  /// * `Buffer`     - A pointer to the buffer to return the requested array of
  ///                  handles that support `Protocol`.
  ///
  ///   This buffer is allocated with a call to the Boot Service
  ///   `EFI_BOOT_SERVICES.AllocatePool()`. It is the caller's responsibility to
  ///   call the Boot Service `EFI_BOOT_SERVICES.FreePool()` when the caller no
  ///   longer requires the contents of `Buffer`.
  ///
  /// # Description
  ///
  /// The `LocateHandleBuffer()` function returns one or more handles that match
  /// the `SearchType` request. `Buffer` is allocated from pool, and the number
  /// of entries in `Buffer` is returned in `NoHandles`. Each `SearchType` is
  /// described in [`EFI_LOCATE_SEARCH_TYPE`].
  ///
  /// If `NoHandles` or `Buffer` is `NULL`, then `EFI_INVALID_PARAMETER` is
  /// returned.  If there are no handles in the handle database that match the
  /// search criteria, then `EFI_NOT_FOUND` is returned. If there are not
  /// enough resources available to allocate `Buffer`, then
  /// `EFI_OUT_OF_RESOURCES` is returned.
  ///
  /// # Errors
  ///
  /// * `EFI_SUCCESS`           - The array of handles was returned in `Buffer`,
  ///                             and the number of handles in `Buffer` was
  ///                             returned in `NoHandles`.
  /// * `EFI_INVALID_PARAMETER` - `NoHandles` is `NULL`
  /// * `EFI_INVALID_PARAMETER` - `Buffer` is `NULL`
  /// * `EFI_NOT_FOUND`         - No handles match the search.
  /// * `EFI_OUT_OF_RESOURCES`  - There is not enough pool memory to store the
  ///                             matching results.
  pub fn EFI_LOCATE_HANDLE_BUFFER for EFI_BOOT_SERVICES::LocateHandleBuffer(
    #[in] SearchType: EFI_LOCATE_SEARCH_TYPE,
    #[in(optional)] Protocol: *const EFI_GUID,
    #[in(optional)] SearchKey: *const VOID,
    #[out] NoHandles: *mut UINTN,
    #[out] Buffer: *mut *const EFI_HANDLE,
  ) -> EFI_STATUS;
}

uefi_struct! {
  #[version(2.11, "4.4.1")]
  /// Contains a table header and pointers to all of the boot services.
  ///
  /// UEFI uses the *EFI Boot Services Table*, which contains a table header and
  /// pointers to all of the boot services. Except for the table header, all
  /// elements in the *EFI Boot Services Tables* are prototypes of function
  /// pointers to functions as defined in [Chapter 7: Services - Boot
  /// Services](#). The function pointers in this table are not valid after the
  /// operating system has taken control of the platform with a call to
  /// `EFI_BOOT_SERVICES.ExitBootServices()`.
  pub struct EFI_BOOT_SERVICES {
    /// The table header for the EFI Boot Services Table.
    ///
    /// This header contains the `EFI_BOOT_SERVICES_SIGNATURE` and
    /// `EFI_BOOT_SERVICES_REVISION` values along with the size of the
    /// `EFI_BOOT_SERVICES` structure and a 32-bit CRC to verify that the
    /// contents of the *EFI Boot Services Table* are valid.
    Header: EFI_TABLE_HEADER,

    ////////////////////////// Task Priority Services //////////////////////////

    /// Raises the task priority level.
    RaiseTPL: #[todo], // EFI 1.0+

    /// Restores/lowers the task priority level.
    RestoreTPL: #[todo], // EFI 1.0+

    ///////////////////////////// Memory Services //////////////////////////////

    /// Allocates pages of a particular type.
    AllocatePages: #[todo], // EFI 1.0+

    /// Frees allocated pages.
    FreePages: #[todo], // EFI 1.0+

    /// Returns the current boot services memory map and memory map key.
    GetMemoryMap: #[todo], // EFI 1.0+

    /// Allocates a pool of a particular type.
    AllocatePool: #[todo], // EFI 1.0+

    /// Frees allocated pool.
    FreePool: #[todo], // EFI 1.0+

    ////////////////////////// Event & Timer Services //////////////////////////

    /// Creates a general-purpose event structure.
    CreateEvent: #[todo], // EFI 1.0+

    /// Sets an event to be signaled at a particular time.
    SetTimer: #[todo], // EFI 1.0+

    /// Stops execution until an event is signaled.
    WaitForEvent: #[function] EFI_WAIT_FOR_EVENT, // EFI 1.0+

    /// Signals an event.
    SignalEvent: #[todo], // EFI 1.0+

    /// Closes and frees an event structure.
    CloseEvent: #[todo], // EFI 1.0+

    /// Checks whether an event is in the signaled state.
    CheckEvent: #[todo], // EFI 1.0+

    //////////////////////// Protocol Handler Services /////////////////////////

    /// Installs a protocol interface on a device handle.
    InstallProtocolInterface: #[todo], // EFI 1.0+

    /// Reinstalls a protocol interface on a device handle.
    ReinstallProtocolInterface: #[todo], // EFI 1.0+

    /// Removes a protocol interface from a device handle.
    UninstallProtocolInterface: #[todo], // EFI 1.0+

    /// Queries a handle to determine if it supports a specified protocol.
    HandleProtocol: #[todo], // EFI 1.0+

    /// Reserved. Must be `NULL`.
    Reserved: *const VOID, // EFI 1.0+

    /// Registers an event that is to be signaled whenever an interface is
    /// installed for a specified protocol.
    RegisterProtocolNotify: #[todo], // EFI 1.0+

    /// Returns an array of handles that support a specified protocol.
    LocateHandle: #[todo], // EFI 1.0+

    /// Locates all devices on a device path that support a specified protocol
    /// and returns the handle to the device that is closest to the path.
    LocateDevicePath: #[todo], // EFI 1.0+

    /// Adds, updates, or removes a configuration table from the EFI System Table.
    InstallConfigurationTable: #[todo], // EFI 1.0+

    ////////////////////////////// Image Services //////////////////////////////

    /// Loads an EFI image into memory.
    LoadImage: #[todo], // EFI 1.0+

    /// Transfers control to a loaded image’s entry point.
    StartImage: #[todo], // EFI 1.0+

    /// Exits the image’s entry point.
    Exit: #[todo], // EFI 1.0+

    /// Unloads an image.
    UnloadImage: #[todo], // EFI 1.0+

    /// Terminates boot services.
    ExitBootServices: #[todo], // EFI 1.0+

    ////////////////////////// Miscellaneous Services //////////////////////////

    /// Returns a monotonically increasing count for the platform.
    GetNextMonotonicCount: #[todo], // EFI 1.0+

    /// Stalls the processor.
    Stall: #[todo], // EFI 1.0+

    /// Resets and sets a watchdog timer used during boot services time.
    SetWatchdogTimer: #[todo], // EFI 1.0+

    ///////////////////////// Driver Support Services //////////////////////////

    /// Uses a set of precedence rules to find the best set of drivers to manage
    /// a controller.
    ConnectController: #[todo], // EFI 1.1

    /// Informs a set of drivers to stop managing a controller.
    DisconnectController: #[todo], // EFI 1.1+

    ///////////////////// Open and Close Protocol Services /////////////////////

    /// Adds elements to the list of agents consuming a protocol interface.
    OpenProtocol: #[todo], // EFI 1.1+

    /// Removes elements from the list of agents consuming a protocol interface.
    CloseProtocol: #[todo], // EFI 1.1+

    /// Retrieve the list of agents that are currently consuming a protocol
    /// interface.
    OpenProtocolInformation: #[todo], // EFI 1.1+

    ///////////////////////////// Library Services /////////////////////////////

    /// Retrieves the list of protocols installed on a handle. The return buffer
    /// is automatically allocated.
    ProtocolsPerHandle: #[todo], // EFI 1.1+

    /// Retrieves the list of handles from the handle database that meet the
    /// search criteria. The return buffer is automatically allocated.
    LocateHandleBuffer: #[function] EFI_LOCATE_HANDLE_BUFFER, // EFI 1.1+

    /// Finds the first handle in the handle database the supports the requested
    /// protocol.
    LocateProtocol: #[todo], // EFI 1.1+

    /// Installs one or more protocol interfaces onto a handle.
    InstallMultipleProtocolInterfaces: #[todo], // EFI 1.1+

    /// Uninstalls one or more protocol interfaces from a handle.
    UninstallMultipleProtocolInterfaces: #[todo], // EFI 1.1+

    /////////////////////////// 32-bit CRC Services ////////////////////////////

    /// Computes and returns a 32-bit CRC for a data buffer.
    CalculateCrc32: #[todo], // EFI 1.1+

    ////////////////////////// Miscellaneous Services //////////////////////////

    /// Copies the contents of one buffer to another buffer.
    CopyMem: #[todo], // EFI 1.1+

    /// Fills a buffer with a specified value.
    SetMem: #[todo], // EFI 1.1+

    /// Creates an event structure as part of an event group.
    CreateEventEx: #[todo], // EFI 2.0+
  }
}
