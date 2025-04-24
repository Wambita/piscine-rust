pub fn add_curry(num: i32) -> Box<dyn Fn(i32) -> i32 {
    Box::new(move |x: i32| x + num)
}