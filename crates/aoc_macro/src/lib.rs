use proc_macro2::Ident;
use quote::{format_ident, quote};
use test_day::TestDay;

pub(crate) mod test_day;

#[proc_macro]
pub fn test_day(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let TestDay {
        year,
        day,
        test_only,
    } = syn::parse_macro_input!(item as test_day::TestDay);

    let mut full = quote! {};

    if !test_only {
        let fn2_name = format_ident!("run_day{}", day);
        let day_name = format!("day{}.txt", day);

        full = create_test_fn(fn2_name, year, day.clone(), day_name)
    }

    let fn1_name = format_ident!("run_day{}_test", day);
    let day_name_test = format!("day{}-test.txt", day);
    let test_fn = create_test_fn(fn1_name, year, day, day_name_test);

    quote! {
        #test_fn

        #full
    }
    .into()
}

fn create_test_fn(
    fn_name: Ident,
    year: u32,
    day: String,
    day_name_test: String,
) -> proc_macro2::TokenStream {
    let import_year = format_ident!("aoc{}", year);
    let import_day = format_ident!("day{}", day);

    quote! {
        #[test]
        fn #fn_name() -> anyhow::Result<()> {
            use advent_of_code::#import_year::#import_day::AOCDay;
            let input = get_input(#year.to_string(), #day_name_test.to_string())?;
            let result = AOCDay::run_pt1(input.clone())?;
            println!("Result: {}", result);
            let solution = get_solution(#year.to_string(), #day_name_test.to_string())?;
            assert_eq!(result, solution.0, "Part 1 does not match solution");
            if let Some(solution) = solution.1 {
                let result = AOCDay::run_pt2(input)?;
                println!("Result: {}", result);
                assert_eq!(result, solution, "Part 2 does not match solution");
            }
            Ok(())
        }
    }
}
