mod battery;
mod types;

use clap::Parser;
use types::Config;

fn main() {
    let config = Config::parse();
    
    // Validate the configuration
    if let Err(e) = config.validate() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    
    println!("Battery Monitor Configuration:");
    println!("  Live mode: {}", config.live_mode);
    println!("  Refresh interval: {} seconds", config.refresh_interval);
    println!("  Threshold: {}%", config.threshold);
}
