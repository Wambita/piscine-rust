pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .into_iter()
        .map(|name| {
            name.split_whitespace()
                .map(|part| {
                    let initial = part.chars().next().unwrap();
                    format!("{}.", initial)
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect()
}