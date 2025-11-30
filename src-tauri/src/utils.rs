/// Formate une durée en minutes en une chaîne lisible "Hh Mm"
/// 
/// # Examples
/// ```
/// use app_lib::utils::format_duration;
/// assert_eq!(format_duration(65), "1h 05m");
/// assert_eq!(format_duration(30), "30m");
/// ```
pub fn format_duration(minutes: i32) -> String {
    if minutes < 60 {
        return format!("{}m", minutes);
    }
    
    let hours = minutes / 60;
    let mins = minutes % 60;
    format!("{}h {:02}m", hours, mins)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(0), "0m");
        assert_eq!(format_duration(30), "30m");
        assert_eq!(format_duration(60), "1h 00m");
        assert_eq!(format_duration(65), "1h 05m");
        assert_eq!(format_duration(125), "2h 05m");
    }
}
