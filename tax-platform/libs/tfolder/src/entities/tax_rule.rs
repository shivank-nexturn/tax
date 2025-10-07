use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRule {
    pub id: Uuid,
    pub code: String,
    pub description: String,
    pub rate: f64,
    pub effective_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TaxRule {
    pub fn new(
        code: String,
        description: String,
        rate: f64,
        effective_date: DateTime<Utc>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            code,
            description,
            rate,
            effective_date,
            expiry_date: None,
            created_at: now,
            updated_at: now,
        }
    }
}
