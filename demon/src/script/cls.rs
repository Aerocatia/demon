pub mod c {
    use c_mine::c_mine;
    use crate::console::CONSOLE_BUFFER;

    #[c_mine]
    pub extern "C" fn main_cls() {
        CONSOLE_BUFFER.write().clear_messages();
    }
}
