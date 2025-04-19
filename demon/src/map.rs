pub mod hsc;

use crate::init::{get_exe_type, ExeType};
use crate::util::{starts_with_ignoring_case, VariableProvider};
use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use minxp::ffi::OsStr;
use minxp::fs::read_dir;
use minxp::path::Path;
use tag_structs::CacheFileHeader;

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
            let Ok(iterator) = read_dir("maps") else {
                return Vec::new()
            };

            for i in iterator {
                let Ok(i) = i else { continue };
                let path = i.path();

                let Some((basename, extension)) = path.file_stem().and_then(|s| Some((s.to_str()?, path.extension().and_then(OsStr::to_str)?))) else {
                    continue
                };

                if extension != "map" {
                    continue
                }

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

            fn iteratafy(needle: &str, dir: &Path, results: &mut Vec<String>, recursion: usize) {
                if recursion == 100 {
                    return
                }
                let Ok(iterator) = read_dir(dir) else { return };
                for i in iterator {
                    let Ok(i) = i else { continue };
                    let path = i.path();

                    let Some((basename, extension)) = path.file_stem().and_then(|s| Some((s.to_str()?, path.extension().and_then(OsStr::to_str)))) else {
                        continue
                    };

                    let path_str = path.to_str().unwrap();
                    if !path_str.starts_with(needle) && !needle.starts_with(path_str) {
                        continue
                    }

                    if extension == Some("scenario") {
                        let path = &path_str[5..path_str.rfind(".").unwrap()];
                        results.push(path.to_owned());
                        continue
                    }

                    iteratafy(needle, path.as_ref(), results, recursion + 1);
                }
            }

            iteratafy(&scenario_test, "tags".as_ref(), &mut suggestions, 0);
        }
    }

    suggestions
}

pub fn verify_map_header(name: &str, header: &CacheFileHeader) -> Result<(), &'static str> {
    (header.head_fourcc == 0x68656164).then_some(()).ok_or("map head fourcc is incorrect")?;
    (header.foot_fourcc == 0x666F6F74).then_some(()).ok_or("map foot fourcc is incorrect")?;
    header.map_type.try_get().map_err(|_| "invalid map type")?;
    (header.cache_version == 609).then_some(()).ok_or("incorrect cache version")?;
    (header.name.to_str() == name).then_some(()).ok_or("incorrect cache file name")?;

    Ok(())
}
