use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::console::printf;
use crate::file::Win32DirectoryIterator;
use crate::init::{get_exe_type, ExeType};
use crate::script::{get_external_globals, get_functions};
use crate::util::CStrPtr;

pub fn complete(what: &str) -> Option<(String, Vec<String>)> {
    let what = what.trim_start();
    let words: Vec<&str> = what.split_whitespace().collect();
    let first_command = if words.is_empty() {
        ""
    }
    else {
        words[0]
    };

    let mut suggestions = Vec::new();
    if words.len() > 1 || words.len() == 1 && what.ends_with( " ") {
        // TODO: command suggestions
        if first_command.eq_ignore_ascii_case("map_name") {
            suggestions = complete_map_name(words.as_slice())?;
        }
        else {
            return None
        }
    }
    else {
        // SAFETY: These are immutable and compiled into this executable. This is always safe.
        unsafe {
            for i in get_functions() {
                let command_name = i.name.expect_str();
                if starts_with_ignoring_case(command_name, first_command) {
                    suggestions.push(format!("{} ", command_name));
                }
            }
            for i in get_external_globals() {
                let global_name = i.name();
                if starts_with_ignoring_case(global_name, first_command) {
                    suggestions.push(format!("{} ", global_name))
                }
            }
        }
    }

    if suggestions.is_empty() {
        None
    }
    else {
        suggestions.sort();

        // Find the most common prefix of all of these suggestions
        let len = find_largest_shared_prefix_length(&suggestions);
        let suggestion = suggestions[0][..len].to_owned();

        // Only show the last word in suggestion
        // FIXME: Handle quoted stuff
        for i in &mut suggestions {
            *i = i.split_whitespace().rev().next().unwrap().to_string();
        }

        Some((suggestion, suggestions))
    }
}

fn find_largest_shared_prefix_length(what: &[String]) -> usize {
    let first = what[0].as_bytes();

    for (len, &byte) in first.iter().enumerate() {
        for i in &what[1..] {
            let Some(&q) = i.as_bytes().get(len) else {
                return len
            };
            if q != byte {
                return len
            }
        }
    }

    first.len()
}

fn starts_with_ignoring_case(string: &str, with: &str) -> bool {
    let mut string_iter = string.chars();
    let mut with_iter = with.chars();

    while let Some(n) = with_iter.next() {
        let Some(s) = string_iter.next() else {
            return false
        };
        if !s.eq_ignore_ascii_case(&n) {
            return false
        }
    }

    true
}

fn complete_map_name(words: &[&str]) -> Option<Vec<String>> {
    if words.len() > 2 {
        return None
    }

    let mut suggestions = Vec::new();
    let cmd = words[0].to_ascii_lowercase();
    let map = words.get(1).unwrap_or(&"");

    match get_exe_type() {
        ExeType::Cache => {
            for i in Win32DirectoryIterator::new("maps")? {
                if i.extension() != Some("map") {
                    continue
                }
                let basename = i.basename();
                if basename.eq_ignore_ascii_case("bitmaps") || basename.eq_ignore_ascii_case("sounds") || basename.eq_ignore_ascii_case("loc") {
                    continue
                }

                // FIXME: handle spaces better
                if basename.contains(" ") {
                    continue;
                }

                if starts_with_ignoring_case(basename, map) {
                    suggestions.push(format!("{cmd} {basename} "));
                    unsafe { printf(CStrPtr::from_cstr(c"%s\n"), format!("{}\x00", i.basename()).as_ptr()); }
                }
            }
        },
        ExeType::Tag => {
            let scenario_test = format!("tags\\{map}");

            fn iteratafy(needle: &str, dir: &str, results: &mut Vec<String>, recursion: usize, cmd: &str) {
                unsafe { printf(CStrPtr::from_cstr(c"%s\n"), format!("{dir}\x00").as_ptr()) };
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
                        // FIXME: handle spaces better
                        results.push(format!("{cmd} {path} "));
                    }
                    iteratafy(needle, i.as_str(), results, recursion + 1, cmd);
                }
            }

            iteratafy(&scenario_test, "tags", &mut suggestions, 0, &cmd);
        }
    }

    if suggestions.is_empty() {
        return None
    }

    Some(suggestions)
}
