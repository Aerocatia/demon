fn main() {
    for i in std::fs::read_dir("hook").unwrap().filter_map(|d| d.ok()) {
        let path = i.path();
        if path.extension() != Some("json".as_ref()) {
            continue
        }
        println!("cargo:rerun-if-changed={}", path.display());
    }
}
