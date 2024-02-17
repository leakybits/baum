use super::*;

impl<S: Copy, T, F: Fn(S) -> Res<S, T>> Parse<S, T> for F {
    fn parse(&self, src: S) -> Res<S, T> {
        self(src)
    }
}

impl<S: Copy, T, P: Parse<S, T>> ParseExt<S, T> for P {
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
        move |_: S| todo!()
    }

    fn pfx<U>(self, pfx: impl Parse<S, U>) -> impl Parse<S, T> {
        pfx.and(self).map(|(_, t)| t)
    }

    fn sfx<U>(self, sfx: impl Parse<S, U>) -> impl Parse<S, T> {
        self.and(sfx).map(|(t, _)| t)
    }

    fn map<U>(self, f: impl Fn(T) -> U) -> impl Parse<S, U> {
        move |src: S| self.parse(src).map(|(src, t)| (src, f(t)))
    }

    fn iff(self, f: impl Fn(&T) -> bool) -> impl Parse<S, T> {
        self.map(move |t| f(&t).then(|| t)).ok()
    }
}

impl<S: Copy, T, P: Parse<S, Option<T>>> ParseOpt<S, T> for P {
    fn ok(self) -> impl Parse<S, T> {
        self.and_then(|src, t| match t {
            Some(t) => Ok((src, t)),
            None => Err(retry!(src, "expected Some, found None")),
        })
    }

    fn ok_or(self, err: Err<S>) -> impl Parse<S, T> {
        self.ok_or_else(move |_| err.clone())
    }

    fn ok_or_else(self, f: impl Fn(S) -> Err<S>) -> impl Parse<S, T> {
        self.and_then(move |src, t| match t {
            Some(t) => Ok((src, t)),
            None => Err(f(src)),
        })
    }

    fn filter(self, f: impl Fn(&T) -> bool) -> impl Parse<S, Option<T>> {
        self.map(move |t| t.filter(|t| f(t)))
    }
}

impl<S: Copy, T, P: Parse<S, T>> IntoFn<S, T> for P {
    fn as_fn(&self) -> impl Fn(S) -> Res<S, T> {
        move |src| self.parse(src)
    }

    fn into_fn(self) -> impl Fn(S) -> Res<S, T> {
        move |src| self.parse(src)
    }
}

macro_rules! ok {
    ($expr:expr) => {
        match $expr {
            Ok(val) => Ok(val),
            Err($crate::Err::Retry(ctx)) => Err($crate::Err::Retry(ctx)),
            Err($crate::Err::Abort(ctx)) => return Err($crate::Err::Abort(ctx)),
        }
    };
}

use ok;
