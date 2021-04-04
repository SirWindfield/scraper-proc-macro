use absolution::{TokenStream, Literal, LitKind, TokenTree};
use quote::quote;
use scraper::Selector;

#[proc_macro]
pub fn selector(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let stream: TokenStream = input.into();
    let first_token = &stream.tokens[0];
    let s = if let TokenTree::Literal(Literal {
        kind: LitKind::Str(s),
        ..
    }) = &first_token {
        s
    } else {
        panic!("First and only argument has to be a string!");
    };

    let selector = Selector::parse(s);
    if selector.is_ok() {
        return quote!(
            ::scraper::Selector::parse(#s).unwrap()
        ).into();
    } else {
        panic!("Failed to parse CSS selector: {:?}", s);
    }
}
