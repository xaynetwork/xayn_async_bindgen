use proc_macro2::Span;
use proc_macro_error::ResultExt;
use syn::{Ident, Path};

use crate::{parse::FunctionInput, utils::{type_from_name, type_from_path_and_name}};

pub(crate) fn additional_dart_inputs() -> Vec<FunctionInput> {
    vec![
        FunctionInput::new(
            Ident::new("_async_bindgen_dart_port_id", Span::call_site()),
            type_from_path_and_name(path_prefix(), "DartPortId"),
        ),
        FunctionInput::new(
            Ident::new("_async_bindgen_dart_completer_id", Span::call_site()),
            type_from_name("i64"),
        ),
    ]
}

pub(crate)fn path_prefix() -> Path {
    syn::parse_str("::async_bindgen::dart").unwrap_or_abort()
}