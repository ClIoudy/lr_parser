use common::{Id, NonTerminal, Variant};

use crate::grammar::Grammar;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub fn enums(grammar: &Grammar) -> TokenStream {
    let all_enums = grammar.all_rules().into_iter().map(|(symbol, variants)| build_enum(symbol, variants));

    quote! {
        #(#all_enums)*
    }
}

fn build_enum(symbol: &NonTerminal, variants: &Vec<Variant>) -> TokenStream {

    let symbol = Ident::new(&symbol.x, Span::call_site());
    let variants = variants.iter().map(|x| build_variant(x)).collect::<Vec<_>>();

    quote! {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum #symbol {
            #(#variants),*
        }
    }
}

fn build_variant(variant: &Variant) -> TokenStream {
    // variant.values()
    // #name(value)
    // where value := String | Box<SymbolOfId> depending on value type

    let name = Ident::new(variant.name(), Span::call_site());

    let values = variant.values().into_iter().map(|x| {
        match x {
            Id::N(n) => {
                let symbol = Ident::new(&n.x, Span::call_site());

                quote! {
                    Box<#symbol>
                }
            },
            Id::T(t) => quote! { Box<String> },
        }
    }).collect::<Vec<_>>();

    quote! { #name( #(#values,)* ) }

}