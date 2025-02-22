use alloc::borrow::Cow;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::char::decode_utf16;
use core::cmp::Ordering;
use core::ffi::{c_char, CStr};
use core::fmt::{Debug, Display, Formatter, Write};
use core::marker::PhantomData;
use core::mem::transmute_copy;
use core::ptr::{null, null_mut};
use windows_sys::Win32::Foundation::{CloseHandle, FALSE, GENERIC_WRITE, MAX_PATH};
use windows_sys::Win32::Globalization::{MultiByteToWideChar, CP_ACP, MB_PRECOMPOSED};
use windows_sys::Win32::Storage::FileSystem::{WriteFile, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL};
use windows_sys::Win32::System::LibraryLoader::GetModuleFileNameA;
use windows_sys::Win32::UI::Shell::PathRemoveFileSpecA;
use crate::init::{get_exe_type, ExeType};

pub fn get_exe_path() -> String {
    // TODO: prepend with \\?\ to bypass max path
    let mut path = [0u8; 1 + MAX_PATH as usize];
    unsafe { GetModuleFileNameA(null_mut(), path.as_mut_ptr(), path.len() as u32); }

    CStr::from_bytes_until_nul(&path)
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

        CStr::from_bytes_until_nul(&path)
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

        let result = WriteFile(file, what.as_ptr(), what.len() as u32, &mut 0, null_mut());

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

/// Decode the UTF-16 array into the target byte array.
///
/// Stops when it reaches the end of `from` or hits a null terminator.
pub fn decode_utf16_inplace<'a>(from: &[u16], to: &'a mut [u8]) -> &'a str {
    let max_length = to.len();
    let mut index = 0;

    let iterator = from
        .iter()
        .copied()
        .take_while(|b| *b != 0);

    for c in decode_utf16(iterator) {
        let character = c.unwrap_or(char::REPLACEMENT_CHARACTER);

        let c_len = character.len_utf8();
        if index + c_len > max_length {
            break;
        }

        character.encode_utf8(&mut to[index..]);
        index += c_len;
    }

    core::str::from_utf8(&to[..index]).expect("we just decoded UTF though")
}

macro_rules! variable {
    {
        name: $name:expr,
        cache_address: $cache:expr,
        tag_address: $tag:expr
    } => {
        crate::util::VariableProvider {
            name: $name,
            cache_address: $cache as *mut _,
            tag_address: $tag as *mut _
        }
    };
    {
        name: $name:expr,
        cache_address: $cache:expr
    } => {
        variable! { name: $name, cache_address: $cache, tag_address: 0 }
    };
    {
        name: $name:expr,
        tag_address: $tag:expr
    } => {
        variable! { name: $name, cache_address: 0, tag_address: $tag }
    };
}

pub(crate) struct VariableProvider<T: Sized> {
    pub name: &'static str,
    pub cache_address: *mut T,
    pub tag_address: *mut T
}
impl<T: Sized> VariableProvider<T> {
    /// Get a reference to the value.
    ///
    /// # Safety
    ///
    /// No guarantees are made that this is being mutated concurrently.
    pub unsafe fn get(&self) -> &'static T {
        let exe_type = get_exe_type();
        let address = match exe_type {
            ExeType::Cache => self.cache_address,
            ExeType::Tag => self.tag_address
        };
        if address.is_null() {
            panic!("trying to get a null VariableProvider ({name}) for this exe type", name=self.name);
        }
        &*address
    }

    /// Get a mutable reference to the value.
    ///
    /// # Safety
    ///
    /// No guarantees are made that this is being mutated concurrently. Also, static mutable
    /// references are unsafe.
    pub unsafe fn get_mut(&self) -> &'static mut T {
        let exe_type = get_exe_type();
        let address = match exe_type {
            ExeType::Cache => self.cache_address,
            ExeType::Tag => self.tag_address
        };
        if address.is_null() {
            panic!("trying to get a mutable null VariableProvider ({name}) for this exe type", name=self.name);
        }
        &mut *address
    }

    /// Get the value, copied bitwise.
    ///
    /// # Safety
    ///
    /// No guarantees are made that this is being mutated concurrently.
    ///
    /// It might also not be safe to copy `T` bitwise.
    pub unsafe fn get_copied(&self) -> T {
        let exe_type = get_exe_type();
        let address = match exe_type {
            ExeType::Cache => self.cache_address,
            ExeType::Tag => self.tag_address
        };
        if address.is_null() {
            panic!("trying to get a mutable null VariableProvider ({name}) for this exe type", name=self.name);
        }
        transmute_copy(&*address)
    }
}

macro_rules! pointer {
    {
        name: $name:expr,
        cache_address: $cache:expr,
        tag_address: $tag:expr
    } => {
        crate::util::PointerProvider {
            name: $name,
            cache_address: $cache,
            tag_address: $tag,
            phantom_data: core::marker::PhantomData
        }
    };
    {
        name: $name:expr,
        cache_address: $cache:expr
    } => {
        pointer! { name: $name, cache_address: $cache, tag_address: 0 }
    };
    {
        name: $name:expr,
        tag_address: $tag:expr
    } => {
        pointer! { name: $name, cache_address: 0, tag_address: $tag }
    };
}

/// Transmutes the given `usize` into a [`T`] depending on EXE type.
///
/// This is useful for transmuting addresses into functions.
pub(crate) struct PointerProvider<T: Sized> {
    pub name: &'static str,
    pub cache_address: usize,
    pub tag_address: usize,
    pub phantom_data: PhantomData<T>
}
impl<T: Sized> PointerProvider<T> {
    pub unsafe fn get(&self) -> T {
        assert!(size_of::<T>() == size_of::<usize>());

        let exe_type = get_exe_type();
        let address = match exe_type {
            ExeType::Cache => self.cache_address,
            ExeType::Tag => self.tag_address
        };

        if address == 0 {
            panic!("trying to get a null VariableProvider ({name}) for this exe type", name=self.name);
        }

        transmute_copy(&address)
    }
}

#[derive(Copy, Clone)]
pub struct StaticStringBytes<const SIZE: usize>([u8; SIZE], usize);

impl<const SIZE: usize> StaticStringBytes<SIZE> {
    pub const fn new() -> Self {
        Self([0u8; SIZE], 0)
    }

    pub fn from_fmt(fmt: core::fmt::Arguments) -> Result<StaticStringBytes<SIZE>, core::fmt::Error> {
        let mut bytes = [0u8; SIZE];
        let length = fmt_to_byte_array(&mut bytes[..SIZE - 1], fmt)?.len();
        Ok(StaticStringBytes(bytes, length))
    }

    pub fn from_strs<'a>(strings: impl Iterator<Item = &'a str>) -> StaticStringBytes<SIZE> {
        let mut bytes = [0u8; SIZE];
        let mut length = 0;
        let max_len = bytes.len() - 1;

        for i in strings {
            let str_bytes = i.as_bytes();
            let end = (length + str_bytes.len()).min(max_len);

            bytes[length..end].copy_from_slice(&str_bytes[0..end - length]);
            length = end;
            if end == max_len {
                break;
            }
        }

        bytes[SIZE - 1] = 0;

        StaticStringBytes(bytes, length)
    }

    pub fn from_utf16(utf16: &[u16]) -> StaticStringBytes<SIZE> {
        let mut bytes = [0u8; SIZE];
        let length = decode_utf16_inplace(utf16, &mut bytes[..SIZE - 1]).len();
        StaticStringBytes(bytes, length)
    }

    pub fn from_display(display: impl Display) -> StaticStringBytes<SIZE> {
        StaticStringBytes::from_fmt(format_args!("{display}")).expect(";-;")
    }

    pub fn as_str(&self) -> &str {
        // Safety: This is guaranteed to be valid UTF-8 because from_fmt and from_strs guarantee this.
        unsafe { core::str::from_utf8_unchecked(self.as_bytes()) }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0[..self.1]
    }

    pub fn as_bytes_with_null(&self) -> &[u8] {
        &self.0[..self.1 + 1]
    }

    pub const fn into_bytes(self) -> [u8; SIZE] {
        self.0
    }

    pub const fn byte_len(&self) -> usize {
        self.1
    }
}

impl<const SIZE: usize> Default for StaticStringBytes<SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const SIZE: usize> Display for StaticStringBytes<SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let str = core::str::from_utf8(&self.0[..self.1])
            .expect("fmt_to_allocated_byte_array exploded");
        f.write_str(str)
    }
}

/// Write the arguments `fmt` to a byte buffer `bytes`, returning a string reference backed by `bytes`.
///
/// If the byte buffer is not large enough, it will be truncated.
///
/// Returns `Err` if an error occurs (`bytes` may be modified).
pub fn fmt_to_byte_array<'a>(bytes: &'a mut [u8], fmt: core::fmt::Arguments) -> Result<&'a str, core::fmt::Error> {
    struct ErrorBuffer<'a> {
        offset: usize,
        data: &'a mut [u8]
    }
    impl<'a> core::fmt::Write for ErrorBuffer<'a> {
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

    let mut buffer = ErrorBuffer {
        offset: 0,
        data: bytes
    };

    core::fmt::write(&mut buffer, fmt)?;

    let length = buffer.offset;
    Ok(core::str::from_utf8(&bytes[..length]).expect("but we just formatted valid utf-8"))
}

pub struct PrintfFormatter<'a> {
    pub printf_string: &'a str,
    pub items: &'a [&'a dyn Display]
}

impl<'a> Display for PrintfFormatter<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut current_item = self.items.iter();
        let mut current_string = self.printf_string;
        while !current_string.is_empty() {
            let Some(n) = current_string.find("%") else {
                f.write_str(current_string)?;
                break;
            };
            let (before, after) = current_string.split_at(n);
            f.write_str(before)?;

            if after == "%" {
                f.write_str(after)?;
                break;
            }
            else if after.starts_with("%d") || after.starts_with("%f") || after.starts_with("%s") {
                if let Some(i) = current_item.next() {
                    Display::fmt(i, f)?;
                }
                else {
                    panic!("{} requires more arguments than expected", self.printf_string);
                }
            }
            else if after.starts_with("%%") {
                f.write_str("%")?;
            }
            else {
                panic!("{} contains an unknown formatter", self.printf_string);
            }
            current_string = &after[2..];
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct CStrPtr(pub *const c_char);
impl CStrPtr {
    pub const fn from_bytes(bytes: &[u8]) -> Self {
        let Some(&last) = bytes.last() else {
            panic!("CStrPtr::from_bytes with empty string")
        };
        if last != 0 {
            panic!("CStrPtr::from_bytes with non-null terminated string")
        }
        Self(bytes.as_ptr() as *const _)
    }
    pub unsafe fn expect_str(&self) -> &str {
        self.as_cstr()
            .to_str()
            .expect("got a non-UTF-8 string")
    }
    pub unsafe fn to_str_lossless(&self) -> Option<&str> {
        self.as_cstr()
            .to_str()
            .ok()
    }
    pub unsafe fn to_str_lossy(&self) -> Cow<str> {
        self.as_cstr()
            .to_string_lossy()
    }
    pub unsafe fn as_cstr(&self) -> &CStr {
        assert!(!self.0.is_null(), "NonNullCString with null pointer");
        CStr::from_ptr(self.0)
    }

    /// Returns an object that can be used to display this string.
    pub unsafe fn display_lossy<'a>(&'a self) -> LossyStringDisplayer<'a> {
        if self.0.is_null() {
            LossyStringDisplayer(b"<null>")
        }
        else {
            LossyStringDisplayer(self.as_cstr().to_bytes())
        }
    }

    /// Returns true if this is a null pointer.
    pub const fn is_null(&self) -> bool {
        self.0.is_null()
    }
}

/// Object that can be used to display a string lossy without performing any heap allocations.
#[repr(transparent)]
pub struct LossyStringDisplayer<'a>(pub &'a [u8]);
impl<'a> Display for LossyStringDisplayer<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        for c in self.0.utf8_chunks() {
            f.write_str(c.valid())?;
            if !c.invalid().is_empty() {
                f.write_char(char::REPLACEMENT_CHARACTER)?;
            }
        }
        Ok(())
    }
}

/// Compare two byte arrays using a case-insensitive ASCII comparison.
pub fn compare_ascii_case_insensitive(a: &[u8], b: &[u8]) -> Ordering {
    let mut lowercase_a = a.iter().map(|a| a.to_ascii_lowercase());
    let mut lowercase_b = b.iter().map(|b| b.to_ascii_lowercase());

    loop {
        let a = lowercase_a.next();
        let b = lowercase_b.next();

        let Some(a) = a else {
            if b.is_none() {
                return Ordering::Equal;
            }
            else {
                return Ordering::Less;
            }
        };

        let Some(b) = b else {
            return Ordering::Greater;
        };

        let c = a.cmp(&b);
        if c == Ordering::Equal {
            continue
        }
        return c
    }
}

/// Convert an 8-bit character from Windows to UTF-8.
pub fn decode_win32_character(character: u8) -> char {
    let chars = [character, 0];
    let mut wide = [0u16; 8];

    // SAFETY: We ensure the buffers are large enough.
    let result = unsafe {
        MultiByteToWideChar(CP_ACP, MB_PRECOMPOSED, chars.as_ptr(), 2, wide.as_mut_ptr(), wide.len() as i32)
    };

    assert!(result > 0, "Failed to parse character 0x{character:02X}; win32 decided that you are not allowed to have text");

    let mut decoder = char::decode_utf16(wide.into_iter().take(result as usize - 1));

    let Some(Ok(c)) = decoder.next() else {
        panic!("Failed to parse character 0x{character:02X}; win32 failed to decode it somehow...")
    };

    if decoder.next().is_some() {
        panic!("Failed to parse character 0x{character:02X}; win32 spawned multiple characters out of just one for no reason whatsoever...")
    }

    c
}
