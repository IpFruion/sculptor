extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Type, Visibility};

fn map_visibility(vis: Visibility) -> impl ToTokens {
    match vis {
        Visibility::Public(_) => quote! {sculpture::modifier::Modifier::Public},
        Visibility::Inherited => quote! {sculpture::modifier::Modifier::Private},
        _ => quote! {sculpture::modifier::Modifier::None},
    }
}

fn map_field_type(field_type: Type) -> impl ToTokens {}

#[proc_macro_derive(Sculptable)]
pub fn derive_sculptable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let data = match input.data {
        syn::Data::Struct(d) => d,
        _ => panic!("Only available for structs (at the moment)"),
    };

    let vis = map_visibility(input.vis);

    let field_input = data
        .fields
        .into_iter()
        .map(|f| {
            let vis = map_visibility(f.vis);
            let name = match f.ident {
                None => panic!("Can't handle tuple structs yet"),
                Some(n) => n,
            };
            let field_type = map_field_type(f.ty);
            quote! {#vis, #name, #field_type}
        })
        .collect::<Vec<_>>();

    let ident = input.ident;
    let generics = input.generics;

    let ident_as_str = format!("{}", ident);

    let expanded = quote! {
        impl sculpture::Sculptable for #ident #generics {
            type Input = ();

            fn sculpt<S: sculpture::Sculptor>(sculptor: &mut S, _: Self::Input) -> Result<S::Ok, S::Error> {
                sculptor.start(#vis, #ident_as_str)?;
                #(sculptor.field(#field_input));*
                sculptor.end()
            }
        }
    };
    TokenStream::from(expanded)
}
