use std::collections::HashMap;

use proc_macro2::{TokenStream, Ident, Span};
use quote::quote;
use quote::ToTokens;


#[derive(Debug, Clone)]
pub struct MapRepr<K, V, I: IntoIterator<Item = (K, V)> + Clone> {
    map: I,
    match_variable_name: TokenStream,
    default: Option<TokenStream>,
}

impl<K, V, I: IntoIterator<Item = (K, V)> + Clone> MapRepr<K, V, I> {
    pub fn new(map: I, match_variable_name: TokenStream, default: Option<TokenStream>) -> Self {
        Self {
            map,
            match_variable_name,
            default,
        }
    }
}

impl<K: ToTokens, V: ToTokens, I: IntoIterator<Item = (K, V)> + Clone> ToTokens for MapRepr<K, V, I> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // let var_name = Ident::new(&self.match_variable_name, Span::call_site());
        let var_name = &self.match_variable_name;
        let (keys, values): (Vec<_>, Vec<_>) = self.map.clone().into_iter().unzip();
        let k: Vec<_> = keys.into_iter().map(|k| quote! { #k }).collect();
        let v: Vec<_>  = values.into_iter().map(|v| quote! { #v }).collect();
        
        if let Some(d) = &self.default {
            tokens.extend(quote! {
                match #var_name {
                    #( #k => #v,)*
                    _ => #d
                }
            });
        } else {
            tokens.extend(quote! {
                match #var_name {
                    #( #k => #v),*
                }
            });
        }
        
    }
}