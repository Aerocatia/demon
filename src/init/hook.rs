use crate::init::{get_exe_type, ExeType};
use crate::util::CFunctionProvider;
use windows_sys::Win32::Foundation::TRUE;
use windows_sys::Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE};

type Thunk = [u8; 5];
unsafe fn overwrite_thunk<T: Sized + Copy + 'static>(thunk: *mut Thunk, to: CFunctionProvider<T>) {
    const THUNK_SIZE: usize = size_of::<Thunk>();
    let mut old_flags = 0;

    let result = VirtualProtect(thunk as *const _, THUNK_SIZE, PAGE_EXECUTE_READWRITE, &mut old_flags);
    assert!(result == TRUE, "VirtualProtect PAGE_EXECUTE_READWRITE failed for 0x{:08X}", thunk as usize);

    let thunk_val = &mut *thunk;
    assert_eq!(0xE9, thunk_val[0], "No JMP dword instruction for thunk at 0x{:08X}", thunk as usize);

    let offset = (to.address() as usize).wrapping_sub(thunk as usize + THUNK_SIZE);
    thunk_val[1..].copy_from_slice(&offset.to_ne_bytes());

    let result = VirtualProtect(thunk as *const _, THUNK_SIZE, old_flags, &mut old_flags);
    assert!(result == TRUE, "VirtualProtect restore failed for 0x{:08X}", thunk as usize);
}

pub unsafe fn init_hooks() {
    // TODO: codegen from JSON
    if get_exe_type() == ExeType::Tag {
        overwrite_thunk(0x00403AB2 as *mut _, crate::table::iterator_next);
    }
    else if get_exe_type() == ExeType::Cache {
        overwrite_thunk(0x004047FF as *mut _, crate::table::iterator_next);
    }
}
