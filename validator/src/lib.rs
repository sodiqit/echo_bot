pub use validator_derive::*;

#[derive(Debug)]
pub struct ValidationError {
    message: String,
    field: &'static str,
}

impl ValidationError {
    pub fn new(message: String, field: &'static str) -> Self {
        Self { message, field }
    }
}

pub fn is_enum(current_value: &str, values: Vec<&'static str>) -> bool {
    values.contains(&current_value)
}

pub trait Validate {
    fn validate(&self) -> Result<(), Vec<ValidationError>>;
}
