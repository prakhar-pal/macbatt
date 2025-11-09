use std::time::Duration;
use clap::Parser;

/// CLI configuration for the battery monitor
#[derive(Parser, Debug)]
#[command(name = "macbatt")]
#[command(about = "Battery monitoring tool for macOS", long_about = None)]
pub struct Config {
    /// Enable live monitoring mode
    #[arg(short = 'l', long = "live")]
    pub live_mode: bool,

    /// Refresh interval in seconds for live mode
    #[arg(short = 'i', long = "interval", default_value_t = 5)]
    pub refresh_interval: u64,

    /// Battery percentage threshold for warnings
    #[arg(short = 't', long = "threshold", default_value_t = 20)]
    pub threshold: u8,
}

impl Config {
    /// Validates the configuration and returns an error if invalid
    pub fn validate(&self) -> Result<(), String> {
        // Validate refresh interval is positive (already guaranteed by u64, but check for 0)
        if self.refresh_interval == 0 {
            return Err("Refresh interval must be a positive integer".to_string());
        }

        // Validate threshold is between 1 and 100
        if self.threshold < 1 || self.threshold > 100 {
            return Err("Threshold must be between 1 and 100".to_string());
        }

        Ok(())
    }
}

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
