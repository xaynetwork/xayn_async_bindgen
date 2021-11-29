use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::{Language, parse::FunctionInfo, utils::type_from_path_and_name};

mod dart_glue;

pub(crate) fn generate_wrapper(
    func: &FunctionInfo,
    lang: Language,
) -> TokenStream {
    let async_name = func.name();
    let wrapped_name = Ident::new(&func.ffi_wrapper_name(lang), Span::call_site());

    let (path_prefix, add_inputs) = match lang {
        Language::Dart => {
            (dart_glue::path_prefix(), dart_glue::additional_dart_inputs())
        },
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

    let completer_args = add_inputs
        .iter()
        .map(|inp| inp.name());
    let call_names = func
        .inputs()
        .iter()
        .map(|inp| inp.name());

    let completer = type_from_path_and_name(path_prefix.clone(), "PreparedCompleter");
    let spawn = type_from_path_and_name(path_prefix, "spawn");

    quote! {
        #[no_mangle]
        extern "C" fn #wrapped_name (#(#wrapper_function_arg_names: #wrapper_function_arg_types),*) -> isize {
            let completer = match #completer::new(#(#completer_args),*) {
                Ok(c) => c,
                Err(_) => return -1,
            };
            let bound = completer.bind_future(#async_name(#(#call_names),*));
            #spawn(bound);
        }
    }
}