use crate::types::{BatteryError, BatteryInfo, BatteryState, ChargingStatus};
use regex::Regex;
use std::process::Command;
use std::time::Duration;

/// Retrieves battery information by executing ioreg command
pub fn get_battery_info() -> Result<BatteryInfo, BatteryError> {
    let output = Command::new("ioreg")
        .arg("-rn")
        .arg("AppleSmartBattery")
        .output()
        .map_err(|e| BatteryError::CommandFailed(format!("Failed to execute ioreg: {}", e)))?;

    if !output.status.success() {
        return Err(BatteryError::CommandFailed(
            "ioreg command returned non-zero exit code".to_string(),
        ));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    parse_ioreg_output(&output_str)
}

/// Parses ioreg output to extract battery information
fn parse_ioreg_output(output: &str) -> Result<BatteryInfo, BatteryError> {
    // Extract UISoc (UI State of Charge) from BatteryData - this is the user-visible percentage
    // that matches what macOS shows in the menu bar
    let percentage = parse_battery_data_field(output, "UISoc")?;
    
    // Extract charging status
    let is_charging = parse_field_bool(output, "IsCharging");
    let external_connected = parse_field_bool(output, "ExternalConnected");
    let fully_charged = parse_field_bool(output, "FullyCharged");
    
    let status = determine_charging_status(is_charging, external_connected, fully_charged);
    
    // Extract time remaining (when discharging) - in minutes
    let time_remaining = if !is_charging && !external_connected {
        parse_field_u64(output, "TimeRemaining").map(|mins| Duration::from_secs(mins * 60))
    } else {
        None
    };
    
    // Extract time to full charge (when charging) - in minutes
    let time_to_full = if is_charging {
        parse_field_u64(output, "AvgTimeToFull")
            .filter(|&mins| mins != 65535) // 65535 means not available
            .map(|mins| Duration::from_secs(mins * 60))
    } else {
        None
    };

    Ok(BatteryInfo {
        percentage,
        status,
        time_remaining,
        time_to_full,
    })
}

/// Extracts a u64 field value from ioreg output
fn parse_field_u64(output: &str, field_name: &str) -> Option<u64> {
    let pattern = format!(r#""{}" = (\d+)"#, field_name);
    let re = Regex::new(&pattern).unwrap();
    
    re.captures(output)
        .and_then(|caps| caps.get(1))
        .and_then(|m| m.as_str().parse::<u64>().ok())
}

/// Extracts a boolean field value from ioreg output
fn parse_field_bool(output: &str, field_name: &str) -> bool {
    let pattern = format!(r#""{}" = Yes"#, field_name);
    output.contains(&pattern)
}

/// Extracts a field value from the BatteryData nested structure
fn parse_battery_data_field(output: &str, field_name: &str) -> Result<u8, BatteryError> {
    // Find the BatteryData line and extract the field from within it
    // Format: "BatteryData" = {"UISoc"=90,...}
    let battery_data_pattern = r#""BatteryData" = \{[^}]+\}"#;
    let re = Regex::new(battery_data_pattern).unwrap();
    
    if let Some(battery_data_match) = re.find(output) {
        let battery_data_str = battery_data_match.as_str();
        
        // Now extract the specific field from within BatteryData
        let field_pattern = format!(r#""{}"=(\d+)"#, field_name);
        let field_re = Regex::new(&field_pattern).unwrap();
        
        if let Some(caps) = field_re.captures(battery_data_str) {
            if let Some(value_str) = caps.get(1) {
                return value_str
                    .as_str()
                    .parse::<u8>()
                    .map_err(|e| BatteryError::ParseError(format!("Invalid {} value: {}", field_name, e)));
            }
        }
    }
    
    Err(BatteryError::ParseError(
        format!("Could not find {} in BatteryData", field_name),
    ))
}

/// Determines charging status based on ioreg flags
fn determine_charging_status(is_charging: bool, external_connected: bool, fully_charged: bool) -> ChargingStatus {
    if fully_charged {
        ChargingStatus::Charged
    } else if is_charging {
        ChargingStatus::Charging
    } else if external_connected {
        ChargingStatus::Charged
    } else {
        ChargingStatus::Discharging
    }
}

/// Determines the battery state based on percentage and threshold
/// 
/// Returns:
/// - Normal: when percentage is above threshold
/// - Warning: when percentage is below threshold but above threshold/2
/// - Critical: when percentage is below threshold/2
pub fn determine_battery_state(percentage: u8, threshold: u8) -> BatteryState {
    let critical_threshold = threshold / 2;
    
    if percentage < critical_threshold {
        BatteryState::Critical
    } else if percentage < threshold {
        BatteryState::Warning
    } else {
        BatteryState::Normal
    }
}
