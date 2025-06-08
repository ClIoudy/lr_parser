use std::collections::HashMap;

use proc_macro2::{TokenStream, Ident, Span};
use quote::quote;
use quote::ToTokens;


#[derive(Debug, Clone)]
pub struct MapRepr<K, V> {
    map: HashMap<K, V>,
    match_variable_name: String,
}

impl<K, V> MapRepr<K, V> {
    pub fn new(map: HashMap<K, V>, match_variable_name: String) -> Self {
        Self {
            map,
            match_variable_name
        }
    }
}

impl<K: ToTokens, V: ToTokens + Clone> ToTokens for MapRepr<K, V> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let var_name = Ident::new(&self.match_variable_name, Span::call_site());


        let k: Vec<_> = self.map.keys().map(|k| quote! { #k }).collect();
        let v: Vec<_>  = self.map.values().map(|v| quote! { #v }).collect();
        
        tokens.extend(quote! {
            match #var_name {
                #( #k => #v),*
            }
        });
        
    }
}