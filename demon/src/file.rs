use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};
use core::iter::FusedIterator;
use core::mem::zeroed;
use core::ops::Div;
use core::ptr::{null, null_mut};
use windows_sys::Win32::Foundation::{CloseHandle, FALSE, GENERIC_READ, HANDLE, INVALID_HANDLE_VALUE, TRUE};
use windows_sys::Win32::Storage::FileSystem::{CreateFileW, FindClose, FindFirstFileW, FindNextFileW, ReadFile, SetFilePointer, FILE_ATTRIBUTE_NORMAL, FILE_BEGIN, FILE_SHARE_READ, OPEN_ALWAYS, OPEN_EXISTING, WIN32_FIND_DATAW};
use crate::util::encode_utf16;

const WIN32_PATH_SEPARATOR: char = '\\';

#[derive(Clone)]
pub struct Path {
    inner: String
}

impl Path {
    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }
    pub fn extension(&self) -> Option<&str> {
        let dot = self.inner.rfind(".")?;
        Some(&self.inner[dot + 1..])
    }
    pub fn join(&mut self, with: &str) {
        if !self.inner.ends_with(WIN32_PATH_SEPARATOR) {
            self.inner.push(WIN32_PATH_SEPARATOR);
        }
        self.inner += with;
    }
    pub fn filename(&self) -> &str {
        let start = self.inner.rfind(WIN32_PATH_SEPARATOR).unwrap_or(0);
        &self.inner[start + 1..]
    }
    pub fn basename(&self) -> &str {
        let filename = self.filename();
        let dot = filename.rfind('.').unwrap_or(filename.len());
        &filename[..dot]
    }
}

impl Div<&str> for &Path {
    type Output = Path;

    fn div(self, rhs: &str) -> Self::Output {
        let mut c = self.clone();
        c.join(rhs);
        c
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.inner)
    }
}

impl From<String> for Path {
    fn from(mut value: String) -> Self {
        while value.ends_with(WIN32_PATH_SEPARATOR) {
            value.truncate(value.len() - 1);
        }

        Self { inner: value }
    }
}

pub struct Win32DirectoryIterator {
    find_data: WIN32_FIND_DATAW,
    handle: HANDLE,
    base_path: Path
}
impl Win32DirectoryIterator {
    pub fn new(path: &str) -> Option<Self> {
        let mut find_data: WIN32_FIND_DATAW = unsafe { zeroed() };
        let path_utf16: Vec<u16> = format!("{path}\\*\x00").encode_utf16().collect();
        let handle = unsafe { FindFirstFileW(path_utf16.as_ptr(), &mut find_data) };
        if handle.is_null() {
            return None
        }
        Some(Self {
            find_data,
            handle,
            base_path: Path::from(path.to_owned())
        })
    }
}

impl FusedIterator for Win32DirectoryIterator {}

impl Iterator for Win32DirectoryIterator {
    type Item = Path;

    fn next(&mut self) -> Option<Self::Item> {
        if self.handle.is_null() {
            return None
        }

        let mut file_name = String::from_utf16(&self.find_data.cFileName)
            .expect("-.- win32 returned a non-utf16 path");

        if let Some(f) = file_name.find(0 as char) {
            file_name.truncate(f);
        }

        let succeed = unsafe { FindNextFileW(self.handle, &mut self.find_data) } == TRUE;
        if !succeed {
            unsafe { FindClose(self.handle) };
            self.handle = null_mut();
        }

        if file_name == "." || file_name == ".." || file_name == "" {
            return self.next()
        }

        Some(&self.base_path / file_name.as_str())
    }
}

pub fn read_partial_data_from_file<'a>(file: &Path, into: &'a mut [u8], offset: usize) -> Option<&'a mut [u8]> {
    let path = encode_utf16::<512>(file.as_str());

    let handle = unsafe { CreateFileW(
        path.as_ptr(),
        GENERIC_READ,
        FILE_SHARE_READ,
        null(),
        OPEN_ALWAYS,
        FILE_ATTRIBUTE_NORMAL,
        null_mut()
    ) };

    if handle == INVALID_HANDLE_VALUE {
        return None
    }

    unsafe { SetFilePointer(handle, offset as i32, null_mut(), FILE_BEGIN) };

    let mut bytes_read = 0;
    let success = unsafe { ReadFile(
        handle,
        into.as_mut_ptr(),
        into.len() as u32,
        &mut bytes_read,
        null_mut()
    ) };

    unsafe { CloseHandle(handle) };

    (success == TRUE).then_some(&mut into[..bytes_read as usize])
}


pub fn read_all_data_from_file<'a>(file: &Path) -> Option<Vec<u8>> {
    let chunk_size = 65536;
    let mut data = Vec::with_capacity(chunk_size);
    let path = encode_utf16::<512>(file.as_str());

    let handle = unsafe { CreateFileW(
        path.as_ptr(),
        GENERIC_READ,
        FILE_SHARE_READ,
        null(),
        OPEN_EXISTING,
        FILE_ATTRIBUTE_NORMAL,
        null_mut()
    ) };

    if handle == INVALID_HANDLE_VALUE {
        return None
    }

    loop {
        let mut bytes_read = 0;
        if data.try_reserve(chunk_size).is_err() {
            unsafe { CloseHandle(handle) };
            return None
        }

        let old_len = data.len();
        data.resize(old_len + chunk_size, 0);

        let success = unsafe { ReadFile(
            handle,
            data[old_len..].as_mut_ptr(),
            chunk_size as u32,
            &mut bytes_read,
            null_mut()
        ) };

        if success == FALSE {
            unsafe { CloseHandle(handle) };
            return None
        }

        if bytes_read == 0 {
            break
        }
    }

    unsafe { CloseHandle(handle) };
    Some(data)
}
