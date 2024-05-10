// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved


use proc_macro::TokenStream;
use quote::quote;

mod jscall;


#[proc_macro_attribute]
pub fn jscall(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    TokenStream::new()
}


#[proc_macro_derive(BytesLike)]
pub fn derive_bytes_like(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let (impls, types, wheres) = &input.generics.split_for_impl();
    let name = &input.ident;

    let tokens = quote! {
        impl #impls #name #types #wheres {
            pub fn is_empty(&self) -> bool { self.bytes.is_empty() }

            pub fn len(&self) -> usize { self.bytes.len() }

            pub fn as_bytes(&self) -> &[u8] { self.bytes.as_ref() }
        }
    };

    tokens.into()
}