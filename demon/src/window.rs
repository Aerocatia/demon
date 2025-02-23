use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::ReleaseCapture;
use crate::util::VariableProvider;

pub mod c;

pub static mut UNFOCUSED_GAIN: f32 = 1.0;

static OLD_WINDOW_FOCUS_GAIN: AtomicU32 = AtomicU32::new(1.0f32.to_bits());
static WAS_FOCUSED: AtomicBool = AtomicBool::new(true);

const MASTER_GAIN: VariableProvider<f32> = variable! {
    name: "MASTER_GAIN",
    cache_address: 0x00F436E0,
    tag_address: 0x00FFACB0
};

pub unsafe fn on_window_focus_change(focus: bool) {
    if WAS_FOCUSED.swap(focus, Ordering::Relaxed) == focus {
        return
    }
    if focus {
        *MASTER_GAIN.get_mut() = f32::from_bits(OLD_WINDOW_FOCUS_GAIN.load(Ordering::Relaxed));
    }
    else {
        let old_gain = *MASTER_GAIN.get();
        OLD_WINDOW_FOCUS_GAIN.store(old_gain.to_bits(), Ordering::Relaxed);
        *MASTER_GAIN.get_mut() = old_gain * UNFOCUSED_GAIN.clamp(0.0, 1.0);
        ReleaseCapture();
    }
}
