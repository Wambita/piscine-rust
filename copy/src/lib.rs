pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp = (c as f64).exp();
    let ln = (c.abs() as f64).ln();
    (c, exp, ln)
}

pub fn str_function(a: String) -> (String, String) {
    let exps: Vec<String> = a
        .split_whitespace()
        .map(|s| {
            let num: i32 = s.parse().unwrap();
            (num as f64).exp().to_string()
        })
        .collect();
    let result = exps.join(" ");
    (a, result)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let logs: Vec<f64> = b.iter().map(|&num| (num.abs() as f64).ln()).collect();
    (b, logs)
}
