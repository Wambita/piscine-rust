pub fn sum(a: u8, b: u8) -> u8 {
    a.checked_add(b).expect("ERROR: attempt to add with overflow")
}

pub fn diff(a: i16, b: i16) -> i16 {
    a.checked_sub(b).expect("ERROR: attempt to subtract with overflow")
}

pub fn pro(a: i8, b: i8) -> i8 {
    a.checked_mul(b).expect("ERROR: attempt to multiply with overflow")
}

pub fn quo(a: f32, b: f32) -> f32 {
    if b != 0.0 {
        a / b
    } else {
        panic!("ERROR: division by zero")
    }
}

pub fn rem(a: f32, b: f32) -> f32 {
    if b != 0.0 {
        a % b
    } else {
        panic!("ERROR: division by zero")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(234, 2), 236);
    }
    
    #[test]
    #[should_panic(expected = "ERROR: attempt to add with overflow")]
    fn test_panic_sum() {
        sum(255, 1);
    }

    #[test]
    fn test_diff() {
        assert_eq!(diff(234, 2), 232);
    }
    
    #[test]
    #[should_panic(expected = "ERROR: attempt to subtract with overflow")]
    fn test_panic_diff() {
        diff(-32768, 32766);
    }

    #[test]
    fn test_pro() {
        assert_eq!(pro(23, 2), 46);
    }
    
    #[test]
    #[should_panic(expected = "ERROR: attempt to multiply with overflow")]
    fn test_panic_pro() {
        pro(-128, 2);
    }

    #[test]
    fn test_quo() {
        assert_eq!(quo(22.0, 2.0), 11.0);
    }

    #[test]
    fn test_rem() {
        assert_eq!(rem(22.0, 2.0), 0.0);
    }
}
