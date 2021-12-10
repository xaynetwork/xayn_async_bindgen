use heck::SnakeCase;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    token::Comma,
    Error, Lit, MetaNameValue, Path, PathArguments, Token,
};

pub(crate) struct ApiMeta {
    // in the future we likely will have more options,
// like e.g. spawner
}

impl Parse for ApiMeta {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let mut list = parse_meta_attr_nv_list(input)?;
        if !input.is_empty() {
            return Err(input.error("only a list of `<ident> = <lit>` mappings is supported"));
        }
        ApiMeta::try_from(list)
    }
}

type MetaNVList = Punctuated<MetaNameValue, Comma>;

fn parse_meta_attr_nv_list(input: ParseStream) -> Result<MetaNVList, Error> {
    let mut list = Punctuated::new();
    while !input.is_empty() {
        let nv = input.parse::<MetaNameValue>()?;
        list.push_value(nv);
        if !input.peek(Token![,]) {
            break;
        }
        list.push_punct(input.parse()?);
    }
    Ok(list)
}

impl TryFrom<MetaNVList> for ApiMeta {
    type Error = Error;

    fn try_from(nv_list: MetaNVList) -> Result<Self, Self::Error> {
        // let mut name_slot = None;
        for nv in nv_list {
            // if expect_single_seg_path(&nv.path)? == "name" {
            //     let name = expect_string_lit(&nv.lit)?;
            //     name_slot = Some(name.to_snake_case());
            // } else {
            return Err(Error::new(
                nv.span(),
                "unsupported async bindgen API option",
            ));
            // }
        }
        Ok(Self {
            // name: name_slot.unwrap_or_else(|| "AsyncBindings".into()),
        })
    }
}

// fn expect_single_seg_path(path: &Path) -> Result<String, Error> {
//     if path.segments.len() == 1 {
//         let seg = path.segments.first().unwrap();
//         if let PathArguments::None = &seg.arguments {
//             return Ok(seg.ident.to_string());
//         }
//     }
//     return Err(Error::new(path.span(), "expected single ident path"));
// }

// fn expect_string_lit(lit: &Lit) -> Result<String, Error> {
//     if let Lit::Str(s) = lit {
//         Ok(s.value())
//     } else {
//         Err(Error::new(lit.span(), "expected a string literal"))
//     }
// }
