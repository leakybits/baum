/// Return early if the error is `Abort`.
#[macro_export]
macro_rules! ok {
    ($expr:expr) => {
        match $expr {
            Ok(val) => Ok(val),
            Err($crate::Err::Retry(ctx)) => Err($crate::Err::Retry(ctx)),
            Err($crate::Err::Abort(ctx)) => Err($crate::Err::Abort(ctx))?,
        }
    };
}

/// Create a "retry" error with the given source and message.
#[macro_export]
macro_rules! retry {
    ($src:expr, $($tt:tt)*) => {
        $crate::Err::retry($src, &format!($($tt)*))
    };
}

/// Create an "abort" error with the given source and message.
#[macro_export]
macro_rules! abort {
    ($src:expr, $($tt:tt)*) => {
        $crate::Err::abort($src, &format!($($tt)*))
    };
}

/// Return the result of the first parser that succeeds.
#[macro_export]
macro_rules! alt {
    ($head:expr, $($tail:expr),+ $(,)?) => {{
        use $crate::*;

        $head.or(alt!($($tail),+))
    }};

    ($head:expr) => {$head};
}

/// Apply each parser in sequence, returning a tuple of all results.
#[macro_export]
macro_rules! seq {
    ($head:expr, $($tail:expr),+ $(,)?) => {{
        use $crate::*;

        let seq = $head.and(seq!($($tail),+));
        let map = |idx!(($head, ($($tail),+)))| idx!(($head, $($tail),+));

        seq.map(map)
    }};

    ($head:expr) => {$head};
}

/// Return the result of the first pattern that matches.
#[macro_export]
macro_rules! map {
    ($($pat:pat => $expr:expr),* $(,)?) => {{
        use $crate::*;

        alt!($(
            any.and_then(|src, item| {
                if let $pat = item {
                    Ok((src, $expr))
                } else {
                    Err(retry!(src, "failed to match pattern"))
                }
            })
        ),*)
    }};
}
