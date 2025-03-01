pub mod c {
    use alloc::string::String;
    use c_mine::c_mine;
    use crate::script::{get_external_globals, get_functions};
    use crate::util::write_to_file;

    #[c_mine]
    pub unsafe extern "C" fn script_doc() {
        let mut args = String::with_capacity(65536);

        core::fmt::write(&mut args, format_args!("[Functions]\n\n")).expect(";-;");

        let mut functions = get_functions().to_vec();
        functions.sort_by(|a, b| a.name.expect_str().cmp(b.name.expect_str()));

        for i in functions {
            let name = i.name.expect_str();
            let description = i.description.get_str().unwrap_or("");
            let usage = i.usage.get_str().unwrap_or("");

            core::fmt::write(&mut args, format_args!("(<{}> {name}", i.return_type)).expect(";-;");

            if usage.is_empty() {
                for i in &i.argument_types[..i.argument_count as usize] {
                    core::fmt::write(&mut args, format_args!(" <{i}>")).expect(";-;");
                }
            }
            else {
                core::fmt::write(&mut args, format_args!(" {usage}")).expect(";-;");
            }

            core::fmt::write(&mut args, format_args!(")\n")).expect(";-;");

            if description.is_empty() {
                core::fmt::write(&mut args, format_args!("<no description>\n")).expect(";-;");
            }
            else {
                core::fmt::write(&mut args, format_args!("{}\n", i.description.expect_str())).expect(";-;");
            }

            core::fmt::write(&mut args, format_args!("\n")).expect(";-;");
        }

        core::fmt::write(&mut args, format_args!("\n[Globals]\n\n")).expect(";-;");

        let mut globals = get_external_globals().to_vec();
        globals.sort_by(|a, b| a.name().cmp(b.name()));

        for i in globals {
            let name = i.name();
            core::fmt::write(&mut args, format_args!("<{}> {name}\n", i.definition.global_type)).expect(";-;");
        }

        match write_to_file("hs_doc.txt", args.as_bytes()) {
            Ok(_) => console!("Dumped all functions and globals to hs_doc.txt"),
            Err(_) => error!("Failed to write to hs_doc.txt")
        };
    }
}
