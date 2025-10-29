use std::collections::HashMap;

use common::{NonTerminal, Variant, VariantId};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use super::super::reprenstations::MapRepr;


pub fn build_rule_fn(rules: &HashMap<NonTerminal, Vec<Variant>>) -> TokenStream {
    // match symbol
    //  match name
    //   to building variant from given children-values

    let r = rules.into_iter().map(|(symbol, variants)| 
        (
            &symbol.x, 
            MapRepr::new(
                variants.into_iter().map(|v| (v.name(), quote_return_variant(v.id()))).collect::<Vec<_>>(),
                quote! { name.as_str() },
                Some( quote! { None } )
            )
        )).collect::<Vec<_>>();

    let repr = MapRepr::new(r, quote! { symbol.as_str() }, Some( quote! {None} ));

    quote! {
        let symbol = &variant.symbol().x;
        let name = variant.name();
        #repr
    }

}

fn quote_return_variant(variant_id: &VariantId) -> TokenStream {
    let symbol = Ident::new(&variant_id.symbol().x, Span::call_site());
    let name = Ident::new(variant_id.name(), Span::call_site());
    
    let pop = quote! { children.pop()?.downcast().ok()? };
    let v = vec![pop; variant_id.length()];
    
    quote! { Some(Box::new(#symbol::#name(#(#v),*))) }
}