# Tea Timer

Tea Timer is a simple and efficient Rust library for measuring and reporting the duration of tasks. It provides an easy-to-use API for creating timers, measuring elapsed time, and formatting durations.

## Features

- Create named timers
- Measure elapsed time
- Format durations in a human-readable format
- Restart timers with new task names
- Optional logging support using the `log` crate

## Installation

Add this to your `Cargo.toml`:

```toml
tea-timer = "0.1.0"
```

## Usage

### Macro Usage
```rust
let result = tea_timer::took! {
    // ...any code
};
// this will print elapsed time and get result of thecode block
```

### Function Usage
```rust
use tea_timer::took;

let result = took(|| {
    // ...any code
}, "task");
// this will print elapsed time and get result of the function
```

### Basic Usage
```rust
use tea_timer::Timer;
use std::thread::sleep;
use std::time::Duration;

let mut timer = Timer::new("task");
// Simulate some work with a sleep
sleep(Duration::from_secs(2));
// this will print elapsed time
timer.elapsed();
// Restart the timer with a new task name
timer.restart("new_task");
// Simulate more work
sleep(Duration::from_millis(500));
// Measure elapsed time again
// consume timer and print elapsed time
timer.stop();
```

### Logging Usage
```rust
use tea_timer::Timer;
use std::thread::sleep;
use std::time::Duration;

let mut timer = Timer::new("task");
timer.log();  // This will log the elapsed time using the log crate
```

