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

pub trait Validate {
    fn validate(&self) -> Result<(), ValidationError>;
}
