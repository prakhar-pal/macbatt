use crate::types::{BatteryInfo, BatteryState, ChargingStatus};
use colored::Colorize;
use std::time::Duration;

/// Formats battery information for display with appropriate styling
pub fn format_battery_display(info: &BatteryInfo, state: &BatteryState) -> String {
    let mut output = String::new();
    
    output.push_str("Battery Status\n");
    output.push_str("━━━━━━━━━━━━━━\n");
    
    // Format percentage with color coding based on state
    let percentage_str = format_percentage(info.percentage, state);
    output.push_str(&format!("Charge:      {}\n", percentage_str));
    
    // Format charging status
    let status_str = format_charging_status(&info.status);
    output.push_str(&format!("Status:      {}\n", status_str));
    
    // Only show time remaining when discharging
    if matches!(info.status, ChargingStatus::Discharging) {
        let time_left_str = format_time_remaining(info, &info.status);
        output.push_str(&format!("Time Left:   {}\n", time_left_str));
    }
    
    // Only show time to full charge when charging
    if matches!(info.status, ChargingStatus::Charging) {
        let charge_time_str = format_time_to_full(info, &info.status);
        output.push_str(&format!("Charge Time: {}\n", charge_time_str));
    }
    
    output
}

/// Formats the battery percentage with appropriate color and icon based on state
fn format_percentage(percentage: u8, state: &BatteryState) -> String {
    let percentage_text = format!("{}%", percentage);
    
    match state {
        BatteryState::Critical => format!("{} {}", "⚠".red(), percentage_text.red()),
        BatteryState::Warning => format!("{} {}", "⚠".yellow(), percentage_text.yellow()),
        BatteryState::Normal => percentage_text,
    }
}

/// Formats the charging status as human-readable text
fn format_charging_status(status: &ChargingStatus) -> String {
    match status {
        ChargingStatus::Charging => "Charging".to_string(),
        ChargingStatus::Discharging => "Discharging".to_string(),
        ChargingStatus::Charged => "Charged".to_string(),
        ChargingStatus::Unknown => "Unknown".to_string(),
    }
}

/// Formats time remaining based on battery info and charging status
fn format_time_remaining(info: &BatteryInfo, status: &ChargingStatus) -> String {
    // Check if fully charged
    if info.percentage == 100 && matches!(status, ChargingStatus::Charged | ChargingStatus::Charging) {
        return "Fully Charged".to_string();
    }
    
    // Only show time remaining when discharging
    match status {
        ChargingStatus::Discharging => {
            if let Some(duration) = info.time_remaining {
                format_duration(duration)
            } else {
                "Calculating".to_string()
            }
        }
        _ => "N/A".to_string(),
    }
}

/// Formats time to full charge based on battery info and charging status
fn format_time_to_full(info: &BatteryInfo, status: &ChargingStatus) -> String {
    // Check if fully charged
    if info.percentage == 100 && matches!(status, ChargingStatus::Charged | ChargingStatus::Charging) {
        return "Fully Charged".to_string();
    }
    
    // Only show time to full when charging
    match status {
        ChargingStatus::Charging => {
            if let Some(duration) = info.time_to_full {
                format_duration(duration)
            } else {
                "Calculating".to_string()
            }
        }
        _ => "N/A".to_string(),
    }
}

/// Formats a Duration into "Xh Ym" format
fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    
    if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}
