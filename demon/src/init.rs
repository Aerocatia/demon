mod hook;
pub mod c;

pub use hook::sudo_write;

use crate::init::hook::init_hooks;
use crate::util::get_exe_path;
use core::ffi::c_void;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use min32::dllmain;
use min32::panic::set_hook;
use windows_sys::Win32::Foundation::HINSTANCE;
use windows_sys::Win32::System::Diagnostics::Debug::{MapFileAndCheckSumA, CHECKSUM_SUCCESS};
use windows_sys::Win32::System::SystemServices;
use windows_sys::Win32::System::Threading::{SetProcessDEPPolicy, PROCESS_DEP_ENABLE};
use crate::panic::on_panic;

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

            // Cache build patched to be a command-line executable
            0x0066D125 => Some(ExeType::Cache),
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
}
