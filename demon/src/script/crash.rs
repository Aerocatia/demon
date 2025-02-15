pub mod c {
    use c_mine::c_mine;
    use crate::util::CStrPtr;

    #[c_mine]
    pub unsafe extern "C" fn main_crash(param: CStrPtr) {
        let message = param.as_str();
        panic!("crash command called with message:\n\n{message}");
    }
}
