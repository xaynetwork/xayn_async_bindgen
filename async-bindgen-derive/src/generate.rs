use proc_macro2::TokenStream;
use quote::quote;

use crate::{parse::FunctionInfo, utils::type_from_path_and_name, Language};

mod dart_glue;

pub(crate) fn generate_wrapper(func: &FunctionInfo, lang: Language) -> TokenStream {
    let async_name = func.name();

    let (path_prefix, add_inputs, call_name, ret_name) = match lang {
        Language::Dart => (
            dart_glue::path_prefix(),
            dart_glue::additional_dart_inputs(),
            dart_glue::call_name(async_name),
            dart_glue::ret_name(async_name),
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
        extern "C" fn #call_name(#(#wrapper_function_arg_names: #wrapper_function_arg_types),*)
        -> Option<extern "C" fn(i64) -> #output> {
            #completer::new(#(#completer_args),*)
                .ok()?
                .spawn(#async_name(#(#async_call_args),*));
            Some(#ret_name)
        }

        extern "C" fn #ret_name(handle: i64) -> #output {
            #completer::extract_result(handle)
        }
    }
}
