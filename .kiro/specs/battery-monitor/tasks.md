# Implementation Plan

- [x] 1. Set up project dependencies and configuration
  - Add required dependencies to Cargo.toml: clap (v4.x with derive feature), colored (v2.x), and regex (v1.x)
  - Configure the binary crate settings if needed
  - _Requirements: All requirements depend on these dependencies_

- [x] 2. Implement core data models and types
  - [x] 2.1 Create battery data structures
    - Define `BatteryInfo` struct with percentage, status, time_remaining, and time_to_full fields
    - Define `ChargingStatus` enum with Charging, Discharging, Charged, and Unknown variants
    - Define `BatteryState` enum with Normal, Warning, and Critical variants
    - Define `BatteryError` enum for error handling
    - _Requirements: 1.1, 2.1, 3.1, 4.1_

- [x] 3. Implement pmset integration and parsing
  - [x] 3.1 Create battery service module
    - Implement `get_battery_info()` function that executes `pmset -g batt` command
    - Handle command execution errors and return appropriate BatteryError
    - _Requirements: 1.1, 1.3, 2.1_
  
  - [x] 3.2 Implement pmset output parser
    - Write regex-based parser to extract battery percentage from pmset output
    - Parse charging status from keywords (charging, discharging, charged, AC attached)
    - Parse time remaining format (e.g., "2:30 remaining")
    - Parse time to full charge format (e.g., "1:15 until charged")
    - Handle edge cases like "calculating", "(no estimate)", and missing data
    - _Requirements: 1.1, 2.1, 2.2, 2.3, 3.1, 3.2, 4.1, 4.2_
  
  - [ ]* 3.3 Write unit tests for pmset parsing
    - Test parsing various pmset output formats
    - Test edge cases (0%, 100%, calculating time, no estimate)
    - Test malformed output handling
    - _Requirements: 1.1, 2.1, 3.1, 4.1_

- [x] 4. Implement battery state determination logic
  - [x] 4.1 Create state evaluation function
    - Implement `determine_battery_state()` that takes percentage and threshold
    - Return Normal state when percentage is above threshold
    - Return Warning state when percentage is below threshold but above threshold/2
    - Return Critical state when percentage is below threshold/2
    - _Requirements: 5.1, 5.2, 6.1_
  
  - [ ]* 4.2 Write unit tests for state determination
    - Test threshold boundaries
    - Test critical state calculation (below threshold/2)
    - Test normal state
    - _Requirements: 5.1, 5.2, 6.1, 6.2_

- [x] 5. Implement CLI argument parsing
  - [x] 5.1 Create CLI configuration structure
    - Define `Config` struct with live_mode, refresh_interval, and threshold fields
    - Use clap derive macros to define command-line arguments
    - Add `--live` / `-l` flag for live mode
    - Add `--interval` / `-i` argument with default value of 5 seconds
    - Add `--threshold` / `-t` argument with default value of 20 percent
    - _Requirements: 5.3, 7.1, 7.3, 8.1, 8.2, 9.1_
  
  - [x] 5.2 Add input validation
    - Validate that refresh interval is a positive integer
    - Validate that threshold is between 1 and 100
    - Return appropriate error messages for invalid inputs
    - _Requirements: 8.3, 8.4_

- [x] 6. Implement display formatting and output
  - [x] 6.1 Create display formatter module
    - Implement `format_battery_display()` function that takes BatteryInfo and BatteryState
    - Format battery percentage with % symbol
    - Format charging status as human-readable text (Charging, Discharging, Charged)
    - Format time remaining in "Xh Ym" format when available, "N/A" or "Calculating" otherwise
    - Format time to full charge in "Xh Ym" format when charging, "N/A" when discharging
    - Display "Fully Charged" when battery is at 100% and on AC power
    - Only show time remaining when discharging, only show charge time when charging
    - _Requirements: 1.2, 2.2, 2.3, 2.4, 3.2, 3.3, 4.2, 4.3_
  
  - [x] 6.2 Implement color coding for battery states
    - Apply yellow color to percentage and add warning icon for Warning state
    - Apply red color to percentage and add error icon for Critical state
    - Use default colors for Normal state
    - Ensure error state has higher visual priority than warning state
    - _Requirements: 5.2, 6.2, 6.3_

- [x] 7. Implement main control loop and execution modes
  - [x] 7.1 Implement single-shot mode
    - Parse CLI arguments to get configuration
    - Call get_battery_info() once
    - Determine battery state based on threshold
    - Format and display output using format_battery_display()
    - Exit program after displaying information
    - _Requirements: 9.1, 9.2, 9.3_
  
  - [x] 7.2 Implement live mode with screen clearing
    - Check if live mode flag is enabled
    - Create `clear_screen()` function using ANSI escape codes
    - Create `format_live_mode_indicator()` to show refresh interval and exit instructions
    - Enter infinite loop that runs until Ctrl+C
    - Clear screen at start of each iteration
    - Fetch battery info and determine state
    - Format and display output with live mode indicator
    - Sleep for configured refresh_interval duration
    - _Requirements: 7.1, 7.2, 7.3, 7.4, 8.2_
  
  - [x] 7.3 Add graceful error handling
    - Handle BatteryError cases with user-friendly messages
    - Display "N/A" for unparseable fields while continuing with available data
    - Exit with appropriate error codes for fatal errors
    - Handle Ctrl+C gracefully in live mode (optional: use ctrlc crate)
    - _Requirements: 1.3, 3.3, 8.4_

- [x] 8. Wire everything together in main.rs
  - Import all modules (battery service, display formatter, CLI config)
  - Parse CLI arguments using clap
  - Implement main execution flow that branches between single-shot and live mode
  - Add top-level error handling and program exit logic
  - _Requirements: All requirements_

- [ ]* 9. Add integration tests
  - Test CLI argument parsing with various inputs
  - Test default values for optional arguments
  - Test invalid input handling
  - _Requirements: 5.3, 8.3, 8.4_
