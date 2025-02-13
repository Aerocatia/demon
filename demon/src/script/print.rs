pub mod c {
    use c_mine::c_mine;
    use tag_structs::primitives::color::{ColorARGB, ColorRGB};
    use crate::console::show_debug_messages;
    use crate::util::CStrPtr;

    #[c_mine]
    pub unsafe extern "C" fn hs_print(message: CStrPtr) {
        if show_debug_messages() {
            console_color!(
                ColorARGB {
                    a: 1.0,
                    color: ColorRGB {
                        r: 0.0,
                        g: 1.0,
                        b: 0.0
                    }
                },
                "{}",
                message.display_lossy()
            );
        }
    }
}
