pub fn delete_and_backspace(s: &mut String) {
    let mut result = Vec::new();
    let mut skip_next = false;
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if skip_next {
            skip_next = false;
            i += 1;
            continue;
        }
        let c = chars[i];
        if c == '-' {
            result.pop();
        } else if c == '+' {
            skip_next = true;
        } else {
            result.push(c);
        }
        i += 1;
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