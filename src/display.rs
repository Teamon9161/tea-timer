const NANOS_PER_MILLI: u128 = 1_000_000;
const NANOS_PER_MICRO: u128 = 1_000;

pub(crate) fn format_duration(duration: std::time::Duration) -> String {
    if duration.as_secs() > 0 {
        format!("{:.2}s", duration.as_secs_f64())
    } else {
        let nanos = duration.as_nanos();
        if nanos > NANOS_PER_MILLI {
            format!("{:.2}ms", nanos as f64 / NANOS_PER_MILLI as f64)
        } else if nanos > NANOS_PER_MICRO {
            format!("{:.2}µs", nanos as f64 / NANOS_PER_MICRO as f64)
        } else {
            format!("{}ns", nanos)
        }
    }
}
