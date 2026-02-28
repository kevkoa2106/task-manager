pub fn bytes_to_gb(bytes: u64) -> f32 {
    bytes as f32 / 1_000_000_000.0
}

pub fn memory_usage_percent(used: f64, total: f64) -> f64 {
    if total > 0.0 {
        (used / total) * 100.0
    } else {
        0.0
    }
}

pub fn format_uptime(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    format!("{hours}:{minutes}    Total in secs: {seconds}")
}

pub fn mhz_to_ghz(mhz: u64) -> f32 {
    mhz as f32 / 1000.0
}
