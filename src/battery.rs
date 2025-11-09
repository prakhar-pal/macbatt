use crate::types::{BatteryError, BatteryInfo, ChargingStatus};
use regex::Regex;
use std::process::Command;
use std::time::Duration;

/// Retrieves battery information by executing pmset command
pub fn get_battery_info() -> Result<BatteryInfo, BatteryError> {
    let output = Command::new("pmset")
        .arg("-g")
        .arg("batt")
        .output()
        .map_err(|e| BatteryError::CommandFailed(format!("Failed to execute pmset: {}", e)))?;

    if !output.status.success() {
        return Err(BatteryError::CommandFailed(
            "pmset command returned non-zero exit code".to_string(),
        ));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    parse_pmset_output(&output_str)
}

/// Parses pmset output to extract battery information
fn parse_pmset_output(output: &str) -> Result<BatteryInfo, BatteryError> {
    // Extract battery percentage
    let percentage = parse_percentage(output)?;
    
    // Extract charging status
    let status = parse_charging_status(output);
    
    // Extract time remaining (when discharging)
    let time_remaining = parse_time_remaining(output);
    
    // Extract time to full charge (when charging)
    let time_to_full = parse_time_to_full(output);

    Ok(BatteryInfo {
        percentage,
        status,
        time_remaining,
        time_to_full,
    })
}

/// Extracts battery percentage from pmset output
fn parse_percentage(output: &str) -> Result<u8, BatteryError> {
    let re = Regex::new(r"(\d+)%").unwrap();
    
    if let Some(caps) = re.captures(output) {
        if let Some(percentage_str) = caps.get(1) {
            return percentage_str
                .as_str()
                .parse::<u8>()
                .map_err(|e| BatteryError::ParseError(format!("Invalid percentage value: {}", e)));
        }
    }
    
    Err(BatteryError::ParseError(
        "Could not find battery percentage in pmset output".to_string(),
    ))
}

/// Extracts charging status from pmset output
fn parse_charging_status(output: &str) -> ChargingStatus {
    let output_lower = output.to_lowercase();
    
    if output_lower.contains("charging") && !output_lower.contains("discharging") {
        ChargingStatus::Charging
    } else if output_lower.contains("discharging") {
        ChargingStatus::Discharging
    } else if output_lower.contains("charged") || output_lower.contains("ac attached") {
        ChargingStatus::Charged
    } else {
        ChargingStatus::Unknown
    }
}

/// Extracts time remaining from pmset output (for discharging state)
fn parse_time_remaining(output: &str) -> Option<Duration> {
    let re = Regex::new(r"(\d+):(\d+) remaining").unwrap();
    
    if let Some(caps) = re.captures(output) {
        let hours: u64 = caps.get(1)?.as_str().parse().ok()?;
        let minutes: u64 = caps.get(2)?.as_str().parse().ok()?;
        return Some(Duration::from_secs(hours * 3600 + minutes * 60));
    }
    
    // Check for edge cases
    if output.contains("(no estimate)") || output.contains("calculating") {
        return None;
    }
    
    None
}

/// Extracts time to full charge from pmset output (for charging state)
fn parse_time_to_full(output: &str) -> Option<Duration> {
    let re = Regex::new(r"(\d+):(\d+) until charged").unwrap();
    
    if let Some(caps) = re.captures(output) {
        let hours: u64 = caps.get(1)?.as_str().parse().ok()?;
        let minutes: u64 = caps.get(2)?.as_str().parse().ok()?;
        return Some(Duration::from_secs(hours * 3600 + minutes * 60));
    }
    
    None
}
