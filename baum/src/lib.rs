//! `baum` is a parser combinator library.

extern crate baum_proc as proc;

mod error;
mod impls;
mod macros;
mod slice;

pub use self::error::*;
pub use self::proc::*;
pub use self::slice::*;

/// A parser: given source `S`, parses a value `T` and returns the remaining source.
pub trait Parse<S, T> {
    /// Parse the source and return the remaining source and the parsed value.
    fn parse(&self, src: S) -> Res<S, T>;

    /// Return a reference to the parser.
    fn as_ref(&self) -> Ref<Self> {
        Ref(self)
    }
}

/// A reference to a parser.
#[derive(Debug)]
pub struct Ref<'a, P: ?Sized>(&'a P);

/// Extension methods for `Parse<S, T>`.
pub trait ParseExt<S: Copy, T>: Parse<S, T> + Sized {
    fn and<U>(self, rhs: impl Parse<S, U>) -> impl Parse<S, (T, U)> {
        self.and_then(move |src, t| rhs.parse(src).map(|(src, u)| (src, (t, u))))
    }

    fn or(self, rhs: impl Parse<S, T>) -> impl Parse<S, T> {
        self.or_else(move |src| rhs.parse(src))
    }

    fn and_then<U>(self, f: impl Fn(S, T) -> Res<S, U>) -> impl Parse<S, U> {
        move |src: S| ok!(self.parse(src)).and_then(|(src, t)| f(src, t))
    }

    fn or_else(self, f: impl Fn(S) -> Res<S, T>) -> impl Parse<S, T> {
        move |src: S| ok!(self.parse(src)).or_else(|_| f(src))
    }

    fn opt(self) -> impl Parse<S, Option<T>> {
        self.map(Some).or_else(|src| Ok((src, None)))
    }

    fn rep(self) -> impl Parse<S, Vec<T>> {
        move |mut src: S| {
            let mut res = Vec::new();

            while let Ok(t) = ok!(self.parse_mut(&mut src)) {
                res.push(t);
            }

            Ok((src, res))
        }
    }

    fn sep<U>(self, sep: impl Parse<S, U>) -> impl Parse<S, Vec<T>> {
        move |src: S| {
            let (src, head) = ok!(self.parse(src))?;
            let (src, tail) = ok!(self.as_ref().pfx(sep.as_ref()).rep().parse(src))?;

            Ok((src, std::iter::once(head).chain(tail).collect()))
        }
    }

    fn pfx<U>(self, pfx: impl Parse<S, U>) -> impl Parse<S, T> {
        pfx.and(self).map(|(_, t)| t)
    }

    fn pfx_opt<U>(self, pfx: impl Parse<S, U>) -> impl Parse<S, T> {
        self.pfx(pfx.opt())
    }

    fn sfx<U>(self, sfx: impl Parse<S, U>) -> impl Parse<S, T> {
        self.and(sfx).map(|(t, _)| t)
    }

    fn sfx_opt<U>(self, sfx: impl Parse<S, U>) -> impl Parse<S, T> {
        self.sfx(sfx.opt())
    }

    fn del<U, V>(self, pfx: impl Parse<S, U>, sfx: impl Parse<S, V>) -> impl Parse<S, T> {
        self.pfx(pfx).sfx(sfx)
    }

    fn del_opt<U, V>(self, pfx: impl Parse<S, U>, sfx: impl Parse<S, V>) -> impl Parse<S, T> {
        self.pfx_opt(pfx).sfx_opt(sfx)
    }

    fn map<U>(self, f: impl Fn(T) -> U) -> impl Parse<S, U> {
        move |src: S| self.parse(src).map(|(src, t)| (src, f(t)))
    }

    fn iff(self, f: impl Fn(&T) -> bool) -> impl Parse<S, Option<T>> {
        self.map(move |t| f(&t).then(|| t))
    }
}

/// Extension methods for `Parse<S, Option<T>>`.
pub trait ParseOptExt<S: Copy, T>: Parse<S, Option<T>> + Sized {
    fn ok_or(self, err: Err<S>) -> impl Parse<S, T> {
        self.ok_or_else(move |_| err.clone())
    }

    fn ok_or_else(self, f: impl Fn(S) -> Err<S>) -> impl Parse<S, T> {
        self.and_then(move |src, t| Ok((src, t.ok_or_else(|| f(src))?)))
    }

    fn filter(self, f: impl Fn(&T) -> bool) -> impl Parse<S, Option<T>> {
        self.map(move |t| t.filter(|t| f(t)))
    }

    fn filter_map<U>(self, f: impl Fn(T) -> Option<U>) -> impl Parse<S, Option<U>> {
        self.map(move |t| t.and_then(|t| f(t)))
    }
}

/// Extension methods for `Parse<S, Result<T, E>>`.
pub trait ParseResExt<S: Copy, T, E>: Parse<S, Result<T, E>> + Sized {
    fn map_ok<U>(self, f: impl Fn(T) -> U) -> impl Parse<S, Result<U, E>> {
        self.map(move |res| res.map(&f))
    }

    fn map_err<U>(self, f: impl Fn(E) -> U) -> impl Parse<S, Result<T, U>> {
        self.map(move |res| res.map_err(&f))
    }
}

/// Parse the source in place.
pub trait ParseMut<S: Copy, T>: Parse<S, T> {
    fn parse_mut(&self, src: &mut S) -> Result<T, Err<S>> {
        let (rem, t) = self.parse(*src)?;

        *src = rem;

        Ok(t)
    }
}
