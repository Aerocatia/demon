#![no_std]
#![allow(unsafe_op_in_unsafe_fn)]

extern crate alloc;

mod util;
use alloc::{collections::btree_map::BTreeMap, ffi::CString, string::String};
use min32::dllmain;
use util::messagebox;
use windows_sys::Win32::System::{LibraryLoader::{GetProcAddress, LoadLibraryA}, Memory::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE, VirtualAlloc, VirtualProtect}, SystemServices};

use core::{ffi::{CStr, c_char, c_void}, mem::transmute};

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
    let target_dll = LoadLibraryA(c"demon.dll".as_ptr() as *const u8);
    if target_dll.is_null() {
        messagebox("Failed to load", "Missing the demon (demon.dll)");
        terminate();
    }

    let Some(expected_checksum_addr) = GetProcAddress(target_dll, c"demon_thunk_checksum".as_ptr() as *const u8) else {
        messagebox("Failed to load", "Missing demon_thunk_checksum in the demon");
        terminate();
    };

    let expected_checksum = *(expected_checksum_addr as *const [u8; 0x20]);

    let Some(thunk_address) = GetProcAddress(target_dll, c"demon_thunk_address".as_ptr() as *const u8) else {
        messagebox("Failed to load", "Missing demon_thunk_address in the demon");
        terminate();
    };

    let start = *(thunk_address as *mut *mut [u8; 5]);

    if *start != [0xCC; 5] {
        messagebox(
            "No thunks",
            &alloc::format!("Mismatched DLL! Failed to find the interrupt block at {start:#?}")
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
            &alloc::format!("No thunks follow at {current_thunk:#?} (mismatched DLL?)")
        );
        terminate();
    }

    let thunk_data = core::slice::from_raw_parts(start as *const u8, (current_thunk as usize).wrapping_sub(start as usize));
    let thunk_checksum = *blake3::hash(thunk_data).as_bytes();

    if expected_checksum != thunk_checksum {
        let expected = GetProcAddress(target_dll, c"demon_target_exe_name".as_ptr() as *const u8)
            .map(|i| CStr::from_ptr(i as *const c_char).to_str().expect("demon_target_exe_name UTF-8"))
            .unwrap_or("???");
        messagebox("Incorrect EXE", &alloc::format!("Mismatched DLL! This DLL targets {expected} (thunk table checksum mismatch!)"));
        terminate();
    }

    let Some(demon_replacements_json) = GetProcAddress(target_dll, c"demon_replacements_json".as_ptr() as *const u8) else {
        messagebox("Failed to load", "Missing demon_replacements_json in the demon");
        terminate();
    };

    let all_functions = CStr::from_ptr(transmute(demon_replacements_json as *const c_char));
    let Ok(all_functions) = all_functions.to_str() else {
        messagebox("Failed to load", "demon_get_all_functions() was not UTF-8, or it wasn't null terminated.");
        terminate();
    };

    // We want to allocate as far away from 0x40440000 as possible to avoid conflicts.
    //
    // This will hold nopped function names which we'll pass into error_function.
    let jmp_data_len = 256 * 1024;
    let jmp_data = VirtualAlloc(0xCACA0000 as *mut _, jmp_data_len, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
    if jmp_data.is_null() {
        messagebox("Failed to load", "Can't allocate 256 KiB @ 0xCACA0000.\n\nIs your EXE not Large Address Aware? Or maybe stuff is already there...");
        terminate();
    }
    let jmp_data = core::slice::from_raw_parts_mut(jmp_data as *mut u8, jmp_data_len);
    let mut remaining_jmp_data = jmp_data;

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

        let address_of_function = match replacement_data.disable {
            Some(DisableType::Forbid) => {
                let Some((name, remaining)) = remaining_jmp_data.split_at_mut_checked(function_name_cstr.count_bytes() + 1) else {
                    messagebox("Failed to load", &alloc::format!("Ran out of jmp buffer space to write the function name."));
                    terminate();
                };

                name.copy_from_slice(function_name_cstr.as_bytes_with_nul());

                // push NAME ; 5 bytes
                // call error_function ; 5 bytes
                // int3 ; 1 byte
                // = 11 bytes

                let Some((asm_code, remaining)) = remaining.split_at_mut_checked(11) else {
                    messagebox("Failed to load", &alloc::format!("Ran out of jmp buffer space to write the error function handler."));
                    terminate();
                };

                let function_name_ptr = name.as_ptr();

                asm_code[0x0] = 0x68; // push...
                asm_code[0x1..0x5].copy_from_slice(&(function_name_ptr as usize).to_le_bytes());

                asm_code[0x5] = 0xE8; // call...
                let offset = asm_code[0xA..].as_ptr() as usize;
                asm_code[0x6..0xA].copy_from_slice(&((error_function as *const () as usize).wrapping_sub(offset)).to_le_bytes());

                asm_code[0xA] = 0xCC;

                // Rest of data goes here then
                remaining_jmp_data = remaining;
                
                asm_code.as_ptr() as *const () as usize
            },
            Some(DisableType::Nop) => {
                nop_function as *const () as usize
            },
            None => {
                let Some(proc_addr) = GetProcAddress(target_dll, function_name_cstr.as_ptr() as *const u8) else {
                    messagebox("Failed to load", &alloc::format!("Function {function_name} has does not exist in the demon."));
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

    if let Some(f) = GetProcAddress(target_dll, c"demon_count_thunks".as_ptr() as *const u8) {
        if *(f as *const u8) == 1 {
            messagebox("Success", &alloc::format!("Found {} thunks! ({} are no-ops in this build)", thunks.len(), no_op_thunks.len()));
        }
    }
}

#[derive(serde::Deserialize)]
struct Replacement {
    address: String,
    disable: Option<DisableType>,
    ignore: Option<bool>,
    sudo: Option<bool>
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum DisableType {
    Forbid,
    Nop
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn error_function(function: *const c_char) {
    let f = if function.is_null() {
        "??? (null function passed lol)"
    }
    else {
        CStr::from_ptr(function).to_str().unwrap_or("non-utf8 function name")
    };

    messagebox("Forbidden function called", &alloc::format!("SCYTHER!! >w< *uses Cut on your process*\n\nTried to call '{f}' which is marked as forbidden.\n\nThis function can't be used. It's recommended that this function is forgotten. Once forgotten, this function can't be called."));
    terminate();
}

#[unsafe(no_mangle)]
pub extern "C" fn nop_function() -> usize { 0 }
