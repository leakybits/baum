//! `baump-proc` implements proc macros used by the `baum` crate.

#[macro_use]
extern crate proc_util;

mod idx;
mod macros;

use proc_macro::TokenStream;

/// Converts a (possibly nested) tuple of expressions into an (equally nested) tuple of unique identifiers.
#[proc_macro]
pub fn idx(args: TokenStream) -> TokenStream {
    render!(args, { idx::expand(args) })
}
