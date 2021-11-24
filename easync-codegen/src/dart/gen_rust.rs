use anyhow::Error;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::FnArg;

use crate::interface::FunctionInputs;

pub(crate) fn additional_dart_inputs() -> Vec<FunctionInputs> {
    vec![
        FunctionInputs::new("easync_dart_port_id", "DartPortId"),
        FunctionInputs::new("easync_dart_completer_id", "i64"),
    ]
}

pub(crate) fn imports() -> TokenStream {
    quote! { use easync_rust_dart_io_utils::{spawn, PreparedCompleter, DartPortId}; }
}
