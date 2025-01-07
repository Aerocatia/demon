fn main() {
    embed_resource::compile("strings/custom_edition/strings.rc", embed_resource::NONE).manifest_optional().unwrap();
}