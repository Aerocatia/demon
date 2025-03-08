use c_mine::c_mine;
use crate::init::{get_exe_type, ExeType};

#[c_mine]
pub extern "C" fn is_cache_build() -> bool {
    get_exe_type() == ExeType::Cache
}
