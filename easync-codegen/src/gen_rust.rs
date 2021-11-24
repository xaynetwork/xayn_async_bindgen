use std::fs;

use anyhow::{bail, Error};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Expr, FnArg, Pat, Type};

use crate::{
    config::Config,
    dart,
    interface::{FunctionSignature, InterfaceDescription},
    Language,
};

pub(crate) fn generate(
    ifd: &InterfaceDescription,
    lang: Language,
    config: &Config,
) -> Result<(), Error> {
    let tokens = generate_token_stream(ifd, lang)?;
    fs::write(&config.rust_out, tokens.to_string())?;
    Ok(())
}

fn generate_token_stream(ifd: &InterfaceDescription, lang: Language) -> Result<TokenStream, Error> {
    let functions = ifd
        .functions()
        .iter()
        .map(|func| generate_wrapped_function(func, lang))
        .collect::<Result<Vec<_>, _>>()?;

    let imports = match lang {
        Language::Dart => dart::gen_rust::imports(),
    };

    Ok(quote! {
        #imports

        #(#functions)*
    })
}

fn generate_wrapped_function(
    func: &FunctionSignature,
    lang: Language,
) -> Result<TokenStream, Error> {
    let name = Ident::new(func.name(), Span::call_site());
    let wrapped_name = Ident::new(&func.ffi_wrapper_name(lang), Span::call_site());

    let add_inputs = match lang {
        Language::Dart => dart::gen_rust::additional_dart_inputs(),
    };

    let wrapper_function_arg_names = func
        .inputs()
        .iter()
        .chain(add_inputs.iter())
        .map(|inp| syn::parse_str::<Ident>(inp.name()).unwrap());
    let wrapper_function_arg_types = func
        .inputs()
        .iter()
        .chain(add_inputs.iter())
        .map(|inp| syn::parse_str::<Type>(inp.r#type()).unwrap());

    let completer_args = add_inputs.iter().map(|inp| syn::parse_str::<Ident>(inp.name()).unwrap());
    let call_names = func.inputs().iter().map(|inp| syn::parse_str::<Ident>(inp.name()).unwrap());

    let tokens = quote! {
        #[no_mangle]
        extern "C" fn #wrapped_name (#(#wrapper_function_arg_names: #wrapper_function_arg_types),*) -> isize {
            let completer = match PreparedCompleter::new(#(#completer_args),*) {
                Ok(c) => c,
                Err(_) => return -1,
            };
            let bound = completer.bind_future(#name(#(#call_names),*));
            spawn(bound);
        }
    };

    Ok(tokens)
}

fn arg_to_expr(arg: &FnArg) -> Result<Expr, Error> {
    match arg {
        FnArg::Receiver(_) => Ok(Expr::Verbatim(quote! { self })),
        FnArg::Typed(typed) => {
            if let Pat::Ident(pat_ident) = &*typed.pat {
                let ident = &pat_ident.ident;
                Ok(Expr::Verbatim(quote! { #ident }))
            } else {
                bail!("no patterns as function arguments allowed");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::assert_rust_code_eq;

    #[test]
    fn test_expected_output() {
        let input = r#"
            async fn dork(x: i32, y: *const i32) -> i64
            async fn dodo()
        "#;

        let mut ifd = InterfaceDescription::empty();

        ifd.parse_functions(input).unwrap();
        assert_eq!(ifd.functions().len(), 2);

        let tokens = generate_token_stream(&ifd, Language::Dart).unwrap();
        assert_rust_code_eq!(
            tokens.to_string(),
            r#"
            use easync_rust_dart_io_utils::{spawn, PreparedCompleter, DartPortId};

            #[no_mangle]
            extern "C" fn easync_dart__dork(
                x: i32,
                y: *const i32,
                easync_dart_port_id: DartPortId,
                easync_dart_completer_id: i64
            ) -> isize {
                let completer = match PreparedCompleter::new(easync_dart_port_id, easync_dart_completer_id) {
                    Ok(c) => c,
                    Err(_) => return -1,
                };
                let bound = completer.bind_future(dork(x , y));
                spawn(bound);
            }

            #[no_mangle]
            extern "C" fn easync_dart__dodo(
                easync_dart_port_id: DartPortId,
                easync_dart_completer_id: i64
            ) -> isize {
                let completer = match PreparedCompleter::new(easync_dart_port_id, easync_dart_completer_id) {
                    Ok(c) => c,
                    Err(_) => return -1,
                };
                let bound = completer.bind_future(dodo());
                spawn(bound);
            }
        "#
        );
    }
}
