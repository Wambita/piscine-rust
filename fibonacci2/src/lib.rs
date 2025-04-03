pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut prev_prev = 0;
    let mut prev = 1;
    let mut current = 0;

    for _ in 2..=n {
        current = prev_prev + prev;
        prev_prev = prev;
        prev = current;
    }

    current
}