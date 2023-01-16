pub use validator_derive::*;

#[derive(Debug)]
pub struct ValidationError {
    message: &'static str,
}

impl ValidationError {
    fn new(message: &'static str) -> Self {
        Self { message }
    }
}

pub fn is_enum(current_value: &str, values: Vec<&'static str>) -> bool {
    values.contains(&current_value)
}

pub trait Validate {
    fn validate(&self) -> Result<(), ValidationError>;
}
