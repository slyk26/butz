use proc_macro::TokenStream;
use syn::DeriveInput;
use quote::quote;

#[proc_macro_derive(Model)]
pub fn model_derive_macro(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let ident = ast.ident;

    (quote! {
        impl Model for #ident {
            fn get_id(&self) -> &Option<Thing> {
                &self.id
            }

            fn set_id(&mut self, key: &str) {
                self.id = Some(Thing::from(split_key(key)));
            }
        }
    }).into()
}
