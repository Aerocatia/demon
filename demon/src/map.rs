pub mod hsc;

use crate::file::Win32DirectoryIterator;
use crate::init::{get_exe_type, ExeType};
use crate::util::{starts_with_ignoring_case, VariableProvider};
use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

const SHOULD_LOAD_MAIN_MENU: VariableProvider<u8> = variable! {
    name: "should_load_main_menu",
    cache_address: 0x00C99727,
    tag_address: 0x00D50CFF
};

/// Load the main menu, instantly ending the current game on the next main_loop iteration.
pub unsafe fn load_main_menu() {
    *SHOULD_LOAD_MAIN_MENU.get_mut() = 1;
}

pub fn find_maps_with_prefix(prefix: &str) -> Vec<String> {
    let mut suggestions = Vec::new();

    match get_exe_type() {
        ExeType::Cache => {
            let Some(iterator) = Win32DirectoryIterator::new("maps") else {
                return Vec::new()
            };
            for i in iterator {
                if i.extension() != Some("map") {
                    continue
                }
                let basename = i.basename();
                if basename.eq_ignore_ascii_case("bitmaps") || basename.eq_ignore_ascii_case("sounds") || basename.eq_ignore_ascii_case("loc") {
                    continue
                }

                if starts_with_ignoring_case(basename, prefix) {
                    suggestions.push(basename.to_owned());
                }
            }
        },
        ExeType::Tag => {
            let scenario_test = format!("tags\\{prefix}");

            fn iteratafy(needle: &str, dir: &str, results: &mut Vec<String>, recursion: usize) {
                if recursion == 100 {
                    return
                }
                let Some(iterator) = Win32DirectoryIterator::new(dir) else {
                    return
                };
                for i in iterator {
                    if !i.as_str().starts_with(needle) && !needle.starts_with(i.as_str()) {
                        continue
                    }
                    if i.extension() == Some("scenario") {
                        let path_str = i.as_str();
                        let path = &path_str[5..path_str.rfind(".").unwrap()];
                        results.push(path.to_owned());
                    }
                    iteratafy(needle, i.as_str(), results, recursion + 1);
                }
            }

            iteratafy(&scenario_test, "tags", &mut suggestions, 0);
        }
    }

    suggestions
}
