use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Span, TokenStream as TokenStream2};
use syn::Error;

use crate::{
    generate::{generate_extern_functions, generate_type, generate_type_import},
    parse::api::Api,
};

mod generate;
mod parse;
#[cfg(test)]
mod test_utils;
mod utils;

#[proc_macro_attribute]
pub fn api(attrs: TokenStream1, item: TokenStream1) -> TokenStream1 {
    api2(attrs.into(), item.into())
        .unwrap_or_else(|err| err.into_compile_error())
        .into()
}

fn api2(attrs: TokenStream2, item: TokenStream2) -> Result<TokenStream2, Error> {
    let res = parse_gen_api(attrs, item)?;
    let crate_root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    res.write_file(&crate_root).map_err(|err| {
        Error::new(
            Span::call_site(),
            format!("Failed to write binding file: {}", err),
        )
    })?;
    Ok(res.into_proc_macro_result())
}

fn parse_gen_api(attrs: TokenStream2, item: TokenStream2) -> Result<AsyncBindgenResult, Error> {
    let mut proc_tokens = item.clone();
    let api = Api::parse(attrs, item.clone())?;

    let mut file_tokens = generate_type(&api);
    proc_tokens.extend(generate_type_import(&api));

    for lang in Language::languages() {
        file_tokens.extend(generate_extern_functions(&api, lang))
    }

    Ok(AsyncBindgenResult {
        api,
        proc_tokens,
        file_tokens,
    })
}

struct AsyncBindgenResult {
    api: Api,
    proc_tokens: TokenStream2,
    file_tokens: TokenStream2,
}

impl AsyncBindgenResult {
    pub fn into_proc_macro_result(self) -> TokenStream2 {
        self.proc_tokens
    }

    pub fn write_file(&self, crate_root: &Path) -> Result<(), io::Error> {
        let file = crate_root
            .join("src")
            .join(self.api.mod_name().to_string())
            .with_extension("rs");
        fs::write(file, self.file_tokens.to_string())?;

        Ok(())
    }
}

#[derive(Clone, Copy)]
enum Language {
    Dart,
}

impl Language {
    pub fn languages() -> impl Iterator<Item = Self> {
        [Language::Dart].iter().copied()
    }
}

#[cfg(test)]
mod tests {

    // use super::*;
    // use crate::test_utils::assert_rust_code_eq;

    // fn test_codegen(func: &str, expected_code: &str) {
    //     let attr = TokenStream2::new();
    //     let item = syn::parse_str::<TokenStream2>(func).unwrap();

    //     let resulting_tokens = async_bindgen2(attr, item);
    //     assert_rust_code_eq!(resulting_tokens.to_string(), expected_code);
    // }

    // #[test]
    // fn test_codegen_no_arguments() {
    //     test_codegen(
    //         "pub async fn dodo() -> *const u8 { todo!() }",
    //         r#"
    //         pub async fn dodo() -> *const u8 { todo!() }
    //         #[no_mangle]
    //         pub extern "C" fn async_bindgen_dart_c__dodo(
    //             async_bindgen_dart_port_id: ::async_bindgen::dart::DartPortId,
    //             async_bindgen_dart_completer_id: i64
    //         ) -> u8 {
    //             match ::async_bindgen::dart::PreparedCompleter::new(
    //                 async_bindgen_dart_port_id,
    //                 async_bindgen_dart_completer_id
    //             ) {
    //                 Ok(completer) => {
    //                     completer.spawn(dodo());
    //                     1
    //                 }
    //                 Err(_) => 0
    //             }
    //         }
    //         #[no_mangle]
    //         pub unsafe extern "C" fn async_bindgen_dart_r__dodo(handle: i64) -> *const u8 {
    //             unsafe { ::async_bindgen::dart::PreparedCompleter::extract_result(handle) }
    //         }
    //     "#,
    //     );
    // }

    // #[test]
    // fn test_codegen_with_arguments() {
    //     test_codegen(
    //         "pub async fn dork(x: i32, y: *const i32) -> isize { todo!() }",
    //         r#"
    //         pub async fn dork(x: i32, y: *const i32) -> isize { todo!() }
    //         #[no_mangle]
    //         pub extern "C" fn async_bindgen_dart_c__dork(
    //             x: i32,
    //             y: *const i32,
    //             async_bindgen_dart_port_id: ::async_bindgen::dart::DartPortId,
    //             async_bindgen_dart_completer_id: i64
    //         ) -> u8 {
    //             match ::async_bindgen::dart::PreparedCompleter::new(
    //                 async_bindgen_dart_port_id,
    //                 async_bindgen_dart_completer_id
    //             ) {
    //                 Ok(completer) => {
    //                     completer.spawn(dork(x, y));
    //                     1
    //                 }
    //                 Err(_) => 0
    //             }
    //         }
    //         #[no_mangle]
    //         pub unsafe extern "C" fn async_bindgen_dart_r__dork(handle: i64) -> isize {
    //             unsafe { ::async_bindgen::dart::PreparedCompleter::extract_result(handle) }
    //         }
    //     "#,
    //     );
    // }
}
