use c_mine::pointer_from_hook;
use crate::util::PointerProvider;

pub unsafe fn close_all_ui_widgets() {
    const UI_WIDGETS_CLOSE_ALL: PointerProvider<unsafe extern "C" fn()> = pointer_from_hook!("ui_widgets_close_all");
    UI_WIDGETS_CLOSE_ALL.get()();
}
