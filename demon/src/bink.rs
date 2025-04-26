use c_mine::pointer_from_hook;
use crate::util::{CStrPtr, PointerProvider, StaticStringBytes};

pub mod hsc;
pub mod c;

pub const PLAY_BINK: PointerProvider<unsafe extern "C" fn(CStrPtr)> = pointer_from_hook!("play_bink");
pub unsafe fn play_bink(bink: &str) {
    PLAY_BINK.get()(CStrPtr::from_bytes(StaticStringBytes::<512>::from_str(bink).as_bytes_with_null()));
}
