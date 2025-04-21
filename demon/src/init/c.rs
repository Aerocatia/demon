use c_mine::c_mine;
use crate::init::{get_command_line_argument_value, get_exe_type, has_command_line_argument_value, ExeType};
use crate::util::CStrPtr;

#[c_mine]
pub extern "C" fn is_cache_build() -> bool {
    get_exe_type() == ExeType::Cache
}

#[c_mine]
pub unsafe extern "C" fn get_command_line_arg(arg: CStrPtr, value: *mut CStrPtr) -> bool {
    let arg = arg.expect_str();
    if value.is_null() {
        has_command_line_argument_value(arg)
    }
    else if let Some(arg_value) = get_command_line_argument_value(arg) {
        *value = arg_value;
        true
    }
    else {
        false
    }
}
