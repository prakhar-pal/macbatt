# mcbatt

A simple, elegant battery monitoring tool for macOS that displays battery status with color-coded warnings.

## Features

- **Single-shot mode**: Quick battery status check
- **Live monitoring mode**: Continuous updates with configurable refresh intervals
- **Color-coded warnings**: Visual indicators for low battery states
- **Customizable thresholds**: Set your own warning levels
- **Time estimates**: Shows time remaining (when discharging) or time to full charge (when charging)

## Installation

### From Source

```bash
git clone <repository-url>
cd mcbatt
cargo build --release
```

The binary will be available at `target/release/mcbatt`.

### Install Locally

```bash
cargo install --path .
```

## Usage

### Basic Usage

Display current battery status:

```bash
mcbatt
```

Output example:
```
Battery Status
━━━━━━━━━━━━━━
Charge:      95%
Status:      Discharging
Time Left:   2h 27m
```

### Live Monitoring Mode

Enable continuous monitoring with automatic updates:

```bash
mcbatt --live
```

This will clear the screen and update the battery status every 5 seconds (default).

### Custom Refresh Interval

Set a custom refresh interval (in seconds) for live mode:

```bash
mcbatt --live --interval 10
```

### Custom Warning Threshold

Set a custom battery percentage threshold for warnings:

```bash
mcbatt --threshold 30
```

- **Normal**: Battery percentage is above the threshold (no indicator)
- **Warning**: Battery percentage is below the threshold but above threshold/2 (⚠ yellow)
- **Critical**: Battery percentage is below threshold/2 (⚠ red)

### Combined Options

```bash
mcbatt --live --interval 3 --threshold 25
```

## Command-Line Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--live` | `-l` | Enable live monitoring mode | Off |
| `--interval` | `-i` | Refresh interval in seconds for live mode | 5 |
| `--threshold` | `-t` | Battery percentage threshold for warnings | 20 |
| `--help` | `-h` | Print help information | - |

## Requirements

- macOS (uses `ioreg` command)
- Rust 1.70+ (for building from source)

## Examples

Check battery status once:
```bash
mcbatt
```

Monitor battery with 2-second updates:
```bash
mcbatt -l -i 2
```

Set warning threshold at 40%:
```bash
mcbatt -t 40
```

Live mode with custom threshold and interval:
```bash
mcbatt --live --threshold 35 --interval 10
```

## License

MIT

## Contributing

