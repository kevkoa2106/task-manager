use task_manager::utilities::{bytes_to_gb, format_uptime, memory_usage_percent, mhz_to_ghz};

#[test]
fn test_bytes_to_gb() {
    assert_eq!(bytes_to_gb(0), 0.0);
    assert_eq!(bytes_to_gb(1_000_000_000), 1.0);
    assert_eq!(bytes_to_gb(16_000_000_000), 16.0);
    assert_eq!(bytes_to_gb(500_000_000), 0.5);
}

#[test]
fn test_memory_usage_percent_normal() {
    let result = memory_usage_percent(50.0, 100.0);
    assert!((result - 50.0).abs() < f64::EPSILON);
}

#[test]
fn test_memory_usage_percent_full() {
    let result = memory_usage_percent(100.0, 100.0);
    assert!((result - 100.0).abs() < f64::EPSILON);
}

#[test]
fn test_memory_usage_percent_zero_total() {
    assert_eq!(memory_usage_percent(50.0, 0.0), 0.0);
}

#[test]
fn test_memory_usage_percent_negative_total() {
    assert_eq!(memory_usage_percent(50.0, -1.0), 0.0);
}

#[test]
fn test_format_uptime_zero() {
    assert_eq!(format_uptime(0), "0:0    Total in secs: 0");
}

#[test]
fn test_format_uptime_one_hour() {
    assert_eq!(format_uptime(3600), "1:0    Total in secs: 3600");
}

#[test]
fn test_format_uptime_mixed() {
    // 2 hours, 30 minutes = 9000 seconds
    assert_eq!(format_uptime(9000), "2:30    Total in secs: 9000");
}

#[test]
fn test_format_uptime_minutes_only() {
    // 45 minutes = 2700 seconds
    assert_eq!(format_uptime(2700), "0:45    Total in secs: 2700");
}

#[test]
fn test_format_uptime_ignores_remaining_seconds() {
    // 1 hour, 1 minute, 1 second = 3661
    assert_eq!(format_uptime(3661), "1:1    Total in secs: 3661");
}

#[test]
fn test_mhz_to_ghz() {
    assert_eq!(mhz_to_ghz(0), 0.0);
    assert_eq!(mhz_to_ghz(1000), 1.0);
    assert_eq!(mhz_to_ghz(3500), 3.5);
    assert_eq!(mhz_to_ghz(2400), 2.4);
}
