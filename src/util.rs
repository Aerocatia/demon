use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::{Debug, Formatter};
use core::ptr::{null, null_mut};
use windows_sys::Win32::Foundation::{CloseHandle, FALSE, GENERIC_WRITE, MAX_PATH};
use windows_sys::Win32::Storage::FileSystem::{WriteFile, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL};
use windows_sys::Win32::System::LibraryLoader::GetModuleFileNameA;
use windows_sys::Win32::UI::Shell::PathRemoveFileSpecA;
use crate::init::{get_exe_type, ExeType};

pub fn get_exe_path() -> String {
    // TODO: prepend with \\?\ to bypass max path
    let mut path = [0u8; 1 + MAX_PATH as usize];
    unsafe { GetModuleFileNameA(null_mut(), path.as_mut_ptr(), path.len() as u32); }

    core::ffi::CStr::from_bytes_until_nul(&path)
        .expect("should have gotten something")
        .to_str()
        .expect("non-utf8 exe path???")
        .to_string()
}

pub fn get_exe_dir() -> String {
    unsafe {
        // TODO: prepend with \\?\ to bypass max path
        let mut path = [0u8; 1 + MAX_PATH as usize];
        GetModuleFileNameA(null_mut(), path.as_mut_ptr(), path.len() as u32);
        PathRemoveFileSpecA(path.as_mut_ptr());

        core::ffi::CStr::from_bytes_until_nul(&path)
            .expect("should have gotten something")
            .to_str()
            .expect("non-utf8 exe path???")
            .to_string()
    }
}

pub fn write_to_file(path: &str, what: &[u8]) -> Result<(), &'static str> {
    let mut new_path = Vec::with_capacity(path.len() + 1);
    new_path.extend_from_slice(path.as_bytes());
    new_path.push(0);

    unsafe {
        let file = windows_sys::Win32::Storage::FileSystem::CreateFileA(
            new_path.as_ptr(),
            GENERIC_WRITE,
            0,
            null(),
            CREATE_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            null_mut()
        );

        if file.is_null() {
            return Err("Failed to open the file!");
        }

        let result = WriteFile(file, what.as_ptr(), what.len() as u32, null_mut(), null_mut());

        CloseHandle(file);

        if result == FALSE {
            Err("Failed to write to the file!")
        }
        else {
            Ok(())
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct CFunctionProvider<T: Sized> {
    pub name: &'static str,
    pub function_getter: fn() -> T,
    pub address_getter: fn(T) -> *const ()
}
impl<T: Sized> CFunctionProvider<T> {
    pub fn get_name(&self) -> &'static str {
        self.name
    }
    pub fn get(&self) -> T {
        (self.function_getter)()
    }
    pub fn address(&self) -> *const () {
        (self.address_getter)(self.get())
    }
}
impl<T: Sized> Debug for CFunctionProvider<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("CFunction({})", self.get_name()))
    }
}

pub(crate) struct VariableProvider<T: Sized> {
    pub name: &'static str,
    pub cache_address: *mut T,
    pub tags_address: *mut T
}
impl<T: Sized> VariableProvider<T> {
    pub unsafe fn get(&self) -> &'static T {
        let exe_type = get_exe_type();
        let address = match exe_type {
            ExeType::Cache => self.cache_address,
            ExeType::Tag => self.tags_address
        };
        if address.is_null() {
            panic!("trying to get a null VariableProvider ({name}) for exe_type:", name=self.name);
        }
        &*address
    }
    pub unsafe fn get_mut(&self) -> &'static mut T {
        let exe_type = get_exe_type();
        let address = match exe_type {
            ExeType::Cache => self.cache_address,
            ExeType::Tag => self.tags_address
        };
        if address.is_null() {
            panic!("trying to get a mutable null VariableProvider ({name}) for exe_type:", name=self.name);
        }
        &mut *address
    }
}