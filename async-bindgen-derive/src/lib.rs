use error::abort_if_dirty;
use generate::generate_wrapper;
use parse::FunctionInfo;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::proc_macro_error;

mod error;
mod generate;
mod parse;
#[cfg(test)]
mod test_utils;
mod utils;

#[proc_macro_attribute]
#[proc_macro_error]
pub fn async_bindgen(attr: TokenStream1, item: TokenStream1) -> TokenStream1 {
    async_bindgen2(attr.into(), item.into()).into()
}

fn async_bindgen2(_attr: TokenStream2, mut item: TokenStream2) -> TokenStream2 {
    let desc = FunctionInfo::parse(item.clone());
    abort_if_dirty();
    for language in Language::languages() {
        let wrapper = generate_wrapper(&desc, language);
        item.extend(wrapper);
    }
    item
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

    use super::*;
    use crate::test_utils::assert_rust_code_eq;

    fn test_codegen(func: &str, expected_code: &str) {
        let attr = TokenStream2::new();
        let item = syn::parse_str::<TokenStream2>(func).unwrap();

        let resulting_tokens = async_bindgen2(attr, item);
        assert_rust_code_eq!(resulting_tokens.to_string(), expected_code);
    }

    #[test]
    fn test_codegen_no_arguments() {
        test_codegen(
            "async fn dodo() -> *const u8 { todo!() }",
            r#"
            async fn dodo() -> *const u8 { todo!() }
            #[no_mangle]
            extern "C" fn async_bindgen_dart_c__dodo(
                async_bindgen_dart_port_id: ::async_bindgen::dart::DartPortId,
                async_bindgen_dart_completer_id: i64
            ) -> Option<extern "C" fn(i64) -> *const u8> {
                ::async_bindgen::dart::PreparedCompleter::new(
                    async_bindgen_dart_port_id,
                    async_bindgen_dart_completer_id
                )
                    .ok()?
                    .spawn(dodo());
                Some(async_bindgen_dart_r__dodo)
            }
            extern "C" fn async_bindgen_dart_r__dodo(handle: i64) -> *const u8 {
                ::async_bindgen::dart::PreparedCompleter::extract_result(handle)
            }
        "#,
        );
    }

    #[test]
    fn test_codegen_with_arguments() {
        test_codegen(
            "async fn dork(x: i32, y: *const i32) -> isize { todo!() }",
            r#"
            async fn dork(x: i32, y: *const i32) -> isize { todo!() }
            #[no_mangle]
            extern "C" fn async_bindgen_dart_c__dork(
                x: i32,
                y: *const i32,
                async_bindgen_dart_port_id: ::async_bindgen::dart::DartPortId,
                async_bindgen_dart_completer_id: i64
            ) -> Option<extern "C" fn(i64) -> isize> {
                ::async_bindgen::dart::PreparedCompleter::new(
                    async_bindgen_dart_port_id,
                    async_bindgen_dart_completer_id
                )
                    .ok()?
                    .spawn(dork(x , y));
                Some(async_bindgen_dart_r__dork)
            }
            extern "C" fn async_bindgen_dart_r__dork(handle: i64) -> isize {
                ::async_bindgen::dart::PreparedCompleter::extract_result(handle)
            }
        "#,
        );
    }
}
