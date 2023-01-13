#[derive(Debug)]
pub enum Validator {
    IS_ENUM { values: Vec<&'static str> }
}