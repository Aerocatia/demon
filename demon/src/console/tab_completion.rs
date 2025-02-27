use alloc::borrow::{Cow, ToOwned};
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::iter::FusedIterator;
use crate::console::printf;
use crate::file::Win32DirectoryIterator;
use crate::init::{get_exe_type, ExeType};
use crate::script::{get_external_globals, get_functions};
use crate::util::CStrPtr;

type Argument<'a> = Cow<'a, str>;

pub fn complete(what: &str) -> Option<(String, Vec<String>)> {
    let what = what.trim_start();

    let words: Vec<Argument> = CommandParser::new(what).collect();
    let first_command = match words.first() {
        Some(a) => a.as_ref(),
        None => ""
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
        return None
    }

    suggestions.sort();

    // Find the most common prefix of all of these suggestions
    let len = find_largest_shared_prefix_length(&suggestions);
    let suggestion = suggestions[0][..len].to_owned();

    // Only show the last word in suggestion
    let mut incomplete_last_argument_may_have_spaces = false;
    for i in &mut suggestions {
        *i = CommandParser::new(i.as_str()).last().expect("can't parse nothing").into_owned();
        incomplete_last_argument_may_have_spaces = incomplete_last_argument_may_have_spaces || i.chars().any(|c| c.is_whitespace())
    }

    incomplete_last_argument_may_have_spaces = incomplete_last_argument_may_have_spaces && suggestions.len() > 1;

    let mut new_suggestion = add_quotes_as_needed(CommandParser::new(suggestion.as_str()), incomplete_last_argument_may_have_spaces);
    if suggestions.len() == 1 && suggestion.ends_with(" ") {
        new_suggestion += " ";
    }

    Some((new_suggestion, suggestions))
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

fn complete_map_name(words: &[Argument]) -> Option<Vec<String>> {
    if words.len() > 2 {
        return None
    }

    let mut suggestions = Vec::new();
    let cmd = words[0].to_ascii_lowercase();
    let map = words.get(1).map(|a| a.as_ref()).unwrap_or("");

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

                if starts_with_ignoring_case(basename, map) {
                    suggestions.push(format!("{cmd} \"{basename}\" "));
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
                        results.push(format!("{cmd} \"{path}\" "));
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

struct CommandParser<'a> {
    string: &'a str
}

impl<'a> CommandParser<'a> {
    pub fn new(string: &'a str) -> CommandParser<'a> {
        CommandParser {
            string
        }
    }
}

impl<'a> Iterator for CommandParser<'a> {
    type Item = Cow<'a, str>;
    fn next(&mut self) -> Option<Self::Item> {
        self.string = self.string.trim_start();
        if self.string.is_empty() {
            return None
        }

        let mut argument = None;
        let mut in_quotes = false;
        for (index, character) in self.string.char_indices() {
            if !in_quotes && character.is_whitespace() {
                let (a, b) = self.string.split_at(index);
                self.string = b;
                argument = Some(a);
                break;
            }

            if character == '\"' {
                in_quotes = !in_quotes;
            }
        }

        let argument = argument.unwrap_or_else(|| core::mem::take(&mut self.string));

        if argument.contains('\"') {
            Some(Cow::Owned(argument.replace('\"', "")))
        }
        else {
            Some(Cow::Borrowed(argument))
        }
    }
}

impl<'a> FusedIterator for CommandParser<'a> {}

fn add_quotes_as_needed<T: AsRef<str>>(i: impl Iterator<Item = T>, incomplete_last_argument_may_have_spaces: bool) -> String {
    let mut r = String::new();
    let mut peek = i.peekable();

    loop {
        let Some(n) = peek.next() else {
            return r;
        };

        let mut already_added_quotes = false;
        if peek.peek().is_none() && incomplete_last_argument_may_have_spaces {
            r += "\"";
            already_added_quotes = true;
        }

        let string = n.as_ref();
        if !already_added_quotes && string.chars().any(|c| c.is_whitespace()) {
            r += "\"";
            r += string;
            r += "\"";
        } else {
            r += string;
        }

        if peek.peek().is_some() {
            r += " ";
        }
    }
}
