use core::sync::atomic::{AtomicBool, Ordering};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetKeyState, VK_NEXT, VK_PRIOR};

pub struct ScrollbackButton {
    page_up_state: AtomicBool,
    page_down_state: AtomicBool
}
impl ScrollbackButton {
    pub const fn new() -> Self {
        Self {
            page_down_state: AtomicBool::new(false),
            page_up_state: AtomicBool::new(false)
        }
    }

    /// Returns the number of pages to scroll.
    ///
    /// If the value is positive, it is the number of pages to scroll up.
    ///
    /// If the value is negative, it is the number of pages to scroll down.
    pub fn poll(&self) -> i32 {
        // SAFETY: These are valid keys; should be fine
        let page_up_pushed = unsafe { GetKeyState(VK_PRIOR as i32) } & 0x80 != 0;
        let page_down_pushed = unsafe { GetKeyState(VK_NEXT as i32) } & 0x80 != 0;

        let mut scrollback_delta = 0;
        if !self.page_up_state.swap(page_up_pushed, Ordering::Relaxed) && page_up_pushed {
            scrollback_delta += 1;
        }
        if !self.page_down_state.swap(page_down_pushed, Ordering::Relaxed) && page_down_pushed {
            scrollback_delta -= 1;
        }
        scrollback_delta
    }
}
