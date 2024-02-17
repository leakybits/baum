use std::result::Result;

/// A parser result type, returning either the parsed value and the remaining source or an error.
pub type Res<S, T> = Result<(S, T), Err<S>>;

/// A parser error type that may be retryable or fatal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Err<S> {
    /// A retryable error: another parser may succeed with the same source.
    Retry(Ctx<S>),

    /// A fatal error: parsing should be aborted.
    Abort(Ctx<S>),
}

impl<S> Err<S> {
    /// Create a new "retry" error.
    pub fn retry(src: S, msg: impl ToString) -> Self {
        Err::Retry(Ctx::new(src, msg))
    }

    /// Create a new "abort" error.
    pub fn abort(src: S, msg: impl ToString) -> Self {
        Err::Abort(Ctx::new(src, msg))
    }

    /// Map the error context.
    fn map_ctx(self, f: impl FnOnce(Ctx<S>) -> Ctx<S>) -> Self {
        match self {
            Err::Retry(ctx) => Err::Retry(f(ctx)),
            Err::Abort(ctx) => Err::Abort(f(ctx)),
        }
    }
}

/// A helper trait for adding context to types.
pub trait WithCtx<S> {
    /// Attach context to the value.
    fn with_ctx(self, src: S, msg: impl ToString) -> Self;
}

impl<S> WithCtx<S> for Err<S> {
    fn with_ctx(self, src: S, msg: impl ToString) -> Self {
        self.map_ctx(|ctx| ctx.wrap(src, msg))
    }
}

impl<S, T> WithCtx<S> for Res<S, T> {
    fn with_ctx(self, src: S, msg: impl ToString) -> Self {
        self.map_err(|e| e.with_ctx(src, msg))
    }
}

/// Parser error context, holding the source, message, and possible child context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ctx<S> {
    src: S,
    msg: String,
    ctx: Option<Box<Ctx<S>>>,
}

impl<S> Ctx<S> {
    /// Return the source at which the error occurred.
    pub fn src(&self) -> &S {
        &self.src
    }

    /// Return the error message.
    pub fn msg(&self) -> &str {
        &self.msg
    }

    /// Return the child context, if any.
    pub fn ctx(&self) -> Option<&Ctx<S>> {
        self.ctx.as_deref()
    }

    fn new(src: S, msg: impl ToString) -> Self {
        Ctx {
            src,
            msg: msg.to_string(),
            ctx: None,
        }
    }

    fn wrap(self, src: S, msg: impl ToString) -> Self {
        Ctx {
            src,
            msg: msg.to_string(),
            ctx: Some(Box::new(self)),
        }
    }
}
