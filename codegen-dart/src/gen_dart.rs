use std::io::Write;

use anyhow::Error;
use handlebars::Handlebars;
use once_cell::sync::Lazy;
use serde::Serialize;

use super::parse_genesis::DartFunctionSignature;

static DART_SKELETON_TMPL_STR: &str = r#"
extension {{ffi_class}}AsyncExt on {{ffi_class}} {
    {{#each functions}}
        Future<{{output}}> {{name}}(
            {{#each inputs}}
                {{type}} {{name}},
            {{/each}}
        ) {
            final completer = FfiCompleterRegistry.newCompleter();
            final rcode = {{wrapped_name}}(
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
    reg.register_escape_fn(|v| v.into());
    reg.register_template_string(DART_SKELETON_TMPL_NAME, DART_SKELETON_TMPL_STR)
        .unwrap();
    reg
});

#[allow(dead_code)]
pub(crate) fn generate(
    ffi_class: &str,
    functions: &[DartFunctionSignature],
    out: impl Write,
) -> Result<(), Error> {
    DART_SKELETON_TMPL.render_to_write(
        DART_SKELETON_TMPL_NAME,
        &Context {
            ffi_class,
            functions,
        },
        out,
    )?;
    Ok(())
}

#[derive(Serialize)]
struct Context<'a> {
    ffi_class: &'a str,
    functions: &'a [DartFunctionSignature],
}

#[cfg(test)]
mod tests {
    use crate::{parse_genesis::DartFunctionInputs, test_utils::assert_trimmed_line_eq};

    use super::*;

    #[test]
    fn test_rendering_template_works() {
        let functions = &[
            DartFunctionSignature {
                doc: vec![],
                name: "func1".into(),
                wrapped_name: "foobar_func1".into(),
                output: "int".into(),
                inputs: vec![
                    DartFunctionInputs {
                        name: "foo".into(),
                        r#type: "ffi.Pointer<int>".into(),
                    },
                    DartFunctionInputs {
                        name: "bar".into(),
                        r#type: "double".into(),
                    },
                ],
            },
            DartFunctionSignature {
                doc: vec![],
                name: "d1".into(),
                wrapped_name: "foobar_d1".into(),
                output: "ffi.Pointer<AStruct>".into(),
                inputs: vec![],
            },
        ];
        let mut out = Vec::<u8>::new();
        generate("DodoFfi", functions, &mut out).unwrap();
        let out = String::from_utf8(out).unwrap();
        assert_trimmed_line_eq!(
            out,
            "
            extension DodoFfiAsyncExt on DodoFfi {
                Future<int> func1(
                    ffi.Pointer<int> foo,
                    double bar,
                ) {
                    final completer = FfiCompleterRegistry.newCompleter();
                    final rcode = foobar_func1(
                        foo,
                        bar,
                        completer.portId,
                        completer.completerId,
                    );
                    if (rcode != 0) {
                        //TODO
                        throw Exception('failed to setup callbacks');
                    }
                    return completer.future;
                }
                Future<ffi.Pointer<AStruct>> d1(
                ) {
                    final completer = FfiCompleterRegistry.newCompleter();
                    final rcode = foobar_d1(
                        completer.portId,
                        completer.completerId,
                    );
                    if (rcode != 0) {
                        //TODO
                        throw Exception('failed to setup callbacks');
                    }
                    return completer.future;
                }
            }
        "
        );
    }
}
