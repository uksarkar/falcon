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

pub fn ellipse_text(s: &str, max_chars: usize) -> String {
    let mut end_idx = 0;
    for (i, (idx, _)) in s.char_indices().enumerate() {
        if i == max_chars {
            break;
        }
        end_idx = idx + 1; // +1 to include the current character
    }

    if s.chars().count() > max_chars {
        format!("{}...", &s[..end_idx])
    } else {
        s.to_string()
    }
}
