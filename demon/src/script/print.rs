pub mod c {
    use c_mine::c_mine;
    use tag_structs::primitives::color::ColorRGB;
    use crate::util::CStrPtr;

    #[c_mine]
    pub unsafe extern "C" fn hs_print(message: CStrPtr) {
        console_color!(
            &const { ColorRGB { r: 0.0, g: 1.0, b: 0.0 }.as_colorargb() },
            "{}",
            message.display_lossy()
        );
    }
}
