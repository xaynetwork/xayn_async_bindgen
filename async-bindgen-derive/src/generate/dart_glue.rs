use proc_macro2::Span;
use syn::{Ident, Path};

use crate::{utils::{type_from_name, type_from_path_and_name}, parse::function::FunctionInput};

pub(crate) fn additional_dart_inputs() -> Vec<FunctionInput> {
    vec![
        FunctionInput::new(
            Ident::new("async_bindgen_dart_port_id", Span::call_site()),
            type_from_path_and_name(path_prefix(), "DartPortId"),
        ),
        FunctionInput::new(
            Ident::new("async_bindgen_dart_completer_id", Span::call_site()),
            type_from_name("i64"),
        ),
    ]
}

pub(crate) fn path_prefix() -> Path {
    syn::parse_str("::async_bindgen::dart").unwrap()
}

pub(crate) fn call_name(api_name: &Ident, fn_name: &Ident) -> Ident {
    Ident::new(
        &format!("async_bindgen_dart_c__{}__{}", api_name, fn_name),
        fn_name.span(),
    )
}

pub(crate) fn ret_name(api_name: &Ident, fn_name: &Ident) -> Ident {
    Ident::new(
        &format!("async_bindgen_dart_r__{}__{}", api_name, fn_name),
        fn_name.span(),
    )
}
