use proc_macro::*;
use quote::quote;
#[proc_macro_attribute]
pub fn command(_: TokenStream, _: TokenStream) -> TokenStream {
    quote!(
        fn a() {}
    )
    .into()
}
