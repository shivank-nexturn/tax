use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaxonomyError {
    #[error("Invalid tax rate: {0}")]
    InvalidTaxRate(f64),
    
    #[error("Invalid date range: {0}")]
    InvalidDateRange(String),
    
    #[error("Invalid tax code: {0}")]
    InvalidTaxCode(String),
    
    #[error("Invalid nation code: {0}")]
    InvalidNationCode(String),
}
