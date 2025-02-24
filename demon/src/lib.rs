#![no_std]
#![allow(unused_variables)]
#![allow(dead_code)]

#[cfg(not(all(target_pointer_width = "32", windows)))]
compile_error!("This crate can only be compiled for i686-pc-windows-* targets!");

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

mod allocator;
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
