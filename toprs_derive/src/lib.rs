use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Edit)]
pub fn into_editor_derive(input: TokenStream) -> TokenStream {
    let _ast: DeriveInput = parse_macro_input!(input as DeriveInput);
    todo!()
}
