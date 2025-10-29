use proc_macro2::TokenStream;
use quote::quote;

pub fn parser_struct_tokens() -> TokenStream {
    quote! {
        use lr_parser::ParserTrait;
        struct Parser;

        impl ParserTrait<Table> for Parser {}
    }
}