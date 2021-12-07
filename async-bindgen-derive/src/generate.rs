use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    parse::{api::Api, function::FunctionInfo},
    utils::type_from_path_and_name,
    Language,
};

mod dart_glue;


pub(crate) fn generate_type(api: &Api) -> TokenStream {
    let type_name = api.type_name();
    quote! {
        pub struct #type_name;
    }
}

pub(crate) fn generate_type_import(api: &Api) -> TokenStream {
    let api_name = api.mod_name();
    let api_type_name = api.type_name();
    quote! {
        use crate::#api_name::#api_type_name;
    }
}

pub(crate) fn generate_extern_functions(api: &Api, lang: Language) -> TokenStream {
    api.functions()
        .iter()
        .fold(TokenStream::new(), |mut res, func| {
            let func = generate_extern_function(api, func, lang);
            res.extend(func);
            res
        })
}

fn generate_extern_function(api: &Api, func: &FunctionInfo, lang: Language) -> TokenStream {
    let api_type_name = api.type_name();
    let async_name = func.name();

    let (path_prefix, add_inputs, call_name, ret_name) = match lang {
        Language::Dart => (
            dart_glue::path_prefix(),
            dart_glue::additional_dart_inputs(),
            dart_glue::call_name(api.mod_name(), async_name),
            dart_glue::ret_name(api.mod_name(), async_name),
        ),
    };

    let wrapper_function_arg_names = func
        .inputs()
        .iter()
        .chain(add_inputs.iter())
        .map(|inp| inp.name());
    let wrapper_function_arg_types = func
        .inputs()
        .iter()
        .chain(add_inputs.iter())
        .map(|inp| inp.r#type());

    let completer_args = add_inputs.iter().map(|inp| inp.name());
    let async_call_args = func.inputs().iter().map(|inp| inp.name());

    let completer = type_from_path_and_name(path_prefix.clone(), "PreparedCompleter");
    let output = func.output();

    quote! {
        #[no_mangle]
        pub extern "C" fn #call_name(#(#wrapper_function_arg_names: #wrapper_function_arg_types),*)
        -> u8 {
            match #completer::new(#(#completer_args),*) {
                Ok(completer) => {
                    completer.spawn(#api_type_name::#async_name(#(#async_call_args),*));
                    1
                }
                Err(_) => 0
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn #ret_name(handle: i64) -> #output {
            unsafe { #completer::extract_result(handle) }
        }
    }
}
