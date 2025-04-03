pub fn delete_and_backspace(s: &mut String) {
    let mut result = Vec::new();
    let mut skip_next = false;

    for c in s.chars() {
        if skip_next {
            skip_next = false;
            continue;
        }

        match c {
            '-' => {
                result.pop();
            }
            '+' => {
                skip_next = true;
            }
            _ => {
                result.push(c);
            }
        }
    }

    *s = result.into_iter().collect();
}



pub fn do_operations(v: &mut [String]) {
    for s in v {
        let op_pos = s.find(|c| c == '+' || c == '-').unwrap();
        let operator = s.chars().nth(op_pos).unwrap();
        let left_str = &s[..op_pos];
        let right_str = &s[op_pos + 1..];
        let left = left_str.parse::<i32>().unwrap();
        let right = right_str.parse::<i32>().unwrap();
        let result = match operator {
            '+' => left + right,
            '-' => left - right,
            _ => unreachable!(),
        };
        *s = result.to_string();
    }
}