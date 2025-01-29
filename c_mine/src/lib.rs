extern crate proc_macro;

use proc_macro::TokenStream;
use std::collections::{HashMap, HashSet};
use std::fmt;
use syn::__private::{quote, ToTokens};
use serde::Deserialize;

/// Wraps the function into a CFunctionProvider
#[proc_macro_attribute]
pub fn c_mine(_: TokenStream, token_stream: TokenStream) -> TokenStream {
    let parsed: syn::ItemFn = syn::parse(token_stream.clone())
        .expect("Cannot parse!");

    let ident = parsed.sig.ident.clone();
    let fn_name = ident.to_string();
    let ident = ident.to_token_stream();

    match (|| -> Result<(), &'static str> {
        let extern_c = match parsed.sig.abi.as_ref().and_then(|a| a.name.as_ref()) {
            Some(n) => n.value() == "C",
            _ => false
        };

        if !extern_c {
            return Err("is not extern \\\"C\\\"");
        }

        Ok(())
    })() {
        Ok(_) => (),
        Err(error) => {
            return format!(
                "{sig} {{ compile_error!(\"Function \\\"{fn_name}\\\" {error}\") }}",
                sig=parsed.sig.clone().to_token_stream()
            ).parse().expect(":(");
        }
    }

    let visibility = parsed.vis.to_token_stream();
    let fn_name_literal: proc_macro2::TokenStream = format!("\"{fn_name}\"")
        .parse()
        .expect("idk how it didn't parse");

    let fn_type = parsed.sig.clone();
    let args = fn_type.inputs.to_token_stream();
    let output = fn_type.output.to_token_stream();

    let asdf = quote::quote! {
        #[allow(non_upper_case_globals)]
        #visibility const #ident: crate::util::CFunctionProvider<unsafe extern "C" fn(#args) #output> = crate::util::CFunctionProvider {
            name: #fn_name_literal,
            function_getter: || {
                #parsed
                #ident
            },
            address_getter: |a| {
                a as *const _
            }
        };
    };

    asdf.into()
}

#[derive(Deserialize)]
struct Hook {
    pub tag: Option<String>,
    pub cache: Option<String>,
    pub replacement: String,
    pub sudo: Option<bool>,

    pub tags: Option<String>
}

#[proc_macro]
pub fn generate_hook_setup_code(_: TokenStream) -> TokenStream {
    let mut tag_code = String::with_capacity(65536);
    let mut cache_code = String::with_capacity(65536);
    let mut forbidden_code = String::with_capacity(65536);

    let mut added = HashSet::new();
    let mut added_tag = HashSet::new();
    let mut added_cache = HashSet::new();

    for i in std::fs::read_dir("c_mine/hook").expect("WHERE?").filter_map(|d| d.ok()) {
        let path = i.path();
        if path.extension() != Some("json".as_ref()) {
            continue
        }
        let data = std::fs::read(path).expect("failed to read hook JSON");
        let parsed: HashMap<String, Hook> = serde_json::from_slice(data.as_slice()).expect("failed to parse JSON");

        for (name, hook) in parsed {
            let mut target = hook
                .replacement
                .as_str();

            if target == "forbid" {
                target = &name;
                fmt::write(&mut forbidden_code, format_args!("#[c_mine] extern \"C\" fn {name}() {{ panic!(\"Entered stubbed-out function `{name}`\") }}")).expect(";-;");
            }

            if target == "nop" {
                target = &name;
                fmt::write(&mut forbidden_code, format_args!("#[c_mine] extern \"C\" fn {name}() {{ }}")).expect(";-;");
            }

            let write_fn = if hook.sudo == Some(true) {
                "write_jmp"
            }
            else {
                "overwrite_thunk"
            };

            if hook.tags.is_some() {
                panic!("{name} used tags! BUTTERFREE fainted!");
            }

            if let Some(tag) = hook.tag {
                if added_tag.contains(&tag) {
                    fmt::write(&mut forbidden_code, format_args!("{{ compile_error!(\"Duplicate tag addr {tag}\") }}")).expect(";-;");
                    break;
                }
                fmt::write(&mut tag_code, format_args!("{write_fn}(\"{name}\", {tag} as *mut _, {target});")).expect("*sad Butterfree noises*");
                added_tag.insert(tag);
            }
            if let Some(cache) = hook.cache {
                if added_cache.contains(&cache) {
                    fmt::write(&mut forbidden_code, format_args!("{{ compile_error!(\"Duplicate cache addr {cache}\") }}")).expect(";-;");
                    break;
                }
                fmt::write(&mut cache_code, format_args!("{write_fn}(\"{name}\", {cache} as *mut _, {target});")).expect("*sad Butterfree noises*");
                added_cache.insert(cache);
            }
            if added.contains(&name) {
                fmt::write(&mut forbidden_code, format_args!("{{ compile_error!(\"Duplicate hook {name}\") }}")).expect(";-;");
                break;
            }
            added.insert(name);
        }
    }

    format!("{forbidden_code} match get_exe_type() {{\nExeType::Cache => {{ {cache_code} }},\nExeType::Tag => {{ {tag_code} }} }}").parse().expect(";-;")
}


#[proc_macro]
pub fn generate_hs_external_globals_array(_: TokenStream) -> TokenStream {
    #[derive(Deserialize)]
    struct Entry {
        name: String,
        r#type: String,
        address: String
    }

    let cache_json_data = include_bytes!("../globals/cache.json");
    let tag_json_data = include_bytes!("../globals/tag.json");
    let demon_json_data = include_bytes!("../globals/demon.json");

    let cache_entries: Vec<Entry> = serde_json::from_slice(cache_json_data).expect("failed to parse cache globals list");
    let tag_entries: Vec<Entry> = serde_json::from_slice(tag_json_data).expect("failed to parse tags globals list");
    let demon_entries: Vec<Entry> = serde_json::from_slice(demon_json_data).expect("failed to parse demon globals list");

    fn make_data(entries: Vec<Entry>) -> String {
        let mut data = String::with_capacity(4096);
        for i in entries {
            let name = i.name;
            let global_type = i.r#type;
            let mut address = i.address;

            if !address.starts_with("0x") {
                address = format!("unsafe {{ core::mem::transmute(&mut {address} as *mut _) }}")
            }
            else {
                address = format!("{address} as *mut [u8; 0]")
            }

            fmt::write(&mut data, format_args!("ExternalGlobal::new(b\"{name}\\x00\", ScriptValueType::{global_type}, {address}),\n")).expect("A");
        }
        data
    }

    let mut cache_list = make_data(cache_entries);
    let mut tag_list = make_data(tag_entries);
    let demon_list = make_data(demon_entries);

    cache_list += &demon_list;
    tag_list += &demon_list;

    format!("{{\
    const CACHE_DEFINITIONS: &[ExternalGlobal] = &[{cache_list}];
    const TAG_DEFINITIONS: &[ExternalGlobal] = &[{tag_list}];

    (CACHE_DEFINITIONS, TAG_DEFINITIONS)
    }}").parse().expect("should've parsed")
}
