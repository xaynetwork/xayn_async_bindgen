use std::fs;

use anyhow::{anyhow, bail, Error};
use quote::ToTokens;
use syn::{FnArg, Signature};

use crate::{config::Config, Language};

pub(crate) struct FunctionSignature {
    name: String,
    return_type: Option<String>,
    inputs: Vec<FunctionInputs>,
}

impl FunctionSignature {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn return_type(&self) -> Option<&str> {
        self.return_type.as_deref()
    }

    pub(crate) fn inputs(&self) -> &[FunctionInputs] {
        &self.inputs
    }

    pub fn ffi_wrapper_name(&self, lang: Language) -> String {
        format!("easync_{}_w__{}", lang.as_str(), self.name())
    }
}

impl TryFrom<Signature> for FunctionSignature {
    type Error = Error;

    fn try_from(sig: Signature) -> Result<Self, Self::Error> {
        let name = sig.ident.to_string();
        let return_type = match sig.output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, rtype) => Some(rtype.to_token_stream().to_string()),
        };
        let inputs = sig
            .inputs
            .iter()
            .map(|input| match input {
                FnArg::Receiver(_) => bail!("can not wrap methods, but fund self receiver"),
                FnArg::Typed(arg) => {
                    let name = match &*arg.pat {
                        syn::Pat::Ident(name) => name.ident.to_string(),
                        _ => {
                            bail!("can not wrap functions with patterns in the function arguments")
                        }
                    };
                    let r#type = arg.ty.to_token_stream().to_string();
                    Ok(FunctionInputs { name, r#type })
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(FunctionSignature {
            name,
            return_type,
            inputs,
        })
    }
}

pub(crate) struct FunctionInputs {
    name: String,
    r#type: String,
}

impl FunctionInputs {
    pub(crate) fn new(name: impl Into<String>, r#type: impl Into<String>) -> Self {
        FunctionInputs {
            name: name.into(),
            r#type: r#type.into(),
        }
    }
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn r#type(&self) -> &str {
        &self.r#type
    }
}

pub(crate) struct InterfaceDescription {
    functions: Vec<FunctionSignature>,
    //TODO type mappings
}

impl InterfaceDescription {
    pub(crate) fn empty() -> Self {
        InterfaceDescription {
            functions: Vec::new(),
        }
    }

    pub fn parse(config: &Config) -> Result<Self, Error> {
        let src = fs::read_to_string(&config.interface_description)?;
        let mut ifd = Self::empty();
        ifd.parse_functions(&src)?;
        Ok(ifd)
    }

    pub(crate) fn functions(&self) -> &[FunctionSignature] {
        &self.functions
    }

    pub(crate) fn parse_functions(&mut self, src: &str) -> Result<(), Error> {
        for (idx, line) in src.lines().enumerate() {
            let line = line.trim();
            if !line.is_empty() {
                self.parse_function(line.trim()).map_err(|err| {
                    anyhow!("Parsing function signature on line {} failed: {}", idx, err)
                })?;
            }
        }
        Ok(())
    }

    pub(crate) fn parse_function(&mut self, function: &str) -> Result<(), Error> {
        let sig: Signature = syn::parse_str(function)?;
        let sig = sig.try_into()?;
        self.functions.push(sig);
        Ok(())
    }
}
