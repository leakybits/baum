macro_rules! inc {
    ($expr:expr) => {{
        let expr = $expr;

        *expr += 1;

        expr
    }};
}

macro_rules! tuplify {
   ($expr:expr) => {{
      let expr = $expr;

      ::quote::quote!((#(#expr),*))
   }};
}

pub(crate) use {inc, tuplify};
