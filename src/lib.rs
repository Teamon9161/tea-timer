//! # Tea Timer
//!
//! Tea Timer is a simple and efficient Rust library for measuring and reporting the duration of tasks. It provides an easy-to-use API for creating timers, measuring elapsed time, and formatting durations.
//!
//! ## Features
//!
//! - Create named timers
//! - Measure elapsed time
//! - Format durations in a human-readable format
//! - Restart timers with new task names
//! - Optional logging support using the `log` crate
//!
//! ## Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! tea-timer = "0.1.0"
//! ```
//!
//! ## Usage
//!
//! ### Macro Usage
//! ```rust
//! // add [macro_use] to the top of the file
//! #[macro_use]
//! extern crate tea_timer;
//!
//! let result = tea_timer::took! {
//!     // ...any code
//! };
//! // this will print elapsed time and get result of thecode block
//! ```
//!
//! ### Function Usage
//! ```rust
//! use tea_timer::took;
//!
//! let result = took(|| {
//!     // ...any code
//! }, "task");
//! // this will print elapsed time and get result of the function
//! ```
//!
//! ### Basic Usage
//! ```rust
//! use tea_timer::Timer;
//! use std::thread::sleep;
//! use std::time::Duration;
//!
//! let mut timer = Timer::new("task");
//! // Simulate some work with a sleep
//! sleep(Duration::from_secs(2));
//! // this will print elapsed time
//! timer.elapsed();
//! // Restart the timer with a new task name
//! timer.restart("new_task");
//! // Simulate more work
//! sleep(Duration::from_millis(500));
//! // Measure elapsed time again
//! // consume timer and print elapsed time
//! timer.stop();
//! ```
//!
//! ### Logging Usage
//! ```rust
//! use tea_timer::Timer;
//! use std::thread::sleep;
//! use std::time::Duration;
//!
//! let mut timer = Timer::new("task");
//! timer.log();  // This will log the elapsed time using the log crate
//! ```

mod display;

use std::time::Instant;

/// A struct for measuring and reporting the duration of tasks.
///
/// # Examples
///
/// ```
/// use tea_timer::Timer;
/// use std::thread::sleep;
/// use std::time::Duration;
///
/// let timer = Timer::new("Some Task");
/// sleep(Duration::from_millis(100));
/// timer.stop(); // This will print the duration of the task
/// ```
pub struct Timer {
    pub start_time: Instant,
    pub task_name: String,
}

impl Default for Timer {
    #[inline]
    fn default() -> Self {
        Timer::new("")
    }
}

impl std::fmt::Debug for Timer {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.elapsed_str())
    }
}

impl std::fmt::Display for Timer {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.elapsed_str())
    }
}

impl Timer {
    /// Creates a new `Timer` instance with the given task name.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_timer::Timer;
    ///
    /// let timer = Timer::new("My Task");
    /// assert_eq!(timer.task_name, "My Task");
    /// ```
    #[inline]
    pub fn new(task_name: &str) -> Self {
        Timer {
            start_time: Instant::now(),
            task_name: task_name.to_string(),
        }
    }

    /// Restarts the timer with a new task name.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_timer::Timer;
    ///
    /// let mut timer = Timer::new("Task 1");
    /// // Do some work...
    /// timer.restart("Task 2");
    /// assert_eq!(timer.task_name, "Task 2");
    /// // Timer now measures a new task
    /// ```
    #[inline]
    pub fn restart(&mut self, task_name: &str) {
        self.start_time = Instant::now();
        self.task_name = task_name.to_string();
    }

    /// Returns the duration elapsed since the timer started.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_timer::Timer;
    /// use std::thread::sleep;
    /// use std::time::Duration;
    ///
    /// let timer = Timer::new("Test Task");
    /// sleep(Duration::from_millis(10));
    /// assert!(timer.duration().as_millis() >= 10);
    /// ```
    #[inline]
    pub fn duration(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }

    /// Returns a formatted string representation of the elapsed duration.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_timer::Timer;
    /// use std::thread::sleep;
    /// use std::time::Duration;
    ///
    /// let timer = Timer::new("Test Task");
    /// sleep(Duration::from_millis(10));
    /// assert!(timer.duration_str().contains("ms"));
    /// ```
    #[inline]
    pub fn duration_str(&self) -> String {
        display::format_duration(self.duration())
    }

    #[inline]
    pub fn elapsed_str(&self) -> String {
        format!("{} elapsed {}", self.task_name, self.duration_str())
    }

    #[inline]
    pub fn took_str(&self) -> String {
        format!("{} took {}", self.task_name, self.duration_str())
    }

    /// Prints the elapsed time for the task.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_timer::Timer;
    /// use std::thread::sleep;
    /// use std::time::Duration;
    ///
    /// let timer = Timer::new("Test Task");
    /// sleep(Duration::from_millis(10));
    /// timer.elapsed(); // This will print to stdout
    /// ```
    #[inline]
    pub fn elapsed(&self) {
        println!("{}", self.elapsed_str());
    }

    /// Stops the timer and prints the duration of the task.
    ///
    /// # Examples
    ///
    /// ```
    /// use tea_timer::Timer;
    /// use std::thread::sleep;
    /// use std::time::Duration;
    ///
    /// let timer = Timer::new("Sleep Task");
    /// sleep(Duration::from_millis(100));
    /// timer.stop(); // This will print: "Sleep Task took 100.00ms" (approximately)
    /// ```
    #[inline]
    pub fn stop(self) {
        println!("{}", self.took_str());
    }

    /// Logs the elapsed time using the `log` crate.
    ///
    /// This method is only available when the `log` feature is enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "log")]
    /// # {
    /// use tea_timer::Timer;
    /// use std::thread::sleep;
    /// use std::time::Duration;
    ///
    /// let timer = Timer::new("Log Task");
    /// sleep(Duration::from_millis(10));
    /// timer.log(); // This will log using the log crate
    /// # }
    /// ```
    #[inline]
    #[cfg(feature = "log")]
    pub fn log(&self) {
        log::info!("{}", self.elapsed_str());
    }
}

#[inline]
pub fn took<F: FnOnce() -> R, R>(f: F, task_name: Option<&str>) -> R {
    let timer = Timer::new(task_name.unwrap_or(""));
    let result = f();
    timer.stop();
    result
}

#[inline]
pub fn ltook<F: FnOnce() -> R, R>(f: F, task_name: Option<&str>) -> R {
    let timer = Timer::new(task_name.unwrap_or(""));
    let result = f();
    timer.log();
    result
}

#[macro_export]
macro_rules! took {
    ($($tt:tt)*) => {
        {
            let timer = Timer::new("");
            let res = {$($tt)*};
            timer.stop();
            res
        }
    };
}

#[macro_export]
macro_rules! ltook {
    ($($tt:tt)*) => {
        {
            let timer = Timer::new("");
            let res = {$($tt)*};
            timer.log();
            res
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_timer_new() {
        let timer = Timer::new("Test Task");
        assert_eq!(timer.task_name, "Test Task");
    }

    #[test]
    fn test_timer_restart() {
        let mut timer = Timer::new("Task 1");
        timer.restart("Task 2");
        assert_eq!(timer.task_name, "Task 2");
    }

    #[test]
    fn test_timer_duration() {
        let timer = Timer::new("Duration Test");
        sleep(Duration::from_millis(10));
        assert!(timer.duration().as_millis() >= 10);
    }

    #[test]
    fn test_timer_duration_str() {
        let timer = Timer::new("Duration Str Test");
        sleep(Duration::from_millis(10));
        assert!(timer.duration_str().contains("ms"));
    }

    #[test]
    fn test_timer_default() {
        let timer = Timer::default();
        assert_eq!(timer.task_name, "");
    }

    #[test]
    fn test_took() {
        let result = took(
            || {
                sleep(Duration::from_millis(10));
                42
            },
            Some("Test Task"),
        );
        assert_eq!(result, 42);
    }

    #[test]
    fn test_took_macro() {
        let result = took! {
            sleep(Duration::from_millis(10));
            42
        };
        assert_eq!(result, 42);
    }
    // Note: We can't easily test the `stop` method as it prints to stdout.
    // In a real-world scenario, we might want to refactor to return the duration
    // instead of printing it, which would make it more testable.
}
