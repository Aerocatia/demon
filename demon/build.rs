fn main() {
    // We are only interested in building this for i686-pc-windows-* targets. Don't try to run
    // windres for anything else.
    let Ok(arch) = std::env::var("CARGO_CFG_TARGET_ARCH") else {
        println!("cargo:warning=Can't detect target arch; windres won't be called");
        return
    };
    let Ok(os) = std::env::var("CARGO_CFG_TARGET_OS") else {
        println!("cargo:warning=Can't detect target OS; windres won't be called");
        return
    };
    if arch == "x86" && os == "windows" {
        embed_resource::compile("../strings/custom_edition/strings.rc", embed_resource::NONE)
            .manifest_optional()
            .expect("Failed to run windres; this is needed to embed the strings.dll contents!");
    }
    else {
        println!("cargo:warning=Not building for i686-pc-windows-* (target_arch={arch}, target_os={os}); windres won't be called");
    }
}
