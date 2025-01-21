use crate::init::{get_exe_type, ExeType};
use crate::util::CFunctionProvider;
use windows_sys::Win32::Foundation::TRUE;
use windows_sys::Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE};
use c_mine::*;

const JMP_SIZE: usize = 5;
type JMP = [u8; JMP_SIZE];

unsafe fn overwrite_thunk<T>(name: &str, thunk: *mut JMP, to: CFunctionProvider<T>) {
    let thunk_val = &mut *thunk;
    assert_eq!(0xE9, thunk_val[0], "No JMP dword instruction for thunk `{name}` at 0x{:08X}", thunk as usize);
    write_jmp(name, thunk, to);
}
unsafe fn write_jmp<T>(_name: &str, at: *mut JMP, to: CFunctionProvider<T>) {
    let dst = &mut *at;
    let mut old_flags = 0;

    let result = VirtualProtect(at as *const _, dst.len(), PAGE_EXECUTE_READWRITE, &mut old_flags);
    assert!(result == TRUE, "VirtualProtect PAGE_EXECUTE_READWRITE failed for 0x{:08X}", at as usize);

    let offset = (to.address() as usize).wrapping_sub(at as usize + dst.len());
    dst[1..].copy_from_slice(&offset.to_ne_bytes());
    dst[0] = 0xE9;

    let result = VirtualProtect(at as *const _, dst.len(), old_flags, &mut old_flags);
    assert!(result == TRUE, "VirtualProtect restore failed for 0x{:08X}", at as usize);
}

pub unsafe fn init_hooks() {
    generate_hook_setup_code!();
}
