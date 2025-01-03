#[macro_use]
extern crate lazy_static;
extern crate proc_macro;

mod syn_util;
use syn_util::*;

use quote::{quote, ToTokens};
use syn::*;

use std::collections::BTreeSet;
use std::fmt::Display;
use std::sync::Mutex;

use util::aoc::*;

lazy_static! {
    static ref REGISTERED_RUNS: Mutex<BTreeSet<Run>> = Mutex::new(Default::default());
}

fn compile_err<T, U>(tokens: T, message: U) -> proc_macro::TokenStream
where
    T: ToTokens,
    U: Display,
{
    Error::new_spanned(tokens, message)
        .to_compile_error()
        .into()
}

fn get_runner_ident(run: &Run) -> Ident {
    quote::format_ident!("Runner{}", run.alphanumeric_repr())
}

#[proc_macro_attribute]
pub fn aoc_run(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    fn check_item(input_fn: ItemFn) -> (Ident, ItemFn) {
        (input_fn.sig.ident.clone(), input_fn)
    }

    // Check for input to be standard run format YY-DDP (Year Year - Day Day Part)
    let run = match attr.to_string().trim().parse::<Run>() {
        Ok(run) => run,
        Err(_) => {
            return compile_err(
                attr.to_string(),
                "Expected #[aoc_run(<YY-DDP>)] \n    eg. #[aoc_run(23-01a)]",
            );
        }
    };
    let runner = get_runner_ident(&run);

    let item_clone = item.clone();
    let (fn_name, fn_definition) = check_item(parse_macro_input!(item_clone as ItemFn));

    let mut runs = REGISTERED_RUNS.lock().unwrap();
    if !runs.insert(run) {
        return compile_err(attr.to_string(), format!("found duplicate run `{}`", run));
    }

    quote!(
        #[derive(Clone, Debug)]
        pub struct #runner;
        impl util::aoc::Runner for #runner {
            fn solve(&self, filename: impl AsRef<Path>) -> Result<String, util::BoxError> {
                Ok(#fn_name(util::read(filename)?)?.to_string())
            }
        }

        #fn_definition
    )
    .into()
}

// Only meant to be run from root/main.rs level
#[proc_macro]
pub fn get_all_runs(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let runs = REGISTERED_RUNS.lock().unwrap();
    let all_runs = runs.iter().map(|run| {
        let part = run.part;
        let day = run.day;
        let year = run.year;
        quote!(
            util::aoc::Run { part: #part, day: #day, year: #year },
        )
    });

    quote!(vec![
        #(#all_runs)*
    ])
    .into()
}

struct RunArgs(syn::Expr, syn_util::StrLitOrExpr);
impl syn::parse::Parse for RunArgs {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let run = input.parse()?;
        input.parse::<Token![,]>()?;
        let filename = input.parse()?;
        input.parse::<Option<Token![,]>>()?;
        Ok(Self(run, filename))
    }
}

#[proc_macro]
pub fn run(run: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let RunArgs(run_, filename) = match syn::parse(run) {
        Ok(t) => t,
        Err(e) => {
            return compile_err(e.to_compile_error(), "Unable to parse get_runner input");
        }
    };

    let runs = REGISTERED_RUNS.lock().unwrap();
    let run_arms = runs.iter().map(|run_key| {
        let year_ = run_key.year;
        let day_ = run_key.day;
        let part_ = run_key.part;
        let year_mod_ = quote::format_ident!("y{}", run_key.year.num_repr());
        let day_mod_ = quote::format_ident!("d{}", run_key.day.num_repr());
        let part_mod_ = quote::format_ident!("{}", run_key.part.lower_repr());
        let runner = get_runner_ident(run_key);
        match &filename {
            StrLitOrExpr::LitStr(filename_lit) => {
                let input_file_ = format!("src/y{}/d{}/{}", run_key.year.num_repr(), run_key.day.num_repr(), filename_lit.value());
                quote!(
                    (#year_, #day_, #part_) => {
                        crate::#year_mod_::#day_mod_::#part_mod_::#runner {}.solve(#input_file_)
                    }
                )
            },
            StrLitOrExpr::Expr(filename_expr) => {
                let input_file_ = format!("src/y{}/d{}", run_key.year.num_repr(), run_key.day.num_repr());
                quote!(
                    (#year_, #day_, #part_) => {
                        crate::#year_mod_::#day_mod_::#part_mod_::#runner {}.solve(format!("{}/{}", #input_file_, #filename_expr))
                    }
                )
            }
        }
    });

    quote!(
        match (#run_.year, #run_.day, #run_.part) {
            #(#run_arms)*
            _ => panic!("Unregistered run {}", #run_)
        }
    )
    .into()
}
