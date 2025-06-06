// pub trait Quotable {

// }

use std::collections::{BTreeMap, HashMap};

use quote::{quote, ToTokens, TokenStreamExt};
use proc_macro2::*;

struct MatchTokenRepr<K, V> {
    map: HashMap<K, V>,
    match_variable_name: String,
}

impl<K, V> MatchTokenRepr<K, V> {
    pub fn new(map: HashMap<K, V>, match_variable_name: String) -> Self {
        Self {
            map,
            match_variable_name
        }
    }
}

impl<K: ToTokens + Ord, V: ToTokens + Clone> ToTokens for MatchTokenRepr<K, V> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let var_name = &self.match_variable_name;


        let k: Vec<_> = self.map.keys().map(|k| quote! { #k }).collect();
        let v: Vec<_>  = self.map.values().map(|v| quote! { #v }).collect();
        
        tokens.extend(quote! {
            match #var_name {
                #( #k => println!("{}", #v), )*
            }
        });
        
    }
}