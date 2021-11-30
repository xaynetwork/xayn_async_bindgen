macro_rules! __assert_rust_code_eq {
    ($left:expr, $right:expr) => ({
        let left = $left;
        let right = $right;

        let left_syn: syn::File = syn::parse_str(left.as_ref()).expect("parsing left failed");
        let right_syn: syn::File = syn::parse_str(right.as_ref()).expect("parsing right failed");

        if left_syn != right_syn {
            panic!("Code is not AST equal.\nLEFT: {}\nRIGHT: {}", left, right);
        }
    });
}

pub(crate) use __assert_rust_code_eq as assert_rust_code_eq;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_equal_ignores_formatting() {
        assert_rust_code_eq!("fn a(x: u32) {}", "fn a(\n\tx : u32\n) {}");
    }
}