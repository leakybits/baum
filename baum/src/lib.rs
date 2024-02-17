//! `baum` is a parser combinator library.

mod error;
mod impls;
mod macros;
mod slice;

pub use self::error::*;
pub use self::slice::*;

/// A parser: given source `S`, parses a value `T` and returns the remaining source.
pub trait Parse<S: Copy, T> {
    /// Parse a value `T` from source `S`.
    fn parse(&self, src: S) -> Res<S, T>;
}

/// Extension methods for `Parse<S, T>`.
pub trait ParseExt<S: Copy, T> {
    /// If `self` succeeds, apply `rhs`, returning both results.
    fn and<U>(self, rhs: impl Parse<S, U>) -> impl Parse<S, (T, U)>;

    /// If `self` fails, apply `rhs`.
    fn or(self, rhs: impl Parse<S, T>) -> impl Parse<S, T>;

    /// If `self` succeeds, call `f` with the remaining source and parsed value.
    fn and_then<U>(self, f: impl Fn(S, T) -> Res<S, U>) -> impl Parse<S, U>;

    /// If `self` fails, call `f` with the remaining source.
    fn or_else(self, f: impl Fn(S) -> Res<S, T>) -> impl Parse<S, T>;

    /// Optionally apply `self`, returning `None` if it fails.
    fn opt(self) -> impl Parse<S, Option<T>>;

    /// Repeatedly apply `self` until it fails, returning a vector of results.
    fn rep(self) -> impl Parse<S, Vec<T>>;

    /// Parse and discard a prefix using `pfx`, then apply `self`.
    fn pfx<U>(self, pfx: impl Parse<S, U>) -> impl Parse<S, T>;

    /// Apply `self`, then parse and discard a suffix using `sfx`.
    fn sfx<U>(self, sfx: impl Parse<S, U>) -> impl Parse<S, T>;

    /// Apply `self`, then map the parsed value using `f`.
    fn map<U>(self, f: impl Fn(T) -> U) -> impl Parse<S, U>;

    /// Apply `self`, validating the parsed value using `f`.
    fn iff(self, f: impl Fn(&T) -> bool) -> impl Parse<S, T>;
}

/// Extension methods for `Parse<S, Option<T>>`.
pub trait ParseOpt<S: Copy, T> {
    /// Apply `self`, returning an error if the result is `None`.
    fn ok(self) -> impl Parse<S, T>;

    /// Apply `self`, returning `err` if the result is `None`.
    fn ok_or(self, err: Err<S>) -> impl Parse<S, T>;

    /// Apply `self`, returning an error from `f` if the result is `None`.
    fn ok_or_else(self, f: impl Fn(S) -> Err<S>) -> impl Parse<S, T>;

    /// Apply `self`, returning `None` if the result is `None`.
    fn filter(self, f: impl Fn(&T) -> bool) -> impl Parse<S, Option<T>>;
}

/// Convert a parser into a function.
pub trait IntoFn<S: Copy, T> {
    /// Return `self` as a function.
    fn as_fn(&self) -> impl Fn(S) -> Res<S, T>;

    /// Convert `self` into a function.
    fn into_fn(self) -> impl Fn(S) -> Res<S, T>;
}
