use crate::{Parse, ParseCtx, ParseExt, ParseMut, ParseOptExt, ParseResExt, Ref, Res};

impl<S, T, F: Fn(S) -> Res<S, T>> Parse<S, T> for F {
    fn parse(&self, src: S) -> Res<S, T> {
        self(src)
    }
}

impl<S, T, P: Parse<S, T>> Parse<S, T> for Ref<'_, P> {
    fn parse(&self, src: S) -> Res<S, T> {
        self.0.parse(src)
    }
}

impl<S: Copy, T, P: Parse<S, T>> ParseMut<S, T> for P {}

impl<S: Copy, T, P: Parse<S, T>> ParseCtx<S, T> for P {}

impl<S: Copy, T, P: Parse<S, T>> ParseExt<S, T> for P {}

impl<S: Copy, T, P: Parse<S, Option<T>>> ParseOptExt<S, T> for P {}

impl<S: Copy, T, E, P: Parse<S, Result<T, E>>> ParseResExt<S, T, E> for P {}
