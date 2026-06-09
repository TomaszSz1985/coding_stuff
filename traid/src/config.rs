use serde::{Deserialize, Serialize};

/// Persistent configuration for a RAID array, stored in `raid.json`.
#[derive(Serialize, Debug, Deserialize)]
pub struct RaidConfig {
    /// Total number of virtual disks (data disks + 1 parity disk).
    pub disks: u8,
    /// Size of a single block in bytes; defines the stripe granularity.
    pub block_size: u64,
    /// Capacity of each virtual disk file in bytes.
    pub disk_size: u64,
    /// Index of the currently failed disk, or `None` when the array is healthy.
    #[serde(default)]
    pub failed_disk: Option<u8>,
}

impl RaidConfig {
    /// Serialises the config to pretty-printed JSON and writes it to `raid.json`.
    pub fn save(&self) -> Result<(), crate::error::TraidError> {
        let str_json = serde_json::to_string_pretty(&self)?;
        std::fs::write("raid.json", str_json)?;
        Ok(())
    }

    /// Reads and deserialises `raid.json` from the current working directory.
    pub fn load() -> Result<RaidConfig, crate::error::TraidError> {
        let file_json = std::fs::read_to_string("raid.json")?;
        let config = serde_json::from_str(&file_json)?;
        Ok(config)
    }
}
