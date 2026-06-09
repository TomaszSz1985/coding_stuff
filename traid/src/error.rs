use thiserror::Error;

/// All error types that can occur during RAID operations.
#[derive(Error, Debug)]
pub enum TraidError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    // #[error("disk index out of range: {0}")]
    // InvalidDisk(u8),
    #[error("Invalid hex string: {0}")]
    InvalidHex(String),

    #[error("Disk {0} is failed")]
    DiskFailed(u8),
}
