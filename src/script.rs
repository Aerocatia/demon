use core::ffi::{c_char, CStr};
use c_mine::c_mine;

#[c_mine]
pub unsafe extern "C" fn main_crash(param: *const c_char) {
    let message = CStr::from_ptr(param).to_string_lossy();
    panic!("crash command called with message:\n\n{}", message.as_ref());
}
