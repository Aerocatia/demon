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
    pub replacement: Option<String>,
    pub sudo: Option<bool>
}

#[proc_macro]
pub fn pointer_from_hook(t: TokenStream) -> TokenStream {
    let parsed: syn::LitStr = syn::parse(t).expect("expected a literal string");
    let name = parsed.value();
    let all_hooks = get_all_hooks();
    let Some(hook) = all_hooks.get(&parsed.value()) else {
        return format!("compile_error!(\"No such hook `{name}`\");").parse().expect(";-;")
    };

    let cache = hook.cache.as_ref().map(String::as_str).unwrap_or("0x00000000");
    let tag = hook.tag.as_ref().map(String::as_str).unwrap_or("0x00000000");

    format!("
pointer! {{
    name: \"{name}\",
    cache_address: {cache},
    tag_address: {tag}
}}").parse().expect(";-;")
}

fn get_all_hooks() -> HashMap<String, Hook> {
    let mut hooks: HashMap<String, Hook> = HashMap::new();
    let mut cache_addresses: HashSet<String> = HashSet::new();
    let mut tag_addresses: HashSet<String> = HashSet::new();

    for i in std::fs::read_dir("c_mine/hook").expect("WHERE?").filter_map(|d| d.ok()) {
        let path = i.path();
        if path.extension() != Some("json".as_ref()) {
            continue
        }
        let data = std::fs::read(path).expect("failed to read hook JSON");
        let parsed: HashMap<String, Hook> = serde_json::from_slice(data.as_slice()).expect("failed to parse JSON");
        for (name, hook) in parsed {
            if hooks.contains_key(&name) {
                panic!("Duplicate hook {name}")
            }
            if let Some(t) = hook.tag.as_ref() {
                if tag_addresses.contains(t) {
                    panic!("Duplicate tag build address {t} ({name})")
                }
                tag_addresses.insert(t.to_owned());
            }
            if let Some(t) = hook.cache.as_ref() {
                if cache_addresses.contains(t) {
                    panic!("Duplicate cache build address {t} ({name})")
                }
                cache_addresses.insert(t.to_owned());
            }
            hooks.insert(name, hook);
        }
    }

    hooks
}

#[proc_macro]
pub fn generate_hook_setup_code(_: TokenStream) -> TokenStream {
    let mut tag_code = String::with_capacity(65536);
    let mut cache_code = String::with_capacity(65536);
    let mut codegen = String::with_capacity(65536);
    let mut c_targets_generated = HashSet::new();

    for (name, hook) in get_all_hooks() {
        let mut target = hook
            .replacement
            .as_ref()
            .map(String::as_str)
            .unwrap_or("original");

        if target.starts_with("_") {
            let function = &target[1..];
            target = &name;

            if c_targets_generated.insert(function.to_owned()) {
                fmt::write(&mut codegen, format_args!("extern {{ fn {function}(); }}")).expect(";-;");
            }

            fmt::write(&mut codegen, format_args!("
const {name}: crate::util::CFunctionProvider<unsafe extern \"C\" fn()> = crate::util::CFunctionProvider {{
    name: \"{name}\",
    function_getter: || {function},
    address_getter: |a| {{
        a as *const _
    }}
}};

")).expect(";-;");
        }

        else if target == "forbid" {
            target = &name;
            fmt::write(&mut codegen, format_args!("#[c_mine] extern \"C\" fn {name}() {{ panic!(\"Entered stubbed-out function `{name}`\") }}")).expect(";-;");
        }

        else if target == "error" {
            target = &name;
            fmt::write(&mut codegen, format_args!("#[c_mine] extern \"C\" fn {name}() {{ error!(\"Entered stubbed-out function `{name}`\") }}")).expect(";-;");
        }

        else if target == "nop" {
            target = &name;
            fmt::write(&mut codegen, format_args!("#[c_mine] extern \"C\" fn {name}() {{ }}")).expect(";-;");
        }

        else if target == "original" {
            continue
        }

        let write_fn = if hook.sudo == Some(true) {
            "write_jmp"
        }
        else {
            "overwrite_thunk"
        };

        if let Some(tag) = hook.tag {
            fmt::write(&mut tag_code, format_args!("{write_fn}(\"{name}\", {tag} as *mut _, {target});")).expect("*sad Butterfree noises*");
        }
        if let Some(cache) = hook.cache {
            fmt::write(&mut cache_code, format_args!("{write_fn}(\"{name}\", {cache} as *mut _, {target});")).expect("*sad Butterfree noises*");
        }
    }

    format!("{codegen} match get_exe_type() {{\nExeType::Cache => {{ {cache_code} }},\nExeType::Tag => {{ {tag_code} }} }}").parse().expect(";-;")
}


#[proc_macro]
pub fn generate_hs_external_globals_array(_: TokenStream) -> TokenStream {
    let all_globals = get_all_globals();

    fn make_data(entries: &HashMap<String, ExternalGlobal>, address_type: &str) -> String {
        let mut data = String::with_capacity(4096);
        for i in entries.values() {
            let name = &i.name;
            let global_type = &i.r#type;
            let Some(address) = match address_type {
                "demon" => &i.address_demon,
                "tag" => &i.address_tag,
                "cache" => &i.address_cache,
                _ => unreachable!()
            }.as_ref() else {
                continue
            };

            fmt::write(&mut data, format_args!("ExternalGlobal::new(b\"{name}\\x00\", ScenarioScriptValueType::{global_type}, ")).expect(";-;");
            fmt::write(&mut data, format_args!("{}", format_address(address))).expect(";-;");
            fmt::write(&mut data, format_args!("),\n")).expect(";-;");
        }
        data
    }

    let mut cache_list = make_data(&all_globals, "cache");
    let mut tag_list = make_data(&all_globals, "tag");
    let demon_list = make_data(&all_globals, "demon");

    cache_list += &demon_list;
    tag_list += &demon_list;

    format!("{{\
    const CACHE_GLOBAL_DEFINITIONS: &[ExternalGlobal] = &[{cache_list}];
    const TAG_GLOBAL_DEFINITIONS: &[ExternalGlobal] = &[{tag_list}];

    (CACHE_GLOBAL_DEFINITIONS, TAG_GLOBAL_DEFINITIONS)
    }}").parse().expect("should've parsed")
}

/// # Safety
///
/// For Demon-internal globals, this is safe.
///
/// Otherwise, this will generate code that does pointer dereferencing. While the pointer does point
/// to valid data, no guarantee is made that there isn't anything accessing it concurrently.
#[proc_macro]
pub fn get_hs_global(token_stream: TokenStream) -> TokenStream {
    get_hs_global_with_borrow(token_stream, "&")
}

/// # Safety
///
/// For Demon-internal globals, this is safe.
///
/// Otherwise, this will generate code that does pointer dereferencing. While the pointer does point
/// to valid data, no guarantee is made that there isn't anything accessing it concurrently.
#[proc_macro]
pub fn get_hs_global_mut(token_stream: TokenStream) -> TokenStream {
    get_hs_global_with_borrow(token_stream, "&mut ")
}

fn get_hs_global_with_borrow(token_stream: TokenStream, borrow: &str) -> TokenStream {
    let parsed: syn::LitStr = syn::parse(token_stream).expect("expected a literal string");
    let name = parsed.value();
    let all_globals = get_all_globals();
    let Some(global) = all_globals.get(&parsed.value()) else {
        return format!("compile_error!(\"No such global `{name}`\");").parse().expect(";-;")
    };
    if let Some(global) = global.address_demon.as_ref() {
        return format!("{borrow}{global}").parse().expect(";-;")
    };

    let type_to_use = match global.r#type.as_str() {
        // these should be zero or one, but we want to make doubly sure Rust won't explode
        "Boolean" => "u8",
        "Real" => "f32",
        "Short" => "i16",
        "Long" => "i32",
        n => return format!("compile_error!(\"`{name}` is a {n} which cannot be used with get_hs_global_* methods\");").parse().expect(";-;")
    };

    let cache = match global.address_cache.as_ref() {
        Some(n) => format!("{borrow}*({n} as *mut {type_to_use})"),
        None => format!("panic!(\"{name} is not available on cache builds\")")
    };

    let tag = match global.address_tag.as_ref() {
        Some(n) => format!("{borrow}*({n} as *mut {type_to_use})"),
        None => format!("panic!(\"{name} is not available on tag builds\")")
    };

    format!("match crate::init::get_exe_type() {{ crate::init::ExeType::Cache => {cache}, crate::init::ExeType::Tag => {tag} }}").parse().expect(";-;")
}

#[derive(Deserialize)]
struct HSFunctionEntry {
    name: String,
    r#return_type: String,
    compile: String,
    evaluate: String,
    description: Option<String>,
    usage: Option<String>,
    arguments: Vec<String>
}

#[derive(Clone)]
struct HSFunction {
    name: String,
    r#return_type: String,
    description: Option<String>,
    usage: Option<String>,
    arguments: Vec<String>,

    compile_evaluate_tag: Option<(String, String)>,
    compile_evaluate_cache: Option<(String, String)>,
    compile_evaluate_demon: Option<(String, String)>
}

fn get_all_scripting_functions() -> Vec<HSFunction> {
    let cache_json_data: Vec<HSFunctionEntry> = serde_json::from_slice(include_bytes!("../functions/cache.json")).expect("could not parse cache json scripting commands");
    let tag_json_data: Vec<HSFunctionEntry> = serde_json::from_slice(include_bytes!("../functions/tag.json")).expect("could not parse tag json scripting commands");
    let demon_json_data: Vec<HSFunctionEntry> = serde_json::from_slice(include_bytes!("../functions/demon.json")).expect("could not parse demon json scripting commands");

    let mut all_commands: Vec<String> = Vec::new();

    let mut group = |data: Vec<HSFunctionEntry>| {
        let mut functions: HashMap<String, HSFunctionEntry> = HashMap::new();

        for i in data {
            if !all_commands.contains(&i.name) {
                all_commands.push(i.name.clone());
            }
            functions.insert(i.name.clone(), i);
        }

        functions
    };

    let mut cache_functions = group(cache_json_data);
    let mut tag_functions = group(tag_json_data);
    let mut demon_functions = group(demon_json_data);

    let mut result = Vec::new();

    for i in all_commands {
        let first = cache_functions
            .get(&i)
            .or_else(|| tag_functions.get(&i))
            .or_else(|| demon_functions.get(&i))
            .expect("should somehow have an entry here");

        result.push(
            HSFunction {
                name: first.name.clone(),
                return_type: first.return_type.clone(),
                usage: first.usage.clone(),
                description: first.description.clone(),
                arguments: first.arguments.clone(),

                compile_evaluate_cache: cache_functions.remove(&i).map(|c| (c.compile, c.evaluate)),
                compile_evaluate_tag: tag_functions.remove(&i).map(|c| (c.compile, c.evaluate)),
                compile_evaluate_demon: demon_functions.remove(&i).map(|c| (c.compile, c.evaluate)),
            }
        );
    }

    #[derive(Deserialize)]
    struct Alias {
        target: String,
        aliases: Vec<String>
    }

    let aliases: Vec<Alias> = serde_json::from_slice(include_bytes!("../functions/alias.json")).expect("could not parse script aliases json");
    for i in aliases {
        let Some(original) = result.iter().find(|c| c.name == i.target) else {
            panic!("Alias {} refers to a function that doesn't exist.", i.target);
        };
        let original = original.clone();
        for name in i.aliases {
            result.push(HSFunction {
                name,
                ..original.clone()
            });
        }
    }

    result
}

#[proc_macro]
pub fn generate_hs_functions_array(_: TokenStream) -> TokenStream {
    let all_functions = get_all_scripting_functions();
    let all_hooks = get_all_hooks();

    fn make_data(entries: &Vec<HSFunction>, address_type: &str, hooks: &HashMap<String, Hook>) -> String {
        let mut data = String::with_capacity(4096);
        for i in entries {
            let name = cleanup_string(&i.name);
            let return_type = cleanup_string(&i.return_type);
            let description = cleanup_string(i.description.as_ref().map(|c| c.as_str()).unwrap_or(""));
            let usage = cleanup_string(i.usage.as_ref().map(|c| c.as_str()).unwrap_or(""));

            let Some((compile, evaluate)) = match address_type {
                "demon" => &i.compile_evaluate_demon,
                "tag" => &i.compile_evaluate_tag,
                "cache" => &i.compile_evaluate_cache,
                _ => unreachable!()
            }.as_ref() else {
                continue
            };

            fn resolve_ptr(address_type: &str, hooks: &HashMap<String, Hook>, ptr: &str) -> String {
                let compile = if address_type == "demon" {
                    format!("{ptr} as *const _")
                } else if let Some(c) = hooks.get(ptr) {
                    let address = match address_type {
                        "tag" => &c.tag,
                        "cache" => &c.cache,
                        _ => unreachable!()
                    }.as_ref();
                    match address {
                        Some(s) => format_address(&s).to_owned(),
                        None => panic!("Found hook {ptr}, but no address for {address_type}")
                    }
                } else {
                    format_address(&ptr)
                };
                compile
            }

            let compile = resolve_ptr(address_type, hooks, compile);
            let evaluate = resolve_ptr(address_type, hooks, evaluate);

            let mut arg_setter_code = String::new();
            for (i, j) in i.arguments.iter().enumerate() {
                arg_setter_code += &format!("arguments[{i}] = ScenarioScriptValueType::{j};");
            }

            fmt::write(&mut data, format_args!("HSScriptFunctionDefinition {{")).expect(";-;");
            fmt::write(&mut data, format_args!("name: CStrPtr::from_bytes(b\"{name}\\x00\"),")).expect(";-;");
            fmt::write(&mut data, format_args!("description: CStrPtr::from_bytes(b\"{description}\\x00\"),")).expect(";-;");
            fmt::write(&mut data, format_args!("usage: CStrPtr::from_bytes(b\"{usage}\\x00\"),")).expect(";-;");
            fmt::write(&mut data, format_args!("return_type: ScenarioScriptValueType::{return_type},")).expect(";-;");
            fmt::write(&mut data, format_args!("compile: {compile},")).expect(";-;");
            fmt::write(&mut data, format_args!("evaluate: {evaluate},")).expect(";-;");
            fmt::write(&mut data, format_args!("argument_count: {},", i.arguments.len())).expect(";-;");
            fmt::write(&mut data, format_args!("argument_types: const {{
                let mut arguments = [ScenarioScriptValueType::Unparsed; 6];
                {arg_setter_code}
                arguments
            }},")).expect(";-;");

            // SAFETY: This struct is perfectly safe to be zeroed.
            fmt::write(&mut data, format_args!("..unsafe {{ core::mem::zeroed() }}")).expect(";-;");

            fmt::write(&mut data, format_args!("}},\n")).expect(";-;");
        }
        data
    }


    let mut cache_list = make_data(&all_functions, "cache", &all_hooks);
    let mut tag_list = make_data(&all_functions, "tag", &all_hooks);
    let demon_list = make_data(&all_functions, "demon", &all_hooks);

    cache_list += &demon_list;
    tag_list += &demon_list;

    let parsed = format!("{{\
    const CACHE_FUNCTION_DEFINITIONS: &[HSScriptFunctionDefinition] = &[{cache_list}];
    const TAG_FUNCTION_DEFINITIONS: &[HSScriptFunctionDefinition] = &[{tag_list}];

    (CACHE_FUNCTION_DEFINITIONS, TAG_FUNCTION_DEFINITIONS)
    }}");

    parsed.parse().expect("should've parsed")
}

fn cleanup_string(string: &str) -> String {
    string
        .replace("\\", "\\\\")
        .replace("\n", "\\\n")
        .replace("\"", "\\\"")
}

fn format_address(address: &str) -> String {
    if !address.starts_with("0x") {
        format!("unsafe {{ core::mem::transmute(&mut {address} as *mut _) }}")
    }
    else {
        format!("{address} as *mut [u8; 0]")
    }
}

#[derive(Deserialize)]
struct ExternalGlobal {
    name: String,
    r#type: String,
    address_demon: Option<String>,
    address_cache: Option<String>,
    address_tag: Option<String>,
}

fn get_all_globals() -> HashMap<String, ExternalGlobal> {
    #[derive(Deserialize)]
    struct ExternalGlobalEntry {
        name: String,
        r#type: String,
        address: String,
    }

    let mut result = HashMap::new();
    let mut insert_entry = |what: ExternalGlobalEntry, address_type: &str| {
        let mut global = result.get_mut(&what.name);
        if global.is_none() {
            result.insert(what.name.clone(), ExternalGlobal {
                name: what.name.clone(),
                r#type: what.r#type.clone(),
                address_cache: None,
                address_demon: None,
                address_tag: None
            });
            global = result.get_mut(&what.name);
        }
        let global = global.expect("we just inserted it!");
        assert!(what.r#type == global.r#type, "type mismatch for {}", what.name);
        let addr = match address_type {
            "demon" => &mut global.address_demon,
            "tag" => &mut global.address_tag,
            "cache" => &mut global.address_cache,
            _ => unreachable!()
        };
        if address_type == "demon" {
            assert!(!what.address.starts_with("0x"), "demon-internal globals must not use hexadecimal addresses")
        }
        else {
            assert!(what.address.starts_with("0x"), "non-demon-internal globals must use hexadecimal addresses")
        }
        assert!(addr.is_none(), "multiple entries for {} in the same build type {address_type} with different addresses", what.name);
        *addr = Some(what.address);
    };

    let cache_json_data = include_bytes!("../globals/cache.json");
    let tag_json_data = include_bytes!("../globals/tag.json");
    let demon_json_data = include_bytes!("../globals/demon.json");

    let cache_entries: Vec<ExternalGlobalEntry> = serde_json::from_slice(cache_json_data).expect("failed to parse cache globals list");
    let tag_entries: Vec<ExternalGlobalEntry> = serde_json::from_slice(tag_json_data).expect("failed to parse tags globals list");
    let demon_entries: Vec<ExternalGlobalEntry> = serde_json::from_slice(demon_json_data).expect("failed to parse demon globals list");

    for i in cache_entries {
        insert_entry(i, "cache");
    }
    for i in tag_entries {
        insert_entry(i, "tag");
    }
    for i in demon_entries {
        insert_entry(i, "demon");
    }

    result
}
