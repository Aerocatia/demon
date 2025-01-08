fn main() {
    // We are only interested in building this for i686-pc-windows-* targets. Don't try to run
    // windres for anything else.
    let Ok(target) = std::env::var("CARGO_BUILD_TARGET") else {
        return
    };
    if target.starts_with("i686-pc-windows-") {
        embed_resource::compile("strings/custom_edition/strings.rc", embed_resource::NONE)
            .manifest_optional()
            .expect("Failed to run windres; this is needed to embed the strings.dll contents!");
    }
}
