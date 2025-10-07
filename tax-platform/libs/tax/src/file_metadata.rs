use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub id: Uuid,
    pub filename: String,
    pub file_size: u64,
    pub content_type: String,
    pub checksum: String,
    pub uploaded_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
    pub status: FileStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}
