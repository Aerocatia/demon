#![no_std]
#![allow(unused_variables)]
#![allow(dead_code)]

// Allows calls to unsafe functions inside other unsafe functions without needing an unsafe block.
//
// This warning was turned on in Rust 2024. It makes sense in most cases (like preventing accidental
// footgun usage), but this crate has to interface with Halo x86 code directly, and it has to share
// data with it in order to work.
//
// This is giga-unsafe! Calling Halo code without causing a slow and painful death of the process
// (or an instant but equally painful death) requires great care. Additionally, data can be
// read/written to at any point (the game does make light use of threads), and we can't track this,
// yet.
//
// The sheer amount of unsafe code in this crate would necessitate having tons of unsafe {} blocks
// OR surrounding each function in an unsafe block. The former makes the codebase unreadable (which
// would make writing safe code around it more painful), and the latter negates the entire purpose
// of this lint.
//
// Until we are able to make this codebase more safe (which will take a very long time), the lint
// stays off.
#![allow(unsafe_op_in_unsafe_fn)]

#[cfg(not(all(target_pointer_width = "32", windows)))]
compile_error!("This crate can only be compiled for i686-pc-windows-* targets!");

extern crate min32;
extern crate alloc;

use tag_structs::primitives::color::{ColorARGB, ColorRGB};

const CONSOLE_COLOR_ERROR: ColorARGB = ColorRGB { r: 1.0, g: 0.0, b: 0.0 }.as_colorargb();
const CONSOLE_COLOR_WARNING: ColorARGB = ColorRGB { r: 1.0, g: 0.8, b: 0.0 }.as_colorargb();

/// Print the formatted string to the in-game console.
#[allow(unused_macros)]
macro_rules! console {
    ($($args:tt)*) => {{
        crate::console::console_put_args(None, format_args!($($args)*));
    }};
}

/// Print the formatted string to the in-game HUD.
#[allow(unused_macros)]
macro_rules! hud {
    ($($args:tt)*) => {{
        crate::rasterizer::hud::hud_put_args(format_args!($($args)*));
    }};
}

/// Print the formatted string to the in-game console with a given color.
///
/// The first argument must be [`ColorARGB`] or [`&ColorARGB`].
#[allow(unused_macros)]
macro_rules! console_color {
    ($color:expr, $($args:tt)*) => {{
        let color: &tag_structs::primitives::color::ColorARGB = tag_structs::primitives::color::ColorARGB::as_ref(&$color);
        crate::console::console_put_args(Some(color), format_args!($($args)*));
    }};
}

/// Print an error to the console with the given formatting.
#[allow(unused_macros)]
macro_rules! error {
    ($($args:tt)*) => {{
        crate::console::console_put_args(Some(&crate::CONSOLE_COLOR_ERROR), format_args!($($args)*));
    }};
}

/// Print a warning to the console with the given formatting.
#[allow(unused_macros)]
macro_rules! warn {
    ($($args:tt)*) => {{
        crate::console::console_put_args(Some(&crate::CONSOLE_COLOR_WARNING), format_args!($($args)*));
    }};
}

/// Log a message to debug.txt.
#[allow(unused_macros)]
macro_rules! debug_log {
    ($($args:tt)*) => {{
        crate::error_log::error_put_args(crate::error_log::ErrorPriority::FileOnly, format_args!($($args)*));
    }};
}

#[macro_use]
mod util;
mod error_log;
mod console;
mod input;

mod panic;
mod init;
mod tag;
mod id;
mod memory;
mod timing;
mod math;
mod script;
mod string;
mod sound;
mod crc32;
mod player;
mod object;
mod bink;
mod multiplayer;
mod ui;
mod rasterizer;
mod globals;
mod game_engine;
mod random;
mod scrollback;
mod model;
mod collision;
mod window;
mod map;
mod bitmap;
mod file;
mod interface;
