use proc_macro2::Span;
use syn::{punctuated::Punctuated, Ident, Path, PathArguments, PathSegment, Type, TypePath};

/// Using a path prefix and a sting name create a type.
///
/// The name will be turned into an `Ident` and then
/// added to the `path_prefix` to create a [`Type::Path`]
/// variant.
pub fn type_from_path_and_name(path_prefix: Path, name: &str) -> Type {
    let mut path = path_prefix;
    path.segments.push(PathSegment {
        ident: Ident::new(name, Span::call_site()),
        arguments: PathArguments::None,
    });

    Type::Path(TypePath { qself: None, path })
}

pub fn type_from_name(name: &str) -> Type {
    let mut segments = Punctuated::new();
    segments.push(PathSegment {
        ident: Ident::new(name, Span::call_site()),
        arguments: PathArguments::None,
    });

    Type::Path(TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments,
        },
    })
}
