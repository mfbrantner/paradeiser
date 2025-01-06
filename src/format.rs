use std::time::Duration;

pub fn format_duration(dur: &Duration) -> String {
    let hours = dur.as_secs() / 3600;
    let minutes = (dur.as_secs() % 3600) / 60;
    let seconds = dur.as_secs() % 60;
    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
    else {
        format!("{:02}:{:02}", minutes, seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration_01() {
        let dur = Duration::from_secs(65);
        assert_eq!(format_duration(&dur), "01:05");
    }

    #[test]
    fn test_format_duration_02() {
        let dur = Duration::from_secs(3600);
        assert_eq!(format_duration(&dur), "01:00:00");
    }

    #[test]
    fn test_format_duration_03() {
        let dur = Duration::from_secs(3661);
        assert_eq!(format_duration(&dur), "01:01:01");
    }

    #[test]
    fn test_format_duration_04() {
        let dur = Duration::from_secs(0);
        assert_eq!(format_duration(&dur), "00:00");
    }

    #[test]
    fn test_format_duration_05() {
        let dur = Duration::from_secs(3599);
        assert_eq!(format_duration(&dur), "59:59");
    }
}