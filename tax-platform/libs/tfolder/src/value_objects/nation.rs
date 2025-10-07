use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Nation {
    pub code: String,
    pub name: String,
}

impl Nation {
    pub fn new(code: String, name: String) -> Self {
        Self { code, name }
    }
}
