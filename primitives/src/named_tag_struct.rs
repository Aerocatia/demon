pub trait NamedTagStruct: Sized {
    fn name() -> &'static str;
}
