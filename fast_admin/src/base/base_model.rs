pub struct BaseModel {
    page_num: Option<i64>,
    page_size: Option<i64>,
    create_time: std::time::SystemTime,
    update_time: std::time::SystemTime,
}

// macro_rules! get_fields {
//     ($ident:ty {}) => {
//
//
//         impl $ident {
//             pub fn get_vec_fields() -> Vec<String> {
//                 let fields = get_fields_from_derive_input(&st);
//                 fields.unwrap().iter().map(| f| f.ident.to_owned().unwrap().to_string()).collect::<Vec<_>>()
//             }
//
//         }
//
//
//     };
// }
//
// #[proc_macro_attribute]
// fn get_fields_from_derive_input(st: &DeriveInput) -> syn::Result<Punctuated<Field, Token![,]>> {
//     if let Data::Struct(DataStruct {
//                             fields: Fields::Named(syn::FieldsNamed { named, .. }), ..
//                         }) = &st.data {
//         return Ok(named.to_owned());
//     }
//     Err(syn::Error::new_spanned(st, "Must defin on struct ,not on enum".to_string()))
// }
