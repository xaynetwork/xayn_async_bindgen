use std::{collections::HashMap, io::Write};

use anyhow::{anyhow, Error};
use handlebars::Handlebars;
use once_cell::sync::Lazy;
use serde_json::json;

use crate::{Language, interface::InterfaceDescription};

use super::parse_genesis::DartFunctionSignature;

static DART_SKELETON_TMPL_STR: &str = r#"
extension {{fficlass}}AsyncExt on {{fficlass}} {
    {{#each functions}}
        Future<{{rtype}}> {{name}}(
            {{#each inputs}}
                {{type}} {{name}},
            {{/each}}
        ) {
            final completer = FfiCompleterRegistry.newCompleter();
            final rcode = {{nativename}}(
                {{#each inputs}}
                    {{name}},
                {{/each}}
                completer.portId,
                completer.completerId,
            );
            if (rcode != 0) {
                //TODO
                throw Exception('failed to setup callbacks');
            }
            return completer.future;
        }
    {{/each}}
}
"#;

static DART_SKELETON_TMPL_NAME: &str = "dart_ext_skeleton";
static DART_SKELETON_TMPL: Lazy<Handlebars> = Lazy::new(|| {
    let mut reg = Handlebars::new();
    reg.register_template_string(DART_SKELETON_TMPL_NAME, DART_SKELETON_TMPL_STR).unwrap();
    reg
});

#[allow(unused)]
pub(crate) fn generate(
    ifd: &InterfaceDescription,
    dart_signatures: HashMap<String, DartFunctionSignature>,
    ffi_class: &str,
    out: impl Write,
) -> Result<(), Error> {
    let functions = ifd.functions()
        .iter()
        .map(|func| {
            let name = func.name().to_string();
            let native_name = func.ffi_wrapper_name(Language::Dart);
            let dart_sig = dart_signatures.get(&native_name)
                .ok_or_else(||anyhow!("No native wrapper was found for function: {}", name))?;

            let inputs = dart_sig.inputs()
                .iter()
                .map(|input| {
                    json!({
                        "name": input.name(),
                        "type": input.r#type(),
                    })
                })
                .collect::<Vec<_>>();

            Ok(json!({
                "rtype": dart_sig.return_type(),
                "name": name,
                "nativename": native_name,
                "inputs": inputs,
            }))
        })
        .collect::<Result<Vec<_>, Error>>()?;

    let data = json!({
        "fficlass": ffi_class,
        "functions": functions
    });

    DART_SKELETON_TMPL.render_template_to_write(DART_SKELETON_TMPL_NAME, &data, out)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rendering_template_works() {
        //TODO
    }
}