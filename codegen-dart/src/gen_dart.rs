use std::io::Write;

use anyhow::Error;
use handlebars::Handlebars;
use once_cell::sync::Lazy;
use serde::Serialize;

use super::parse_genesis::AsyncFunctionSignature;

static DART_SKELETON_TMPL_STR: &str = r#"
import 'package:async_bindgen_dart_utils/async_bindgen_dart_utils.dart';

extension {{ffi_class}}AsyncExt on {{ffi_class}} {
    {{#each functions}}
        Future<{{output}}> {{name}}(
            {{#each inputs}}
                {{type}} {{name}},
            {{/each}}
        ) {
            final setup = FfiCompleterRegistry.newCompleter();
            final call_ok = {{ffi_call_name}}(
                {{#each inputs}}
                    {{name}},
                {{/each}}
                setup.portId,
                setup.completerId,
            );
            if (call_ok == 0) {
                //TODO
                throw Exception('failed to setup callbacks');
            }
            setup.extractor = {{ffi_ret_name}};
            return setup.future;
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

pub(crate) fn generate(
    ffi_class: &str,
    functions: &[AsyncFunctionSignature],
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
    functions: &'a [AsyncFunctionSignature],
}

#[cfg(test)]
mod tests {
    use crate::{parse_genesis::DartFunctionInputs, test_utils::assert_trimmed_line_eq};

    use super::*;

    #[test]
    fn test_rendering_template_works() {
        let functions = &[
            AsyncFunctionSignature {
                doc: vec![],
                name: "func1".into(),
                ffi_call_name: "c_foobar_func1".into(),
                ffi_ret_name: "r_foobar_func1".into(),
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
            AsyncFunctionSignature {
                doc: vec![],
                name: "d1".into(),
                ffi_call_name: "foobar_d1c".into(),
                ffi_ret_name: "foobar_d1r".into(),
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
            import 'package:async_bindgen_dart_utils/async_bindgen_dart_utils.dart';

            extension DodoFfiAsyncExt on DodoFfi {
                Future<int> func1(
                    ffi.Pointer<int> foo,
                    double bar,
                ) {
                    final setup = FfiCompleterRegistry.newCompleter();
                    final call_ok = c_foobar_func1(
                        foo,
                        bar,
                        setup.portId,
                        setup.completerId,
                    );
                    if (call_ok == 0) {
                        //TODO
                        throw Exception('failed to setup callbacks');
                    }
                    setup.extractor = r_foobar_func1;
                    return setup.future;
                }
                Future<ffi.Pointer<AStruct>> d1(
                ) {
                    final setup = FfiCompleterRegistry.newCompleter();
                    final call_ok = foobar_d1c(
                        setup.portId,
                        setup.completerId,
                    );
                    if (call_ok == 0) {
                        //TODO
                        throw Exception('failed to setup callbacks');
                    }
                    setup.extractor = foobar_d1r;
                    return setup.future;
                }
            }
        "
        );
    }
}
