extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Position)]
pub fn derive_position(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics Position for #name #ty_generics #where_clause {
            type T = T;

            fn position(&self) -> &Self::T {
                &self.position
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Velocity, attributes(position))]
pub fn derive_velocity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics Position for #name #ty_generics #where_clause {
            type T = T;

            fn position(&self) -> &Self::T {
                &self.position
            }
        }

        impl #impl_generics Velocity for #name #ty_generics #where_clause {
            fn velocity(&self) -> &Self::T {
                &self.velocity
            }
        }
    };

    TokenStream::from(expanded)
}
