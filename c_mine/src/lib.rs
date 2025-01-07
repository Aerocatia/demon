extern crate proc_macro;

use proc_macro::TokenStream;
use syn::__private::{quote, ToTokens};

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

