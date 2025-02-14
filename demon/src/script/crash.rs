pub mod c {
    use c_mine::c_mine;
    use crate::script::get_external_globals;
    use crate::util::CStrPtr;

    #[c_mine]
    pub unsafe extern "C" fn main_crash(param: CStrPtr) {
        for i in get_external_globals().iter().take(1) {
            console!("{}", i.name());
        }
        let message = param.as_str();
        panic!("crash command called with message:\n\n{message}");
    }
}
