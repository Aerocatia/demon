mod hook;
pub mod c;

use alloc::string::String;
use alloc::vec::Vec;
pub use hook::sudo_write;

use crate::init::hook::init_hooks;
use crate::panic::on_panic;
use crate::util::{get_exe_path, CStrPtr, StaticStringBytes};
use core::ffi::c_void;
use core::ptr::null_mut;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use min32::dllmain;
use min32::set_hook;
use minxp::env::args;
use spin::Lazy;
use windows_sys::Win32::Foundation::HINSTANCE;
use windows_sys::Win32::System::Diagnostics::Debug::{MapFileAndCheckSumA, CHECKSUM_SUCCESS};
use windows_sys::Win32::System::SystemServices;
use windows_sys::Win32::System::Threading::{ExitProcess, GetCurrentProcess, SetProcessDEPPolicy, TerminateProcess, PROCESS_DEP_ENABLE};
use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBoxA, IDCANCEL, MB_ICONEXCLAMATION, MB_OKCANCEL};

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ExeType {
    /// Build that loads scenarios from cache files (maps/*.map)
    Cache,

    /// Build that loads scenarios tag files (tags/*/*/.scenario)
    Tag
}

impl ExeType {
    fn exe_type_from_checksum(checksum: u32) -> Option<Self> {
        match checksum {
            0x0066D124 => Some(ExeType::Cache),
            0x00720EBD => Some(ExeType::Tag),

            // Patched to be a command-line executable
            0x0066D125 => Some(ExeType::Cache),
            0x00720EBE => Some(ExeType::Tag),
            _ => None
        }
    }
}

#[dllmain]
unsafe fn main(_hinstance: HINSTANCE, reason: u32, _reserved: *mut c_void) -> bool {
    match reason {
        SystemServices::DLL_PROCESS_ATTACH => attach_if_not_attached(),
        _ => ()
    }
    true
}

static ATTACHED: AtomicBool = AtomicBool::new(false);
static EXE_TYPE: AtomicU32 = AtomicU32::new(0xFFFFFFFF);

/// Gets the EXE type.
pub fn get_exe_type() -> ExeType {
    get_exe_type_if_available().expect("Not loaded; can't get EXE type! This is a bug. *sad Butterfree noises*")
}

/// Gets the EXE type.
pub fn get_exe_type_if_available() -> Option<ExeType> {
    let exe_type = EXE_TYPE.load(Ordering::Relaxed);
    if exe_type != 0xFFFFFFFF {
        unsafe {
            Some(core::mem::transmute(exe_type))
        }
    }
    else {
        None
    }
}

static COMMAND_LINE_ARGS: Lazy<Vec<CommandLineArg>> = Lazy::new(|| {
    args()
        .skip(1)
        .map(CommandLineArg::new)
        .collect()
});

struct CommandLineArg {
    cstrptr: CStrPtr,
    str: &'static str
}

impl CommandLineArg {
    fn new(v: String) -> Self {
        let mut buffer = v.into_bytes();
        buffer.push(0); // null terminated

        let bytes: &'static [u8] = buffer.leak();
        Self {
            cstrptr: CStrPtr::from_bytes(bytes),
            str: core::str::from_utf8(&bytes[..bytes.len()-1]).unwrap() // remove null terminator from string
        }
    }
}

unsafe impl Send for CommandLineArg {}
unsafe impl Sync for CommandLineArg {}

unsafe fn attach_if_not_attached() {
    if ATTACHED.swap(true, Ordering::Relaxed) {
        return
    }

    set_hook(Some(on_panic));


    // 2b
    SetProcessDEPPolicy(PROCESS_DEP_ENABLE);

    // TODO: add a method for null terminating stuff or just getting null terminated path?
    let exe_path = get_exe_path();
    let mut exe_data = exe_path.into_bytes();
    exe_data.push(0);

    let mut idc = 0;
    let mut actual_checksum = 0;
    let checksum_success = MapFileAndCheckSumA(exe_data.as_ptr(), &mut idc, &mut actual_checksum);
    assert_eq!(checksum_success, CHECKSUM_SUCCESS, "Failed to checksum the exe");

    let Some(exe_type) = ExeType::exe_type_from_checksum(actual_checksum) else {
        panic!("Cannot determine which EXE is being used (checksum = 0x{actual_checksum:08X}). You might be trying to do bullshit!");
    };

    // Set the EXE type for get_exe_type()
    EXE_TYPE.swap(exe_type as u32, Ordering::Relaxed);

    init_hooks();

    crate::ini::INI.try_load();

    if ini_bool!("debug", "messagebox") == Some(true) {
        let mut stats = String::new();

        stats += StaticStringBytes::<256>::from_fmt(format_args!("EXE type: {:?}\n", get_exe_type())).unwrap().as_str();
        stats += StaticStringBytes::<256>::from_fmt(format_args!("EXE checksum: 0x{actual_checksum:08X}\n")).unwrap().as_str();

        let mut stats = stats.into_bytes();
        stats.push(0u8);

        let result = MessageBoxA(
            null_mut(),
            stats.as_ptr(),
            c"debug.messagebox".as_ptr() as *const u8,
            MB_OKCANCEL | MB_ICONEXCLAMATION
        );

        if result == IDCANCEL {
            TerminateProcess(GetCurrentProcess(), 135);
            ExitProcess(135);
        }
    }
}

pub unsafe fn get_command_line_argument_value(argument: &str) -> Option<CStrPtr> {
    COMMAND_LINE_ARGS
        .iter()
        .skip_while(|i| i.str != argument)
        .skip(1)
        .map(|i| i.cstrptr)
        .next()
}

pub unsafe fn has_command_line_argument_value(argument: &str) -> bool {
    COMMAND_LINE_ARGS.iter().any(|i| i.str == argument)
}
