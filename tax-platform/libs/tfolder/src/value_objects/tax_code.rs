use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TaxCode {
    pub code: String,
    pub description: String,
}

impl TaxCode {
    pub fn new(code: String, description: String) -> Self {
        Self { code, description }
    }
}
