#![no_std]
#![allow(unused_variables)]
#![allow(dead_code)]

#[cfg(not(all(target_pointer_width = "32", windows)))]
compile_error!("This crate can only be compiled for i686-pc-windows-* targets!");

extern crate alloc;

#[macro_use]
mod util;
#[macro_use]
mod console;

mod allocator;
mod panic;
mod init;
mod tag;
mod id;
mod memory;
mod timing;
mod mouse;
mod scoreboard;
mod math;
mod script;
mod string;
mod sound;
