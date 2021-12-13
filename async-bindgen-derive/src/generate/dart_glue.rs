use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, Path};

use crate::{
    parse::function::FunctionInput,
    utils::{type_from_name, type_from_path_and_name},
};

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

pub(crate) fn generate_dart_api_init(api_name: &Ident) -> TokenStream {
    let init_name = Ident::new(
        &format!("async_bindgen_dart_init_api__{}", api_name),
        api_name.span(),
    );

    quote! {
        /// Initializes the dart api.
        ///
        /// Is safe to be called multiple times and form multiple
        /// thread.
        ///
        /// # Safety
        ///
        /// Must be called with a pointer produced by dart using
        /// `NativeApi.initializeApiDLData`.
        #[no_mangle]
        pub unsafe extern "C" fn #init_name(init_data: *mut ::std::ffi::c_void) -> u8 {
            let res = unsafe { ::dart_api_dl::initialize_dart_api_dl(init_data) };
            res.is_ok() as u8
        }
    }
}
