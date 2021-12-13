use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    parse::{
        api::Api,
        function::{FunctionInfo, FunctionInput},
    },
    utils::type_from_path_and_name,
    Language,
};

use self::dart_glue::generate_dart_api_init;

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
        .fold(generate_api_init_functions(api, lang), |mut res, func| {
            res.extend(generate_extern_function(api, func, lang));
            res
        })
}

fn generate_api_init_functions(api: &Api, lang: Language) -> TokenStream {
    match lang {
        Language::Dart => generate_dart_api_init(api.mod_name()),
    }
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
        .map(FunctionInput::name);
    let wrapper_function_arg_types = func
        .inputs()
        .iter()
        .chain(add_inputs.iter())
        .map(FunctionInput::r#type);

    let completer_args = add_inputs.iter().map(FunctionInput::name);
    let async_call_args = func.inputs().iter().map(FunctionInput::name);

    let completer = type_from_path_and_name(path_prefix, "PreparedCompleter");
    let output = func.output();

    quote! {
        /// Wrapper for initiating the call to an async function.
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

        /// Extern "C"  wrapper delegating to `PreparedCompleter::extract_result()`.
        ///
        /// # Safety
        ///
        /// See the language specific version of `PreparedCompleter::extract_result()`.
        #[no_mangle]
        pub unsafe extern "C" fn #ret_name(handle: i64) -> #output {
            unsafe { #completer::extract_result(handle) }
        }
    }
}