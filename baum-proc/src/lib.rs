//! `baump-proc` implements proc macros used by the `baum` crate.

#[macro_use]
extern crate proc_util;

/// Implements the `idx!` macro.
mod idx;

/// Implements crate-local helper macros.
mod macros;

/// Converts a (possibly nested) tuple of expressions into an (equally nested) tuple of unique identifiers.
#[proc_macro]
pub fn idx(args: proc_macro::TokenStream) -> proc_macro::TokenStream {
    render!(args, { idx::expand(&args) })
}
