use proc_macro2::{TokenStream};
use quote::{quote, ToTokens};
use syn::{Data, DataStruct, Field, Fields, parse_macro_input, Token};
use syn::DeriveInput;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

#[proc_macro_derive(ModelAttribute)]
pub fn derive_builder(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput );
    let result = do_expand(&derive_input);
    match result {
        Ok(token_stream) => { token_stream.into_token_stream().into() }
        Err(e) => { e.into_compile_error().into() }
    }
}

fn generate_builder_struct_fields_def(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let fiedls = get_fields_from_derive_input(&st);

    let idents = fiedls.unwrap().iter().map(|f| &f.ident).collect::<Vec<_>>();
    todo!()
}


fn get_fields(st: DeriveInput) -> Vec<String> {
    let fields = get_fields_from_derive_input(&st);
    fields.unwrap().iter().map(| f| f.ident.to_owned().unwrap().to_string()).collect::<Vec<_>>()
}


fn do_expand(st: &DeriveInput) -> syn::Result<TokenStream> {
    eprintln!("{:#?}", &st);

    let struct_name = st.ident.to_string();
    let builder_name_literal = format!("{}Builder", struct_name);

    let builder_name_ident = syn::Ident::new(&*builder_name_literal, st.span());

    let struct_ident = &st.ident;

    let ret = quote!(
        pub struct #builder_name_ident {

        }
    );
    Ok(ret)
}


fn get_fields_from_derive_input(st: &DeriveInput) -> syn::Result<Punctuated<Field, Token![,]>> {
    if let Data::Struct(DataStruct {
                            fields: Fields::Named(syn::FieldsNamed { named, .. }), ..
                        }) = &st.data {
        return Ok(named.to_owned());
    }
    Err(syn::Error::new_spanned(st, "Must defin on struct ,not on enum".to_string()))
}






































































