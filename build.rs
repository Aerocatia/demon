fn main() {
    embed_resource::compile("strings/strings.rc", embed_resource::NONE).manifest_optional().unwrap();
}