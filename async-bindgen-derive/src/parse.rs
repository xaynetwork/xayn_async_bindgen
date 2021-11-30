use proc_macro2::TokenStream;
use proc_macro_error::ResultExt;
use syn::{FnArg, Ident, Item, Signature, Type};

use crate::{
    error::{abort, emit_error},
    Language,
};

/// Simplified version of `syn::Signature`.
pub(crate) struct FunctionInfo {
    name: Ident,
    output: Option<Type>,
    inputs: Vec<FunctionInput>,
}

impl FunctionInfo {
    pub(crate) fn name(&self) -> &Ident {
        &self.name
    }

    #[allow(unused)]//TODO fixme
    pub(crate) fn output(&self) -> Option<&Type> {
        self.output.as_ref()
    }

    pub(crate) fn inputs(&self) -> &[FunctionInput] {
        &self.inputs
    }

    pub(crate) fn ffi_wrapper_name(&self, lang: Language) -> String {
        format!("_async_bindgen_{}_w__{}", lang.as_str(), self.name())
    }

    pub(crate) fn parse(item: TokenStream) -> Self {
        let item: Item = syn::parse2(item).expect_or_abort("parsing item failed");
        if let Item::Fn(func) = item {
            Self::from_signature(&func.sig)
        } else {
            abort!(item, "expected an annotated function");
        }
    }

    pub(crate) fn from_signature(sig: &Signature) -> Self {
        if sig.asyncness.is_none() {
            emit_error!(sig, "async bindgen only works with async functions"  => {});
        }
        let name = sig.ident.clone();
        let output = match &sig.output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, ty) => Some((**ty).clone()),
        };
        let inputs = sig
            .inputs
            .iter()
            .filter_map(|input| match input {
                FnArg::Receiver(_) => {
                    emit_error!(input, "`self` is not supported by async bindgen" => return None);
                }
                FnArg::Typed(arg) => {
                    let name = match &*arg.pat {
                        syn::Pat::Ident(name) => name.ident.clone(),
                        _ => {
                            emit_error!(
                                name,
                                "patterns in function arguments are not supported by async bindgen"
                                => return None
                            );
                        }
                    };
                    let r#type = (*arg.ty).clone();
                    Some(FunctionInput { name, r#type })
                }
            })
            .collect::<Vec<_>>();

        FunctionInfo {
            name,
            output,
            inputs,
        }
    }
}

pub(crate) struct FunctionInput {
    name: Ident,
    r#type: Type,
}

impl FunctionInput {
    pub(crate) fn new(name: Ident, r#type: Type) -> Self {
        Self { name, r#type }
    }
    pub(crate) fn name(&self) -> &Ident {
        &self.name
    }

    pub(crate) fn r#type(&self) -> &Type {
        &self.r#type
    }
}
