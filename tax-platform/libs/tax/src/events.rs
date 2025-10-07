use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotReady {
    pub snapshot_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub data_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadCompleted {
    pub load_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub records_processed: u64,
    pub status: LoadStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadStatus {
    Success,
    Partial,
    Failed,
}
