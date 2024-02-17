macro_rules! inc {
    ($expr:expr) => {{
        let expr = $expr;

        *expr += 1;

        expr
    }};
}

macro_rules! identify {
    ($expr:expr) => {{
        let expr = ::quote::format_ident!("_{}", $expr);

        ::quote::quote!(#expr)
    }};
}

macro_rules! tuplify {
   ($expr:expr) => {{
      let expr = $expr;

      ::quote::quote!((#(#expr),*))
   }};
}

pub(crate) use {identify, inc, tuplify};
