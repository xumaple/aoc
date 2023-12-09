#[macro_use]
extern crate lazy_static;
extern crate proc_macro;

mod syn_util;
use syn_util::*;

use quote::{quote, ToTokens};
use syn::*;

use std::collections::BTreeSet;
use std::fmt::Display;
use std::result::Result as StdResult;
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

#[proc_macro_attribute]
pub fn aoc_run(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // Check for input to be DDP (Day Day Part)
    fn check_attr(input_attr: String) -> StdResult<(Day, Part), BoxError> {
        let input_attr = input_attr.trim();
        if input_attr.len() != 3 {
            return Err(E::ParseError)?;
        }
        let day: Day = input_attr[..2].parse()?;
        let part: Part = input_attr[2..].parse()?;
        Ok((day, part))
    }

    fn check_item(input_fn: ItemFn) -> (Ident, ItemFn) {
        (input_fn.sig.ident.clone(), input_fn)
    }

    let (day, part) = match check_attr(attr.to_string()) {
        Ok(v) => v,
        Err(_) => {
            return compile_err(
                attr.to_string(),
                "Expected #[aoc_run(<DDP>)] \n    eg. #[aoc_run(01a)]",
            );
        }
    };
    let run = Run { day, part };
    let runner = quote::format_ident!("Runner{}", run.to_string());

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
            fn solve(&self, filename: impl AsRef<Path>) -> Result<u64, util::BoxError> {
                Ok(#fn_name(util::read(filename)?)?.uinton())
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
        quote!(
            util::aoc::Run { part: #part, day: #day },
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
        let part_ = run_key.part;
        let day_ = run_key.day;
        let day_mod_ = quote::format_ident!("d{}", run_key.day.num_repr());
        let part_mod_ = quote::format_ident!("{}", run_key.part.lower_repr());
        let runner = quote::format_ident!("Runner{}", run_key.to_string());
        match &filename {
            StrLitOrExpr::LitStr(filename_lit) => {
                let input_file_ = format!("src/d{}/{}", run_key.day.num_repr(), filename_lit.value());
                quote!(
                    (#day_, #part_) => {
                        crate::#day_mod_::#part_mod_::#runner {}.solve(#input_file_)
                    }
                )
            },
            StrLitOrExpr::Expr(filename_expr) => {
                let input_file_ = format!("src/d{}", run_key.day.num_repr());
                quote!(
                    (#day_, #part_) => {
                        crate::#day_mod_::#part_mod_::#runner {}.solve(format!("{}/{}", #input_file_, #filename_expr))
                    }
                )
            }
        }
    });

    quote!(
        match (#run_.day, #run_.part) {
            #(#run_arms)*
            _ => panic!("Unregistered run {}", #run_)
        }
    )
    .into()
}
