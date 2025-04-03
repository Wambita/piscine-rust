pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * (5.0 / 9.0)
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * (9.0 / 5.0)) + 32.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert!((fahrenheit_to_celsius(-459.67) - (-273.15)).abs() < 1e-7);
        assert!((fahrenheit_to_celsius(32.0) - 0.0).abs() < 1e-7);
        assert!((fahrenheit_to_celsius(212.0) - 100.0).abs() < 1e-7);
        assert!((fahrenheit_to_celsius(20.0) - (-6.666666666666666)).abs() < 1e-7); // Fixed failing test
    }

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert!((celsius_to_fahrenheit(0.0) - 32.0).abs() < 1e-7);
        assert!((celsius_to_fahrenheit(100.0) - 212.0).abs() < 1e-7);
        assert!((celsius_to_fahrenheit(-40.0) - (-40.0)).abs() < 1e-7);
    }
}
