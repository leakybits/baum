use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Expr;

use crate::macros::{inc, tuplify};

pub fn expand(expr: &Expr) -> TokenStream {
    expand_idx(&mut 0, expr)
}

fn expand_idx(idx: &mut usize, expr: &Expr) -> TokenStream {
    if let Expr::Tuple(expr) = expr {
        tuplify!(expr.elems.iter().map(|expr| expand_idx(idx, expr)))
    } else {
        identify!(inc!(idx)).into_token_stream()
    }
}
