use proc_macro::TokenStream;

use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::__private::TokenStream2;
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Generics,
    Index,
};

mod html;

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    html::html(input)
}

#[proc_macro_derive(Edit)]
pub fn edit_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);
    impl_edit(ast).into()
}

fn impl_edit(ast: DeriveInput) -> TokenStream2 {
    match ast.data {
        Data::Struct(data_struct) => impl_edit_struct(ast.ident, data_struct, ast.generics),
        Data::Enum(_) => todo!("enums are not yet supported"),
        Data::Union(_) => panic!("unions are not supported"),
    }
}

fn impl_edit_struct(ident: Ident, data_struct: DataStruct, generics: Generics) -> TokenStream2 {
    match data_struct.fields {
        Fields::Named(fields) => impl_edit_named_struct(ident, fields, generics),
        Fields::Unnamed(fields) => impl_edit_unnamed_struct(ident, fields, generics),
        Fields::Unit => impl_edit_unit_struct(ident, generics),
    }
}

fn impl_edit_named_struct(ident: Ident, fields: FieldsNamed, generics: Generics) -> TokenStream2 {
    let editor_ident = format_ident!("{ident}Editor");
    let (field_idents, field_types): (Vec<_>, Vec<_>) = fields
        .named
        .iter()
        .map(|field| (&field.ident, &field.ty))
        .unzip();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let editor_struct = quote! {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct #editor_ident {
            #(#field_idents: <#field_types as Edit>::Editor),*
        }
    };

    let editor_impl = quote! {
        impl #impl_generics Editor for #editor_ident #ty_generics #where_clause {
            type Input = #ident;
            type Output = #ident;

            fn start(&mut self, value: Option<Self::Input>, gen: &mut Generator) {
                match value {
                    None => {
                        #(self.#field_idents.start(None, gen);)*
                    }
                    Some(value) => {
                        #(self.#field_idents.start(Some(value.#field_idents), gen);)*
                    }
                }
            }

            fn component(&self) -> Component {
                let children = vec![
                    #(self.#field_idents.component()),*
                ];

                Component::new(Id::INVALID, Widget::Group(children))
            }

            fn on_event(&mut self, event: Event, gen: &mut Generator) -> Option<Feedback> {
                #(if let Some(feedback) = self.#field_idents.on_event(event.clone(), gen) { return Some(feedback) })*

                None
            }

            fn read(&self) -> Result<Self::Output, EditorError> {
                let value = #ident {
                    #(#field_idents: self.#field_idents.read()?),*
                };

                Ok(value)
            }
        }
    };

    let edit_impl = quote! {
        impl Edit for #ident {
            type Editor = #editor_ident;

            fn default_editor() -> Self::Editor {
                #editor_ident {
                    #(#field_idents: <#field_types as Edit>::default_editor()),*
                }
            }
        }
    };

    quote! {
        #editor_struct
        #editor_impl
        #edit_impl
    }
}

fn impl_edit_unnamed_struct(
    ident: Ident,
    fields: FieldsUnnamed,
    generics: Generics,
) -> TokenStream2 {
    let editor_ident = format_ident!("{ident}Editor");
    let (field_indices, field_types): (Vec<_>, Vec<_>) = fields
        .unnamed
        .iter()
        .enumerate()
        .map(|(index, field)| (Index::from(index), &field.ty))
        .unzip();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let editor_struct = quote! {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct #editor_ident #impl_generics (#(<#field_types as Edit>::Editor),*) #where_clause;
    };

    let editor_impl = quote! {
        impl #impl_generics Editor for #editor_ident #ty_generics #where_clause {
            type Input = #ident;
            type Output = #ident;

            fn start(&mut self, value: Option<Self::Input>, gen: &mut Generator) {
                match value {
                    None => {
                        #(self.#field_indices.start(None, gen);)*
                    }
                    Some(value) => {
                        #(self.#field_indices.start(Some(value.#field_indices), gen);)*
                    }
                }
            }

            fn component(&self) -> Component {
                let children = vec![
                    #(self.#field_indices.component()),*
                ];

                Component::new(Id::INVALID, Widget::Group(children))
            }

            fn on_event(&mut self, event: Event, gen: &mut Generator) -> Option<Feedback> {
                #(if let Some(feedback) = self.#field_indices.on_event(event.clone(), gen) { return Some(feedback) })*

                None
            }

            fn read(&self) -> Result<Self::Output, EditorError> {
                let value = #ident(#(self.#field_indices.read()?),*);

                Ok(value)
            }
        }
    };

    let edit_impl = quote! {
        impl Edit for #ident {
            type Editor = #editor_ident;

            fn default_editor() -> Self::Editor {
                #editor_ident(#(<#field_types as Edit>::default_editor()),*)
            }
        }
    };

    quote! {
        #editor_struct
        #editor_impl
        #edit_impl
    }
}

fn impl_edit_unit_struct(ident: Ident, generics: Generics) -> TokenStream2 {
    let editor_ident = format_ident!("{ident}Editor");
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let editor_struct = quote! {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct #editor_ident;
    };

    let editor_impl = quote! {
        impl #impl_generics Editor for #editor_ident #ty_generics #where_clause {
            type Input = #ident;
            type Output = #ident;

            fn start(&mut self, value: Option<Self::Input>, gen: &mut Generator) {}

            fn component(&self) -> Component {
                Component::new(Id::INVALID, Widget::Group(Vec::new()))
            }

            fn on_event(&mut self, event: Event, gen: &mut Generator) -> Option<Feedback> {
                None
            }

            fn read(&self) -> Result<Self::Output, EditorError> {
                Ok(#ident)
            }
        }
    };

    let edit_impl = quote! {
        impl Edit for #ident {
            type Editor = #editor_ident;

            fn default_editor() -> Self::Editor {
                #editor_ident
            }
        }
    };

    quote! {
        #editor_struct
        #editor_impl
        #edit_impl
    }
}
