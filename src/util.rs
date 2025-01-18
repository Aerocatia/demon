use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;
use core::mem::transmute_copy;
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

macro_rules! variable {
    {
        name: $name:expr,
        cache_address: $cache:expr,
        tags_address: $tags:expr
    } => {
        crate::util::VariableProvider {
            name: $name,
            cache_address: $cache as *mut _,
            tags_address: $tags as *mut _
        }
    };
    {
        name: $name:expr,
        cache_address: $cache:expr
    } => {
        variable! { name: $name, cache_address: $cache, tags_address: 0 }
    };
    {
        name: $name:expr,
        tags_address: $tags:expr
    } => {
        variable! { name: $name, cache_address: 0, tags_address: $tags }
    };
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

macro_rules! pointer {
    {
        name: $name:expr,
        cache_address: $cache:expr,
        tags_address: $tags:expr
    } => {
        crate::util::PointerProvider {
            name: $name,
            cache_address: $cache,
            tags_address: $tags,
            phantom_data: core::marker::PhantomData
        }
    };
    {
        name: $name:expr,
        cache_address: $cache:expr
    } => {
        pointer! { name: $name, cache_address: $cache, tags_address: 0 }
    };
    {
        name: $name:expr,
        tags_address: $tags:expr
    } => {
        pointer! { name: $name, cache_address: 0, tags_address: $tags }
    };
}

/// Transmutes the given `usize` into a [`T`] depending on EXE type.
///
/// This is useful for transmuting addresses into functions.
pub(crate) struct PointerProvider<T: Sized> {
    pub name: &'static str,
    pub cache_address: usize,
    pub tags_address: usize,
    pub phantom_data: PhantomData<T>
}
impl<T: Sized> PointerProvider<T> {
    pub unsafe fn get(&self) -> T {
        assert!(size_of::<T>() == size_of::<usize>());

        let exe_type = get_exe_type();
        let address = match exe_type {
            ExeType::Cache => self.cache_address,
            ExeType::Tag => self.tags_address
        };

        if address == 0 {
            panic!("trying to get a null VariableProvider ({name}) for exe_type:", name=self.name);
        }

        transmute_copy(&address)
    }
}

/// Write the arguments `fmt` to a byte buffer `bytes`.
///
/// If the byte buffer is not large enough, it will be truncated.
///
/// Returns `Err` if an error occurs (`bytes` will not be modified).
pub fn fmt_to_byte_array<const LEN: usize>(bytes: &mut [u8; LEN], fmt: core::fmt::Arguments) -> core::fmt::Result {
    struct ErrorBuffer<const LEN: usize> {
        offset: usize,
        data: [u8; LEN]
    }
    impl<const LEN: usize> core::fmt::Write for ErrorBuffer<LEN> {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            let max_len = self.data.len();
            let remainder = &mut self.data[self.offset..max_len-1];
            let bytes_to_add = s.as_bytes();
            let bytes = &bytes_to_add[..remainder.len().min(bytes_to_add.len())];
            if !bytes.is_empty() {
                remainder[..bytes.len()].copy_from_slice(bytes);
                self.offset += bytes.len();
            }
            Ok(())
        }
    }

    let mut buffer: ErrorBuffer<LEN> = ErrorBuffer {
        offset: 0,
        data: [0u8; LEN]
    };

    core::fmt::write(&mut buffer, fmt)?;

    *bytes = buffer.data;

    Ok(())
}
