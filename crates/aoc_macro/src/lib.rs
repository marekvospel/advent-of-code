use proc_macro2::TokenTree;
use quote::{format_ident, quote};

#[proc_macro]
pub fn test_day(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = proc_macro2::TokenStream::from(item);

    let mut test_only = false;

    let mut iter = item.into_iter().filter(|tt| match tt {
        TokenTree::Punct(punct) => punct.as_char() != ',',
        _ => true,
    });

    let year = match iter.next() {
        Some(TokenTree::Literal(lit)) => lit.to_string(),
        _ => panic!("tests! macro requires a year as its first argument"),
    };
    let day = match iter.next() {
        Some(TokenTree::Literal(lit)) => lit.to_string(),
        _ => panic!("tests! macro requires a day as its second argument"),
    };
    let year = year.trim_matches('"');
    let day = day.trim_matches('"');

    for item in iter {
        match item {
            TokenTree::Ident(ident) => {
                if ident.to_string() == "test_only" {
                    test_only = true;
                }
            }
            _ => {}
        }
    }

    let import_year = format_ident!("aoc{}", year);
    let import_day = format_ident!("day{}", day);
    let fn1_name = format_ident!("run_day{}_test", day);
    let day_name_test = format!("day{}-test.txt", day);

    let mut full = quote! {};

    if !test_only {
        let fn2_name = format_ident!("run_day{}", day);
        let day_name = format!("day{}.txt", day);

        full = quote! {
            #[test]
            fn #fn2_name() -> anyhow::Result<()> {
                use advent_of_code::#import_year::#import_day::AOCDay;
                let input = get_input(#year, #day_name)?;
                let result = AOCDay::run_pt1(input.clone())?;
                println!("Result: {}", result);
                let solution = get_solution(#year, #day_name)?;
                assert_eq!(result, solution.0, "Part 1 does not match solution");
                if let Some(solution) = solution.1 {
                    let result = AOCDay::run_pt2(input)?;
                    println!("Result: {}", result);
                    assert_eq!(result, solution, "Part 2 does not match solution");
                }
                Ok(())
            }
        };
    }

    quote! {
        #[test]
        fn #fn1_name() -> anyhow::Result<()> {
            use advent_of_code::#import_year::#import_day::AOCDay;
            let input = get_input(#year, #day_name_test)?;
            let result = AOCDay::run_pt1(input.clone())?;
            println!("Result: {}", result);
            let solution = get_solution(#year, #day_name_test)?;
            assert_eq!(result, solution.0, "Part 1 does not match solution");
            if let Some(solution) = solution.1 {
                let result = AOCDay::run_pt2(input)?;
                println!("Result: {}", result);
                assert_eq!(result, solution, "Part 2 does not match solution");
            }
            Ok(())
        }

        #full
    }
    .into()
}
