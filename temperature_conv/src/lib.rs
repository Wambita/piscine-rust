pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) / (9.0 / 5.0)
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * (9.0 / 5.0) + 32.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fahrenheit_to_celsius() {
        let test_cases = vec![
            (32.0, 0.0),
            (212.0, 100.0),
            (-40.0, -40.0),
            (-459.67, -273.15),
            (98.6, 37.0),
        ];

        for (f, expected_c) in test_cases {
            let result = fahrenheit_to_celsius(f);
            assert!(
                (result - expected_c).abs() < 1e-10,
                "fahrenheit_to_celsius({}) = {}, expected {}",
                f,
                result,
                expected_c
            );
        }
    }

    #[test]
    fn test_celsius_to_fahrenheit() {
        let test_cases = vec![
            (0.0, 32.0),
            (100.0, 212.0),
            (-40.0, -40.0),
            (37.0, 98.6),
            (-273.15, -459.67),
            (25.0, 77.0),
        ];

        for (c, expected_f) in test_cases {
            let result = celsius_to_fahrenheit(c);
            assert!(
                (result - expected_f).abs() < 1e-10,
                "celsius_to_fahrenheit({}) = {}, expected {}",
                c,
                result,
                expected_f
            );
        }
    }
}