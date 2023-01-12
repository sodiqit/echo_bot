use proc_macro::{Ident, TokenStream};
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Data, DataStruct, DeriveInput, Field, Fields};

#[proc_macro_derive(Validate)]
#[proc_macro_error]
pub fn validate_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    let ast = parse_macro_input!(input);

    // Build the trait implementation
    impl_validate_macro(&ast)
}

fn impl_validate_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let implemented_ast = quote! {
        impl Validate for #name {
            fn validate(&self) -> Result<(), ValidationError> {
                Ok(())
            }
        }
    };
    implemented_ast.into()
}

fn collect_fields(ast: &DeriveInput) -> Vec<Field> {
    match ast.data {
        Data::Struct(DataStruct { ref fields, .. }) => match fields {
            Fields::Named(named_fields) => {
                println!("named_fields: {:#?}", named_fields);
                vec![]
            }
            _ => abort!(fields.span(), "#[derive(Validate)] not supported empty structure"),
        },
        _ => abort!(
            ast.span(),
            "#[derive(Validate)] can only be used with structs"
        ),
    }
}
