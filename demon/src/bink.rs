pub mod hsc;

use c_mine::c_mine;

#[c_mine]
pub extern "C" fn bink_playback_active() -> bool {
    false
}
