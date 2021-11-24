//! Ad-hoc parses a genesis.dart file for the type definitions produced by ffigen.
//!

use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::{Captures, Regex};

pub(crate) struct DartFunctionSignature {
    doc: Vec<String>,
    return_type: String,
    arguments: Vec<DartFunctionInputs>,
}

impl DartFunctionSignature {

    pub(crate) fn doc(&self) -> &[String] {
        &self.doc
    }

    pub(crate) fn return_type(&self) -> &str {
        &self.return_type
    }

    pub(crate) fn inputs(&self) -> &[DartFunctionInputs] {
        &self.arguments
    }
}

pub(crate) struct DartFunctionInputs {
    name: String,
    r#type: String,
}

impl DartFunctionInputs {

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn r#type(&self) -> &str {
        &self.r#type
    }
}

impl DartFunctionSignature {
    pub(crate) fn sniff_dart_signatures(
        dart_src: &str,
        mut name_filter_predicate: impl FnMut(&str) -> bool,
    ) -> HashMap<String, DartFunctionSignature> {
        SNIFF_FUNCTION_REGEX
            .captures_iter(dart_src)
            .flat_map(|captures| {
                //UNWRAP_SAFE: capture group is not optional
                let name = captures.name("name").unwrap().as_str().trim();
                name_filter_predicate(name).then(|| {
                    (
                        name.to_owned(),
                        DartFunctionSignature::from_captures(captures),
                    )
                })
            })
            .collect()
    }

    fn from_captures(captures: Captures) -> Self {
        let doc = get_doc_from_captures(&captures);
        //UNWRAP_SAFE: capture group is not optional
        let rtype = captures.name("rtype").unwrap().as_str().trim().to_owned();
        let arguments = get_arguments_from_captures(&captures);

        DartFunctionSignature {
            doc,
            return_type: rtype,
            arguments,
        }
    }
}

fn get_doc_from_captures(captures: &Captures) -> Vec<String> {
    captures_as_trimmed_lines(captures, "doc")
        .map(ToOwned::to_owned)
        .collect()
}

fn get_arguments_from_captures(captures: &Captures) -> Vec<DartFunctionInputs> {
    captures_as_trimmed_lines(captures, "args")
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
        \s*(?P<rtype>[a-zA-Z0-9_<>.]+)\s(?P<name>[[:word:]]+)\(\n
            (?P<args>(?:\s+[a-zA-Z0-9_<>.]+\s[[:word:]]+,\n)*)
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
        ffi.Pointer<CustomCType> function_1_magic(
            ffi.Pointer<CFoo> foo,
            ffi.Pointer<CBar> bar,
        ) {
            return _function_1_magic(
                foo,
                bar,
            );
        }

        /// Serializes
        ///
        /// # Safety
        ///
        /// The behavior is undefined if:
        /// - I'm a dog.
        /// - I'm not a dog.
        int function_2_magic(
            double a,
        ) {
            return _function_1_magic(
                a,
            );
        }
    "#;

    #[test]
    fn test_sniffing() {
        let sigs = DartFunctionSignature::sniff_dart_signatures(TEST_DART_SRC, |_| true);
        assert!(sigs.contains_key("function_1_magic"));
        assert!(sigs.contains_key("function_2_magic"));
        assert_eq!(sigs.len(), 2);

        let f1m = &sigs["function_1_magic"];
        assert_eq!(&f1m.return_type, "ffi.Pointer<CustomCType>");

        assert_eq!(&f1m.arguments[0].name, "foo");
        assert_eq!(&f1m.arguments[0].r#type, "ffi.Pointer<CFoo>");
        assert_eq!(&f1m.arguments[1].name, "bar");
        assert_eq!(&f1m.arguments[1].r#type, "ffi.Pointer<CBar>");
        assert_eq!(f1m.arguments.len(), 2);

        assert_eq!(&f1m.doc[0], "/// Foobar");
        assert_eq!(f1m.doc.len(), 6);

        let f2m = &sigs["function_2_magic"];
        assert_eq!(&f2m.return_type, "int");

        assert_eq!(&f2m.arguments[0].name, "a");
        assert_eq!(&f2m.arguments[0].r#type, "double");
        assert_eq!(f2m.arguments.len(), 1);

        assert_eq!(&f2m.doc[0], "/// Serializes");
        assert_eq!(f2m.doc.len(), 7);
    }

    #[test]
    fn test_regex_matches_function_sig() {
        let captures = SNIFF_FUNCTION_REGEX.captures_iter(TEST_DART_SRC);

        let captures = captures.collect::<Vec<_>>();

        assert_eq!(captures.len(), 2);

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
            "ffi.Pointer<CustomCType>",
            "function_1_magic",
            vec!["ffi.Pointer<CFoo> foo,", "ffi.Pointer<CBar> bar,"],
        );

        test_match(
            &captures[1],
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
            "function_2_magic",
            vec!["double a,"],
        );

        fn test_match(
            captures: &Captures,
            doc_comments: Vec<&str>,
            rtype: &str,
            name: &str,
            args: Vec<&str>,
        ) {
            let found_doc_comments = captures_as_trimmed_lines(captures, "doc").collect::<Vec<_>>();
            assert_eq!(found_doc_comments, doc_comments);
            assert_eq!(captures.name("name").unwrap().as_str().trim(), name);
            assert_eq!(captures.name("rtype").unwrap().as_str().trim(), rtype);
            let found_args = captures_as_trimmed_lines(captures, "args").collect::<Vec<_>>();
            assert_eq!(found_args, args);
        }
    }
}
