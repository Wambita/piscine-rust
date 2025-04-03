pub fn km_per_hour_to_meters_per_second(km_h: f64) -> f64 {
    km_h / 3.6
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_zero() {
        assert_eq!(km_per_hour_to_meters_per_second(0.0), 0.0);
    }

    #[test]
    fn test_36_kmh() {
        assert_eq!(km_per_hour_to_meters_per_second(36.0), 10.0);
    }

    #[test]
    fn test_100_kmh() {
        let result = km_per_hour_to_meters_per_second(100.0);
        assert!((result - 27.77777777777778).abs() < 0.0001);
    }

    #[test]
    fn test_negative_speed() {
        assert_eq!(km_per_hour_to_meters_per_second(-90.0), -25.0);
    }
}