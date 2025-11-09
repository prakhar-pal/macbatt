mod battery;
mod display;
mod types;

use battery::{determine_battery_state, get_battery_info};
use clap::Parser;
use display::format_battery_display;
use std::thread;
use std::time::Duration;
use types::{BatteryError, Config};

fn main() {
    let config = Config::parse();

    // Validate the configuration
    if let Err(e) = config.validate() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    // Execute based on mode
    if config.live_mode {
        if let Err(e) = run_live_mode(&config) {
            handle_error(e);
        }
    } else {
        if let Err(e) = run_single_shot(&config) {
            handle_error(e);
        }
    }
}

/// Runs the battery monitor in single-shot mode
fn run_single_shot(config: &Config) -> Result<(), BatteryError> {
    // Get battery info
    let battery_info = get_battery_info()?;

    // Determine battery state based on threshold
    let battery_state = determine_battery_state(battery_info.percentage, config.threshold);

    // Format and display output
    let output = format_battery_display(&battery_info, &battery_state);
    println!("{}", output);

    Ok(())
}

/// Runs the battery monitor in live mode with continuous updates
fn run_live_mode(config: &Config) -> Result<(), BatteryError> {
    loop {
        // Clear screen at start of each iteration
        clear_screen();

        // Fetch battery info and determine state
        let battery_info = get_battery_info()?;
        let battery_state = determine_battery_state(battery_info.percentage, config.threshold);

        // Format and display output
        let output = format_battery_display(&battery_info, &battery_state);
        println!("{}", output);

        // Display live mode indicator
        let indicator = format_live_mode_indicator(config.refresh_interval);
        println!("{}", indicator);

        // Sleep for configured refresh interval
        thread::sleep(Duration::from_secs(config.refresh_interval));
    }
}

/// Clears the terminal screen using ANSI escape codes
fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

/// Formats the live mode indicator with refresh interval and exit instructions
fn format_live_mode_indicator(refresh_interval: u64) -> String {
    format!(
        "\n[Live mode: Updates every {}s | Press Ctrl+C to exit]",
        refresh_interval
    )
}

/// Handles battery errors with user-friendly messages
fn handle_error(error: BatteryError) {
    match error {
        BatteryError::CommandFailed(msg) => {
            eprintln!("Error: Unable to retrieve battery information.");
            eprintln!("Details: {}", msg);
            eprintln!("\nMake sure you are running this on macOS with pmset available.");
            std::process::exit(1);
        }
        BatteryError::ParseError(msg) => {
            eprintln!("Error: Failed to parse battery information.");
            eprintln!("Details: {}", msg);
            eprintln!("\nThe battery data format may have changed. Please report this issue.");
            std::process::exit(1);
        }
        BatteryError::InvalidThreshold(msg) => {
            eprintln!("Error: {}", msg);
            std::process::exit(1);
        }
        BatteryError::InvalidInterval(msg) => {
            eprintln!("Error: {}", msg);
            std::process::exit(1);
        }
    }
}
