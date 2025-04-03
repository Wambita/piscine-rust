pub fn delete_and_backspace(s: &mut String) {
    let mut chars: Vec<char> = s.chars().collect();
    let mut i = chars.len() ;	
	while i  > 0 {
		i -= 1;
        if chars[i] == '+' {
			chars.remove(i); 
            if i < chars.len() {
                chars.remove(i);
            }
        }
    }	
    i = 0;
    while i < chars.len() {
        if chars[i] == '-' {
            if i > 0 {
                chars.remove(i - 1);
                i -= 1;
            }
            chars.remove(i); 
        }  else {
            i += 1;
        }
    }
	
    *s = chars.into_iter().collect();
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