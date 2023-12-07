#[macro_use]
extern crate lazy_static;
extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, Parser};
use syn::punctuated::Pair;
use syn::*;

use std::collections::BTreeMap;
use std::sync::Mutex;

use util::aoc::*;

lazy_static! {
    static ref TRANSFORMER_IDENTS: Mutex<BTreeMap<Run, String>> = Mutex::new(Default::default());
}
