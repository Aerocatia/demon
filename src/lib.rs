#![no_std]

#[cfg(not(all(target_pointer_width = "32", windows)))]
compile_error!("This crate can only be compiled for i686-pc-windows-* targets!");

extern crate alloc;

mod allocator;
mod util;
mod panic;
mod init;