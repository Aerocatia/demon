#![no_std]
#![allow(unused_variables)]
#![allow(dead_code)]

#[cfg(not(all(target_pointer_width = "32", windows)))]
compile_error!("This crate can only be compiled for i686-pc-windows-* targets!");

extern crate alloc;

#[macro_use]
mod util;
#[macro_use]
mod error_log;
#[macro_use]
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
