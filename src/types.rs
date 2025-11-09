use std::time::Duration;

/// Represents the charging status of the battery
#[derive(Debug, Clone, PartialEq)]
pub enum ChargingStatus {
    Charging,
    Discharging,
    Charged,
    Unknown,
}

/// Represents the battery state based on threshold
#[derive(Debug, Clone, PartialEq)]
pub enum BatteryState {
    Normal,
    Warning,
    Critical,
}

/// Contains all battery information retrieved from pmset
#[derive(Debug, Clone)]
pub struct BatteryInfo {
    pub percentage: u8,
    pub status: ChargingStatus,
    pub time_remaining: Option<Duration>,
    pub time_to_full: Option<Duration>,
}

/// Error types for battery operations
#[derive(Debug)]
pub enum BatteryError {
    CommandFailed(String),
    ParseError(String),
    InvalidThreshold(String),
    InvalidInterval(String),
}

impl std::fmt::Display for BatteryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BatteryError::CommandFailed(msg) => write!(f, "Command failed: {}", msg),
            BatteryError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            BatteryError::InvalidThreshold(msg) => write!(f, "Invalid threshold: {}", msg),
            BatteryError::InvalidInterval(msg) => write!(f, "Invalid interval: {}", msg),
        }
    }
}

impl std::error::Error for BatteryError {}
