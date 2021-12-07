//! Ad-hoc parses a genesis.dart file for the type definitions produced by ffigen.
//!
use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct AsyncFunctionSignature {
    pub(crate) doc: Vec<String>,
    pub(crate) name: String,
    pub(crate) ffi_call_name: String,
    pub(crate) ffi_ret_name: String,
    pub(crate) output: String,
    pub(crate) inputs: Vec<DartFunctionInputs>,
}

impl AsyncFunctionSignature {
    fn from_call_and_ret_func(
        name: String,
        call_fn: DartFunctionSignature,
        ret_fn: DartFunctionSignature,
    ) -> Self {
        //FIXME better errors
        assert_eq!(call_fn.output, "int");
        assert_eq!(ret_fn.inputs.len(), 1);
        assert_eq!(ret_fn.inputs[0].r#type, "int");

        AsyncFunctionSignature {
            doc: call_fn.doc,
            name,
            ffi_call_name: call_fn.name,
            ffi_ret_name: ret_fn.name,
            output: ret_fn.output,
            inputs: call_fn.inputs,
        }
    }

    pub(crate) fn sniff_dart_signatures(dart_src: &str) -> Vec<AsyncFunctionSignature> {
        let mut functions = HashMap::<String, (Option<_>, Option<_>)>::new();
        for captures in SNIFF_FUNCTION_REGEX.captures_iter(dart_src) {
            let func = DartFunctionSignature::from_captures(&captures);
            if let Some(name) = func.name.strip_prefix("async_bindgen_dart_c__") {
                let slots = functions.entry(name.into()).or_default();
                //FIXME proper error message
                assert!(slots.0.is_none());
                slots.0 = Some(func);
            } else if let Some(name) = func.name.strip_prefix("async_bindgen_dart_r__") {
                let slots = functions.entry(name.into()).or_default();
                //FIXME proper error message
                assert!(slots.1.is_none());
                slots.1 = Some(func);
            }
        }

        functions
            .into_iter()
            .map(|(name, (call_fn, ret_fn))| {
                //FIXME better error
                let call_fn = call_fn.expect("Part of async glue missing: call fn.");
                let ret_fn = ret_fn.expect("Part of async glue missing: ret fn.");
                AsyncFunctionSignature::from_call_and_ret_func(name, call_fn, ret_fn)
            })
            .collect()
    }
}

#[derive(Serialize)]
pub(crate) struct DartFunctionInputs {
    pub(crate) name: String,
    pub(crate) r#type: String,
}

pub(crate) struct DartFunctionSignature {
    pub(crate) doc: Vec<String>,
    pub(crate) name: String,
    pub(crate) output: String,
    pub(crate) inputs: Vec<DartFunctionInputs>,
}

impl DartFunctionSignature {
    fn from_captures(captures: &Captures) -> Self {
        //UNWRAP_SAFE: capture group is not optional
        let name = captures.name("func_name").unwrap().as_str().trim().into();
        let doc = get_doc_from_captures(&captures);
        //UNWRAP_SAFE: capture group is not optional
        let output = captures.name("output").unwrap().as_str().trim().to_owned();
        let inputs = get_inputs_from_captures(captures);
        DartFunctionSignature {
            doc,
            name,
            output,
            inputs,
        }
    }
}

fn get_doc_from_captures(captures: &Captures) -> Vec<String> {
    captures_as_trimmed_lines(captures, "doc")
        .map(ToOwned::to_owned)
        .collect()
}

fn get_inputs_from_captures(captures: &Captures) -> Vec<DartFunctionInputs> {
    captures_as_trimmed_lines(captures, "inputs")
        .flat_map(|line| {
            let line = line.trim_end_matches(',');
            let (r#type, name) = line.rsplit_once(' ')?;
            Some(DartFunctionInputs {
                name: name.to_owned(),
                r#type: r#type.to_owned(),
            })
        })
        .collect()
}

fn captures_as_trimmed_lines<'a>(
    captures: &'a Captures,
    name: &'_ str,
) -> impl Iterator<Item = &'a str> {
    captures
        .name(name)
        .map(|cap| cap.as_str())
        .unwrap_or("")
        .lines()
        .flat_map(|line| {
            let line = line.trim();
            (!line.is_empty()).then(|| line)
        })
}

static SNIFF_FUNCTION_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?x)
        (?:\n|^)
        (?P<doc>(?:\s*///(?:\s.*)?\n)*)
        \s*(?P<output>[a-zA-Z0-9_<>.]+)\s(?P<func_name>[[:word:]]+)\(\n
            (?P<inputs>(?:\s+[a-zA-Z0-9_<>.]+\s[[:word:]]+,\n)*)
        \s*\)\s\{\n
    ",
    )
    .unwrap()
});

#[cfg(test)]
mod tests {
    use regex::Captures;

    use super::*;

    static TEST_DART_SRC: &str = r#"/// Foobar
        ///
        /// # Errors
        ///
        /// The foo errors
        /// - With the bar
        int async_bindgen_dart_c__function_1_magic(
            ffi.Pointer<CFoo> foo,
            ffi.Pointer<CBar> bar,
        ) {
            return _async_bindgen_dart_c__function_1_magic(
                foo,
                bar,
            );
        }

        ffi.Pointer<CustomCType> async_bindgen_dart_r__function_1_magic(
            int handle,
        ) {
            return _async_bindgen_dart_r__function_1_magic(
                handle,
            );
        }

        /// Serializes
        ///
        /// # Safety
        ///
        /// The behavior is undefined if:
        /// - I'm a dog.
        /// - I'm not a dog.
        int async_bindgen_dart_c__function_2_magic(
            double a,
        ) {
            return _async_bindgen_dart_w__function_2_magic(
                a,
            );
        }

        int async_bindgen_dart_r__function_2_magic(
            int handle,
        ) {
            return _async_bindgen_dart_r__function_2_magic(
                handle,
            );
        }
    "#;

    #[test]
    fn test_sniffing() {
        let mut sigs = AsyncFunctionSignature::sniff_dart_signatures(TEST_DART_SRC);
        assert_eq!(sigs.len(), 2);

        sigs.sort_by(|f1, f2| f1.name.cmp(&f2.name));

        let f1m = &sigs[0];
        assert_eq!(&f1m.name, "function_1_magic");
        assert_eq!(&f1m.ffi_call_name, "async_bindgen_dart_c__function_1_magic");
        assert_eq!(&f1m.ffi_ret_name, "async_bindgen_dart_r__function_1_magic");
        assert_eq!(&f1m.output, "ffi.Pointer<CustomCType>");

        assert_eq!(&f1m.inputs[0].name, "foo");
        assert_eq!(&f1m.inputs[0].r#type, "ffi.Pointer<CFoo>");
        assert_eq!(&f1m.inputs[1].name, "bar");
        assert_eq!(&f1m.inputs[1].r#type, "ffi.Pointer<CBar>");
        assert_eq!(f1m.inputs.len(), 2);

        assert_eq!(&f1m.doc[0], "/// Foobar");
        assert_eq!(f1m.doc.len(), 6);

        let f2m = &sigs[1];
        assert_eq!(&f2m.name, "function_2_magic");
        assert_eq!(&f2m.ffi_call_name, "async_bindgen_dart_c__function_2_magic");
        assert_eq!(&f2m.ffi_ret_name, "async_bindgen_dart_r__function_2_magic");
        assert_eq!(&f2m.output, "int");

        assert_eq!(&f2m.inputs[0].name, "a");
        assert_eq!(&f2m.inputs[0].r#type, "double");
        assert_eq!(f2m.inputs.len(), 1);

        assert_eq!(&f2m.doc[0], "/// Serializes");
        assert_eq!(f2m.doc.len(), 7);
    }

    #[test]
    fn test_regex_matches_function_sig() {
        let captures = SNIFF_FUNCTION_REGEX.captures_iter(TEST_DART_SRC);

        let captures = captures.collect::<Vec<_>>();

        assert_eq!(captures.len(), 4);

        test_match(
            &captures[0],
            vec![
                "/// Foobar",
                "///",
                "/// # Errors",
                "///",
                "/// The foo errors",
                "/// - With the bar",
            ],
            "int",
            "async_bindgen_dart_c__function_1_magic",
            vec!["ffi.Pointer<CFoo> foo,", "ffi.Pointer<CBar> bar,"],
        );

        test_match(
            &captures[1],
            vec![],
            "ffi.Pointer<CustomCType>",
            "async_bindgen_dart_r__function_1_magic",
            vec!["int handle,"],
        );

        test_match(
            &captures[2],
            vec![
                "/// Serializes",
                "///",
                "/// # Safety",
                "///",
                "/// The behavior is undefined if:",
                "/// - I'm a dog.",
                "/// - I'm not a dog.",
            ],
            "int",
            "async_bindgen_dart_c__function_2_magic",
            vec!["double a,"],
        );

        test_match(
            &captures[3],
            vec![],
            "int",
            "async_bindgen_dart_r__function_2_magic",
            vec!["int handle,"],
        );

        fn test_match(
            captures: &Captures,
            doc_comments: Vec<&str>,
            output: &str,
            name: &str,
            inputs: Vec<&str>,
        ) {
            let found_doc_comments = captures_as_trimmed_lines(captures, "doc").collect::<Vec<_>>();
            assert_eq!(found_doc_comments, doc_comments);
            assert_eq!(
                captures.name("func_name").unwrap().as_str().trim(),
                name
            );
            assert_eq!(captures.name("output").unwrap().as_str().trim(), output);
            let found_args = captures_as_trimmed_lines(captures, "inputs").collect::<Vec<_>>();
            assert_eq!(found_args, inputs);
        }
    }
}
