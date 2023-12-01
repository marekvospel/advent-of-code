use syn::{parse::Parse, Ident, LitInt, LitStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestDay {
    pub year: u32,
    pub day: String,
    pub test_only: bool,
}

#[derive(Debug, Clone, Copy, Default)]
struct TestDayArgs {
    test_only: bool,
}

#[derive(Debug, Clone)]
enum TestDayArg {
    TestOnly,
}

impl Parse for TestDay {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let year = input.parse::<LitInt>()?;
        input.parse::<syn::Token![,]>()?;
        let day;

        if input.peek(syn::LitInt) {
            day = input.parse::<LitInt>()?.base10_parse::<u8>()?.to_string();
        } else if input.peek(Ident) {
            day = input.parse::<Ident>()?.to_string();
        } else {
            day = input.parse::<LitStr>()?.value();
        }

        let TestDayArgs { test_only } = input.parse::<TestDayArgs>()?;

        Ok(Self {
            year: year.base10_parse()?,
            day: day,
            test_only,
        })
    }
}

impl Parse for TestDayArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut args = Self::default();

        while input.peek(syn::Token![,]) {
            input.parse::<syn::Token![,]>()?;
            let arg = input.parse::<TestDayArg>()?;

            match arg {
                TestDayArg::TestOnly => args.test_only = true,
            }
        }

        Ok(args)
    }
}

impl Parse for TestDayArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Ident) {
            let ident = input.parse::<Ident>()?;

            match ident.to_string().as_str() {
                "test_only" => return Ok(Self::TestOnly),
                _ => {}
            }
        }

        Err(input.error("expected `test_only`"))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quote::quote;

    #[test]
    fn it_should_parse_test_day() {
        let input = quote! { 2023, 1 };
        let test_day = syn::parse2::<TestDay>(input).unwrap();

        assert_eq!(
            test_day,
            TestDay {
                year: 2023,
                day: "1".to_string(),
                test_only: false,
            }
        )
    }

    #[test]
    #[should_panic]
    fn it_should_fail_as_argument_missing() {
        let input = quote! { 2023 };
        let _test_day = syn::parse2::<TestDay>(input).unwrap();
    }

    #[test]
    #[should_panic]
    fn it_should_fail_as_dangling_comma() {
        let input = quote! { 2023, 1, };
        let _test_day = syn::parse2::<TestDay>(input).unwrap();
    }

    #[test]
    #[should_panic]
    fn it_should_fail_as_invalid_extra_arg() {
        let input = quote! { 2023, 1, avalididentifier_butinvalid_arg };
        let _test_day = syn::parse2::<TestDay>(input).unwrap();
    }

    #[test]
    fn it_should_parse_with_args() {
        let input = quote! { 2023, 1, test_only };
        let test_day = syn::parse2::<TestDay>(input).unwrap();

        assert_eq!(
            test_day,
            TestDay {
                year: 2023,
                day: "1".to_string(),
                test_only: true,
            }
        )
    }
}
