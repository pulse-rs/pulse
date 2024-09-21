use std::time::{Duration, Instant};

pub fn format_time(time: Instant) -> String {
    let time = Instant::now().duration_since(time);

    let millis = time.as_millis();
    let seconds = millis / 1000;
    let milliseconds = millis % 1000;

    if seconds > 0 {
        format!("{}s {}ms", seconds, milliseconds)
    } else {
        format!("{}ms", milliseconds)
    }
}
