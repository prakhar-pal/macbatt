# Battery Monitor Design Document

## Overview

The Battery Monitor is a command-line Rust application that interfaces with macOS's `pmset` utility to retrieve and display battery information. The application supports two operational modes: single-shot (default) and live monitoring with configurable refresh intervals. It provides visual feedback through warning and error states based on configurable battery thresholds.

## Architecture

The application follows a modular architecture with clear separation of concerns:

```
┌─────────────────┐
│   CLI Parser    │ (clap)
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Main Control   │
│     Loop        │
└────────┬────────┘
         │
         ├──────────────┐
         ▼              ▼
┌─────────────────┐  ┌──────────────┐
│ Battery Service │  │   Display    │
│   (pmset exec)  │  │   Formatter  │
└─────────────────┘  └──────────────┘
```

### Key Components

1. **CLI Parser**: Handles command-line argument parsing using the `clap` crate
2. **Battery Service**: Executes `pmset` and parses output into structured data
3. **Display Formatter**: Formats battery data with appropriate warnings/errors
4. **Main Control Loop**: Orchestrates single-shot vs live mode execution

## Components and Interfaces

### 1. CLI Configuration

```rust
struct Config {
    live_mode: bool,
    refresh_interval: u64,  // seconds
    threshold: u8,          // percentage (default: 20)
}
```

**Command-line flags:**
- `--live` or `-l`: Enable live monitoring mode
- `--interval <seconds>` or `-i <seconds>`: Set refresh interval (default: 5)
- `--threshold <percentage>` or `-t <percentage>`: Set warning threshold (default: 20)

### 2. Battery Data Model

```rust
struct BatteryInfo {
    percentage: u8,
    status: ChargingStatus,
    time_remaining: Option<Duration>,
    time_to_full: Option<Duration>,
}

enum ChargingStatus {
    Charging,
    Discharging,
    Charged,
    Unknown,
}

enum BatteryState {
    Normal,
    Warning,    // below threshold
    Critical,   // below threshold/2
}
```

### 3. Battery Service

**Responsibilities:**
- Execute `pmset -g batt` command
- Parse output to extract battery metrics
- Handle command execution errors

**Key functions:**
```rust
fn get_battery_info() -> Result<BatteryInfo, BatteryError>
fn parse_pmset_output(output: &str) -> Result<BatteryInfo, ParseError>
```

**pmset output parsing strategy:**
- Use regex or string parsing to extract percentage from output like "80%"
- Parse charging status from strings: "charging", "discharging", "charged", "AC attached"
- Extract time estimates from formats like "2:30 remaining" or "1:15 until charged"

### 4. Display Formatter

**Responsibilities:**
- Format battery information for terminal output
- Apply color coding for warning/error states
- Clear screen in live mode

**Key functions:**
```rust
fn format_battery_display(info: &BatteryInfo, state: BatteryState) -> String
fn determine_battery_state(percentage: u8, threshold: u8) -> BatteryState
fn clear_screen()
```

**Display format:**
```
Battery Status
━━━━━━━━━━━━━━
Charge:     85%
Status:     Charging
Time Left:  2h 30m
Charge Time: N/A

[Live mode: Updates every 5s | Press Ctrl+C to exit]
```

**Color scheme (using colored crate):**
- Normal: Default terminal colors
- Warning: Yellow text for percentage and warning icon
- Critical: Red text for percentage and error icon

### 5. Main Control Loop

**Single-shot mode:**
1. Parse CLI arguments
2. Fetch battery info
3. Display formatted output
4. Exit

**Live mode:**
1. Parse CLI arguments
2. Enter infinite loop:
   - Clear screen
   - Fetch battery info
   - Display formatted output
   - Sleep for refresh_interval
   - Repeat until Ctrl+C

## Data Models

### pmset Output Example

```
Now drawing from 'Battery Power'
 -InternalBattery-0 (id=12345678)	85%; discharging; 2:30 remaining present: true
```

### Parsing Strategy

1. Extract percentage: Regex `(\d+)%`
2. Extract status: Match keywords "charging", "discharging", "charged", "AC attached"
3. Extract time: Regex `(\d+):(\d+) remaining` or `(\d+):(\d+) until charged`
4. Handle edge cases: "calculating", "(no estimate)", fully charged scenarios

## Error Handling

### Error Types

```rust
enum BatteryError {
    CommandFailed(String),
    ParseError(String),
    InvalidThreshold(String),
    InvalidInterval(String),
}
```

### Error Handling Strategy

1. **Command execution failures**: Display user-friendly error message indicating pmset is unavailable or failed
2. **Parse errors**: Log warning and display "N/A" for unparseable fields, continue with available data
3. **Invalid CLI arguments**: Display error message with usage help and exit with non-zero code
4. **Ctrl+C in live mode**: Gracefully exit with cleanup (restore cursor, clear line)

### Graceful Degradation

- If time estimates are unavailable, display "Calculating..." or "N/A"
- If charging status is ambiguous, default to "Unknown"
- Continue operation even if some fields cannot be parsed

## Testing Strategy

### Unit Tests

1. **pmset output parsing**:
   - Test various pmset output formats
   - Test edge cases (0%, 100%, calculating time)
   - Test malformed output handling

2. **Battery state determination**:
   - Test threshold boundaries
   - Test critical state (below threshold/2)
   - Test normal state

3. **Time formatting**:
   - Test duration to string conversion
   - Test None/Some handling

### Integration Tests

1. **CLI argument parsing**:
   - Test default values
   - Test custom intervals and thresholds
   - Test invalid inputs

2. **End-to-end single-shot mode**:
   - Mock pmset output
   - Verify formatted display output

### Manual Testing

1. Test on actual macOS system with various battery states
2. Test live mode with different refresh intervals
3. Test warning/critical states by adjusting threshold
4. Test charging vs discharging scenarios

## Dependencies

- `clap` (v4.x): Command-line argument parsing with derive macros
- `colored` (v2.x): Terminal color output
- `std::process::Command`: Execute pmset command
- `std::thread::sleep`: Implement refresh intervals
- `regex` (v1.x): Parse pmset output

## Implementation Notes

1. **Platform-specific**: This application is macOS-only due to pmset dependency
2. **Permissions**: No special permissions required; pmset is available to all users
3. **Terminal compatibility**: Use ANSI escape codes for screen clearing (works in most modern terminals)
4. **Signal handling**: Consider using `ctrlc` crate for graceful shutdown in live mode
5. **Performance**: pmset execution is fast (<100ms), so refresh intervals as low as 1 second are feasible
