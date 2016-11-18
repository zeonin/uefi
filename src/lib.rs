#![no_std]

#![allow(non_snake_case,non_camel_case_types)]

pub type EFI_HANDLE = *const ();
pub type EFI_EVENT = *const ();

#[repr(usize)]
pub enum EFI_STATUS {
    EFI_SUCCESS                     = 0x0000000000000000,
    EFI_WARN_UNKNOWN_GLYPH          = 0x0000000000000001,
    EFI_WARN_DELETE_FAILURE         = 0x0000000000000002,
    EFI_WARN_WRITE_FAILURE          = 0x0000000000000003,
    EFI_WARN_BUFFER_TOO_SMALL       = 0x0000000000000004,
    EFI_WARN_STALE_DATA             = 0x0000000000000005,
    EFI_LOAD_ERROR                  = 0x8000000000000001,
    EFI_INVALID_PARAMETER           = 0x8000000000000002,
    EFI_UNSUPPORTED                 = 0x8000000000000003,
    EFI_BAD_BUFFER_SIZE             = 0x8000000000000004,
    EFI_BUFFER_TOO_SMALL            = 0x8000000000000005,
    EFI_NOT_READY                   = 0x8000000000000006,
    EFI_DEVICE_ERROR                = 0x8000000000000007,
    EFI_WRITE_PROTECTED             = 0x8000000000000008,
    EFI_OUT_OF_RESOURCES            = 0x8000000000000009,
    EFI_VOLUME_CORRUPTED            = 0x800000000000000a,
    EFI_VOLUME_FULL                 = 0x800000000000000b,
    EFI_NO_MEDIA                    = 0x800000000000000c,
    EFI_MEDIA_CHANGED               = 0x800000000000000d,
    EFI_NOT_FOUND                   = 0x800000000000000e,
    EFI_ACCESS_DENIED               = 0x800000000000000f,
    EFI_NO_RESPONSE                 = 0x8000000000000010,
    EFI_NO_MAPPING                  = 0x8000000000000011,
    EFI_TIMEOUT                     = 0x8000000000000012,
    EFI_NOT_STARTED                 = 0x8000000000000013,
    EFI_ALREADY_STARTED             = 0x8000000000000014,
    EFI_ABORTED                     = 0x8000000000000015,
    EFI_ICMP_ERROR                  = 0x8000000000000016,
    EFI_TFTP_ERROR                  = 0x8000000000000017,
    EFI_PROTOCOL_ERROR              = 0x8000000000000018,
    EFI_INCOMPATIBLE_VERSION        = 0x8000000000000019,
    EFI_SECURITY_VIOLATION          = 0x800000000000001a,
    EFI_CRC_ERROR                   = 0x800000000000001b,
    EFI_END_OF_MEDIA                = 0x800000000000001c,
    EFI_END_OF_FILE                 = 0x800000000000001f,
    EFI_INVALID_LANGUAGE            = 0x8000000000000020,
    EFI_COMPROMISED_DATA            = 0x8000000000000021,
    EFI_IP_ADDRESS_CONFLICT         = 0x8000000000000022,
}

#[allow(dead_code)]
pub struct EFI_TABLE_HEADER {
    pub Signature:              u64,
    pub Revision:               u32,
    pub HeaderSize:             u32,
    pub CRC32:                  u32,
    Reserved:                   u32,
}

pub struct EFI_INPUT_KEY {
    pub ScanCode:               u16,
    pub UnicodeChar:            u16,
}

pub type EFI_INPUT_RESET = extern fn(*const EFI_SIMPLE_TEXT_INPUT_PROTOCOL, bool) -> EFI_STATUS;
pub type EFI_INPUT_READ_KEY = extern fn(*const EFI_SIMPLE_TEXT_INPUT_PROTOCOL, *const EFI_INPUT_KEY) -> EFI_STATUS;

pub struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL  {
    pub Reset:                  EFI_INPUT_RESET,
    pub ReadKeyStroke:          EFI_INPUT_READ_KEY,
    pub WaitForKey:             EFI_EVENT,
}

pub type EFI_TEXT_RESET                 = extern fn(*const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, bool) -> EFI_STATUS;
pub type EFI_TEXT_STRING                = extern fn(*const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, *const u16) -> EFI_STATUS;
pub type EFI_TEXT_TEST_STRING           = extern fn(*const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, *const u16) -> EFI_STATUS;
pub type EFI_TEXT_QUERY_MODE            = extern fn(*const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, usize, *mut usize, *mut usize) -> EFI_STATUS;
pub type EFI_TEXT_SET_MODE              = extern fn(*const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, usize) -> EFI_STATUS;
pub type EFI_TEXT_SET_ATTRIBUTE         = extern fn(*const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, usize) -> EFI_STATUS;
pub type EFI_TEXT_CLEAR_SCREEN          = extern fn(*const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL) -> EFI_STATUS;
pub type EFI_TEXT_SET_CURSOR_POSITION   = extern fn(*const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, usize, usize) -> EFI_STATUS;
pub type EFI_TEXT_ENABLE_CURSOR         = extern fn(*const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, bool) -> EFI_STATUS;

pub struct SIMPLE_TEXT_OUTPUT_MODE {
    pub MaxMode:                i32,
    pub Mode:                   i32,
    pub Attribute:              i32,
    pub CursorColumn:           i32,
    pub CursorRow:              i32,
    pub CursorVisible:          bool,
}

pub struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    pub Reset:                  EFI_TEXT_RESET,
    pub OutputString:           EFI_TEXT_STRING,
    pub TestString:             EFI_TEXT_TEST_STRING,
    pub QueryMode:              EFI_TEXT_QUERY_MODE,
    pub SetMode:                EFI_TEXT_SET_MODE,
    pub SetAttribute:           EFI_TEXT_SET_ATTRIBUTE,
    pub ClearScreen:            EFI_TEXT_CLEAR_SCREEN,
    pub SetCursorPosition:      EFI_TEXT_SET_CURSOR_POSITION,
    pub EnableCursor:           EFI_TEXT_ENABLE_CURSOR,
    pub Mode:                   *const SIMPLE_TEXT_OUTPUT_MODE,
}

pub struct EFI_SYSTEM_TABLE {
    pub Hdr:                    EFI_TABLE_HEADER,
    pub FirmwareVendor:         *const u16,
    pub FirmwareRevision:       u32,
    pub ConsoleInHandle:        EFI_HANDLE,
    pub ConIn:                  *const EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    pub ConsoleOutHandle:       EFI_HANDLE,
    pub ConOut:                 *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    pub StandardErrorHandle:    EFI_HANDLE,
    pub StdErr:                 *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    //RuntimeServices:        *const EFI_RUNTIME_SERVICES,
    //BootServices:           *const EFI_BOOT_SERVICES,
    //NumberOfTableEntries:   usize,
    //ConfigurationTable:     *const EFI_CONFIGURATION_TABLE,
}
