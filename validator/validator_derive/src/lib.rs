use proc_macro2::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, Data, DataStruct, DeriveInput, Field, Fields,
    Lit, Meta, MetaList, NestedMeta,
};
use validate::{FieldValidation, Validator};

mod validate;

#[proc_macro_derive(Validate, attributes(validate))]
#[proc_macro_error]
pub fn validate_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a representation of Rust code as a syntax tree
    let ast = parse_macro_input!(input);

    // Build the trait implementation
    impl_validate_macro(&ast).into()
}

fn impl_validate_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields_validations = collect_fields_validations(&ast);

    let validations = quote_validation(fields_validations);

    let implemented_ast = quote! {
        use validator::ValidationError;

        impl Validate for #name {
            fn validate(&self) -> Result<(), ValidationError> {
                #(#validations)*
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

fn collect_fields_validations(ast: &DeriveInput) -> Vec<FieldValidation> {
    let fields = collect_fields(ast);

    fields
        .iter()
        .map(|field| find_field_validations(field))
        .collect::<Vec<_>>()
}

fn find_field_validations(field: &Field) -> FieldValidation {
    let mut validators = vec![];

    for attr in &field.attrs {
        let meta = attr.parse_meta();

        // check if attr is #[validate]
        if attr.path != parse_quote!(validate) {
            continue;
        }

        match meta {
            Ok(Meta::List(MetaList { ref nested, .. })) => {
                let meta_items = nested.iter().collect::<Vec<_>>();

                meta_items.into_iter().for_each(|item| {
                    match item {
                        NestedMeta::Meta(ref nested_item) => {
                            match nested_item {
                                // attributes with several args
                                Meta::List(list) => {
                                    match list.path.get_ident().unwrap().to_string().as_str() {
                                        "is_enum" => validators.push(Validator::Enum(
                                            extract_enum_args_validations(list),
                                        )),
                                        v => abort!(
                                            nested_item.span(),
                                            "unexpected name value validator: {:?}",
                                            v
                                        ),
                                    }
                                }
                                _ => abort!(
                                    attr.span(),
                                    "currently support only attributes with args",
                                ),
                            }
                        }
                        _ => abort!(
                            attr.span(),
                            "currently support only structured meta, like: validate(is_enum(1, 2))"
                        ),
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

    FieldValidation {
        name: field.ident.clone().unwrap(),
        validators,
    }
}

fn extract_enum_args_validations(meta_list: &MetaList) -> Vec<String> {
    if meta_list.nested.is_empty() {
        abort!(meta_list.span(), "is_enum must have 1 or more args");
    }

    meta_list
        .nested
        .clone()
        .into_iter()
        .map(|meta| match meta {
            NestedMeta::Meta(_) => None,
            NestedMeta::Lit(ref lit) => match lit {
                Lit::Str(_) => validate::lit_to_string(lit),
                _ => abort!(lit.span(), "invalid arg type. is_enum support only strings"),
            },
        })
        .flatten()
        .collect::<Vec<_>>()
}

fn quote_validation(fields_validation: Vec<FieldValidation>) -> Vec<TokenStream> {
    let result = fields_validation
        .into_iter()
        .fold(vec![], |mut acc, field_validation| {
            let FieldValidation { name, validators } = field_validation;
            for validator in validators {
                match validator {
                    Validator::Enum(values) => {
                        let res = quote! {
                            use validator::is_enum;

                            let is_valid_enum = is_enum(self.#name.as_str(), vec!(#(#values,)*));
                        };

                        acc.push(res);
                    }
                }
            }

            acc
        });

    result
}
