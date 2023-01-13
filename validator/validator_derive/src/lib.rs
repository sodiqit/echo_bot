use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::{
    parse_macro_input, spanned::Spanned, Data, DataStruct, DeriveInput, Field, Fields, Meta,
    MetaList, NestedMeta,
};
use validate::Validator;

mod validate;

#[proc_macro_derive(Validate, attributes(validate))]
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
        use validator::ValidationError;

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
            Fields::Named(_) => fields.iter().cloned().collect::<Vec<_>>(),
            _ => abort!(
                fields.span(),
                "#[derive(Validate)] not supported empty structure"
            ),
        },
        _ => abort!(
            ast.span(),
            "#[derive(Validate)] can only be used with structs"
        ),
    }
}

fn collect_field_validations(ast: &DeriveInput) {
    let fields = collect_fields(ast);

    let mut validators = vec![];

    for field in &fields {
        for attr in &field.attrs {
            let meta = attr.parse_meta();

            match meta {
                Ok(Meta::List(MetaList { ref nested, .. })) => {
                    let meta_items = nested.iter().collect::<Vec<_>>();

                    meta_items
                        .into_iter()
                        // .inspect(|item| println!("item: {:#?}", item))
                        .for_each(|item| {
                            match item {
                                NestedMeta::Meta(ref nested_item) => {
                                    match nested_item {
                                        Meta::List(MetaList { ref path, .. }) => {
                                            println!("nested_item: {:#?}", nested_item);
                                            println!("name: {:#?}", path);
                                            match path.get_ident().unwrap().to_string().as_str() {
                                                "is_enum" => validators
                                                    .push(Validator::IS_ENUM { values: vec![] }),
                                                v => abort!(
                                                    nested_item.span(),
                                                    "unexpected name value validator: {:?}",
                                                    v
                                                ),
                                            }
                                        }
                                        _ => unimplemented!(), //FIXME:
                                    }
                                }
                                _ => unimplemented!(), //FIXME:
                            }
                        });
                }
                Err(e) => {
                    abort!(
                        attr.span(),
                        "This attributes for the field `{}` seem to be misformed",
                        e
                    );
                }
                _ => {}
            }
        }
    }

    println!("validators: {:#?}", validators);
}
