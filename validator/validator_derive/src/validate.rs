use proc_macro2::Ident;
use syn::Lit;

#[derive(Debug)]
pub enum Validator {
    Enum(Vec<String>)
}

#[derive(Debug)]
pub struct FieldValidation {
    pub name: Ident,
    pub validators: Vec<Validator>,
}

pub fn lit_to_string(lit: &Lit) -> Option<String> {
    match *lit {
        syn::Lit::Str(ref s) => Some(s.value()),
        _ => None,
    }
}