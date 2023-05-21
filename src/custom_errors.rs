use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("Unknown error")]
    Unknown,
}