use std::time::Duration;

pub fn page_title(title: &str) -> String {
    format!("Falcon | {}", title)
}

pub fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs_f64();
    if secs < 1.0 {
        format!("{:.2}ms", secs * 1000.0)
    } else if secs < 60.0 {
        format!("{:.1}s", secs)
    } else if secs < 3600.0 {
        let mins = secs / 60.0;
        format!("{:.1}m", mins)
    } else {
        let hours = secs / 3600.0;
        format!("{:.1}h", hours)
    }
}