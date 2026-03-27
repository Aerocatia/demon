#![no_std]
#![allow(unsafe_op_in_unsafe_fn)]

extern crate alloc;

mod util;
use alloc::{collections::btree_map::BTreeMap, ffi::CString, string::String};
use min32::dllmain;
use util::messagebox;
use windows_sys::Win32::System::{LibraryLoader::{GetProcAddress, LoadLibraryA}, Memory::{PAGE_EXECUTE_READWRITE, VirtualProtect}, SystemServices};

use core::{ffi::{CStr, c_void}, mem::transmute};

use crate::util::terminate;

#[cfg(not(target_os = "windows"))]
compile_error!("Must be Windows, ideally i686-pc-windows-gnu");

#[cfg(not(target_pointer_width = "32"))]
compile_error!("Must be for 32-bit only, ideally i686-pc-windows-gnu");

#[dllmain]
pub unsafe fn main(
    _hinstance: *const c_void,
    reason: u32,
    _reserved: *const c_void
) -> windows_sys::core::BOOL {
    match reason {
        SystemServices::DLL_PROCESS_ATTACH => attach(),
        SystemServices::DLL_PROCESS_DETACH => {
            // we don't care - our work here is done
        },
        _ => {}
    }
    true as _
}

#[expect(dangerous_implicit_autorefs)]
unsafe fn attach() {
    let start = 0x00401000 as *mut [u8; 5];

    if *start != [0xCC; 5] {
        messagebox(
            "No thunks",
            &alloc::format!("Failed to find the interrupt block at {start:#?}")
        );
        terminate();
    }

    let mut current_thunk = start.add(1);

    let mut thunks= BTreeMap::<*mut [u8; 5], usize>::new();
    let mut no_op_thunks= BTreeMap::<*mut [u8; 5], usize>::new();

    while (*current_thunk)[0] == 0xE9 {
        let offset = usize::from_le_bytes((*current_thunk)[1..].try_into().unwrap());
        let next = current_thunk.add(1); // need the next thunk because JMP is offset + current instruction pointer
        let to: usize = next as usize + offset;

        if *(to as *const [u8; 5]) == [0x55, 0x8B, 0xEC, 0x5D, 0xC3] { // push ebp; mov ebp, esp; pop ebp; ret;
            no_op_thunks.insert(current_thunk, to);
            thunks.insert(current_thunk, to);
        }
        else {
            thunks.insert(current_thunk, to);
        }

        current_thunk = next;
    }

    if thunks.is_empty() {
        messagebox(
            "No thunks",
            &alloc::format!("No thunks follow at {current_thunk:#?}")
        );
        terminate();
    }

    let thunk_data = core::slice::from_raw_parts(start as *const u8, (current_thunk as usize).wrapping_sub(start as usize));
    let thunk_checksum = *blake3::hash(thunk_data).as_bytes();

    let expected_checksum = [
        0xE2, 0xD6, 0x45, 0x65, 0xA8, 0xAF, 0x40, 0x7E,
        0xF3, 0xAD, 0x12, 0xD6, 0x1E, 0xA0, 0x71, 0x80,
        0xE7, 0xD6, 0x02, 0xDB, 0x6B, 0xA8, 0x90, 0x38,
        0x67, 0x61, 0xA3, 0x73, 0x8F, 0x00, 0x17, 0x40
    ];

    if expected_checksum != thunk_checksum {
        messagebox("Incorrect EXE", "This needs to be run on the cache build (thunk table checksum mismatch!)");
        terminate();
    }

    let demon = LoadLibraryA(c"demon.dll".as_ptr() as *const u8);
    if demon.is_null() {
        messagebox("Failed to load", "Missing demon.dll");
        terminate();
    }

    let Some(demon_replacements_json) = GetProcAddress(demon, c"demon_replacements_json".as_ptr() as *const u8) else {
        messagebox("Failed to load", "Missing demon_get_all_functions() in demon.dll");
        terminate();
    };

    let all_functions = CStr::from_ptr(transmute(demon_replacements_json));
    let Ok(all_functions) = all_functions.to_str() else {
        messagebox("Failed to load", "demon_get_all_functions() was not UTF-8, or it wasn't null terminated.");
        terminate();
    };

    let functions: BTreeMap<String, Replacement> = match serde_json::from_str(all_functions) {
        Ok(n) => n,
        Err(e) => {
            messagebox("Failed to load", &alloc::format!("demon_get_all_functions() returned invalid data: {e}"));
            terminate();
        }
    };

    for (function_name, replacement_data) in functions {
        if replacement_data.ignore.unwrap_or(false) {
            continue
        }

        if !replacement_data.address.starts_with("0x") {
            messagebox("Failed to load", &alloc::format!("Function {function_name}'s address must start with 0x"));
            terminate();
        }
        let Ok(address) = usize::from_str_radix(&replacement_data.address[2..], 16) else {
            messagebox("Failed to load", &alloc::format!("Function {function_name}'s address is not valid hex"));
            terminate();
        };

        let sudo = replacement_data.sudo.unwrap_or(false);
        let thunk_addr = address as *mut [u8; 5];
        let is_thunk = thunks.contains_key(&thunk_addr);

        if sudo && is_thunk {
            messagebox("Failed to load", &alloc::format!("Function {function_name} has sudo set to true, but it's a thunk."));
            terminate();
        }
        if !sudo && !is_thunk {
            messagebox("Failed to load", &alloc::format!("Function {function_name} has is not a valid thunk (you need sudo to bypass this - be careful!)."));
            terminate();
        }

        let function_name_cstr = CString::new(function_name.clone()).expect("no way this can fail");

        let address_of_function = match replacement_data.disabled {
            Some(true) => error_function as *const u8 as usize,
            _ => {
                let Some(proc_addr) = GetProcAddress(demon, function_name_cstr.as_ptr() as *const u8) else {
                    messagebox("Failed to load", &alloc::format!("Function {function_name} has does not exist in demon.dll."));
                    terminate();
                };
                proc_addr as usize
            }
        };

        let thunk = &mut *thunk_addr;
        let mut old_protect = 0;

        VirtualProtect(
            thunk_addr as *const c_void,
            size_of_val(thunk),
            PAGE_EXECUTE_READWRITE,
            &mut old_protect
        );

        let instruction_pointer_before_jump = thunk_addr.add(1) as usize;
        let to_address = address_of_function.wrapping_sub(instruction_pointer_before_jump);

        thunk[0] = 0xE9;
        thunk[1..5].copy_from_slice(&to_address.to_le_bytes());

        VirtualProtect(
            thunk_addr as *const c_void,
            size_of_val(thunk),
            old_protect,
            &mut 0
        );
    }

    if let Some(f) = GetProcAddress(demon, c"demon_count_thunks".as_ptr() as *const u8) {
        if *(f as *const u8) == 1 {
            messagebox("Success", &alloc::format!("Found {} thunks! ({} are no-ops in this build)", thunks.len(), no_op_thunks.len()));
        }
    }
}

#[derive(serde::Deserialize)]
struct Replacement {
    address: String,
    disabled: Option<bool>,
    ignore: Option<bool>,
    sudo: Option<bool>
}

#[unsafe(no_mangle)]
pub extern "C" fn error_function() {
    messagebox("Disabled function called", &alloc::format!("SCYTHER!! >w< *uses Cut on your process*"));
    terminate();
}
