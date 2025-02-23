use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use crate::script::{get_external_globals, get_functions};

pub fn complete(what: &str) -> Option<(String, Vec<String>)> {
    let what = what.trim_start();
    let words: Vec<&str> = what.split_whitespace().collect();
    let first_command = if words.is_empty() {
        ""
    }
    else {
        words[0]
    };

    if words.len() > 1 {
        // TODO: command suggestions
        return None
    }

    let mut suggestions = Vec::new();

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

    if suggestions.len() == 1 {
        Some((suggestions[0].clone(), suggestions))
    }
    else if suggestions.is_empty() {
        None
    }
    else {
        suggestions.sort();

        // Find the most common prefix of all of these suggestions
        let len = find_largest_shared_prefix_length(&suggestions);
        let suggestion = suggestions[0][..len].to_owned();
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
