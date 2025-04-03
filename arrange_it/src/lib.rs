pub fn arrange_phrase(phrase: &str) -> String {
    let words: Vec<&str> = phrase.split_whitespace().collect();
    let mut processed: Vec<(u32, String)> = words
        .into_iter()
        .map(|word| {
            let mut num_str = String::new();
            let mut cleaned = String::new();
            for c in word.chars() {
                if c.is_ascii_digit() {
                    num_str.push(c);
                } else {
                    cleaned.push(c);
                }
            }
            let pos = num_str.parse::<u32>().unwrap();
            (pos, cleaned)
        })
        .collect();
    processed.sort_by_key(|&(pos, _)| pos);
    let sorted_words: Vec<String> = processed.into_iter().map(|(_, word)| word).collect();
    sorted_words.join(" ")
}