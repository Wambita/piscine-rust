pub fn delete_prefix<'a>(prefix: &'a str, s: &'a str) -> Option<&'a str> {
    if s.starts_with(prefix) {
        Some(&s[prefix.len()..])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_prefix() {
        assert_eq!(delete_prefix("ab", "abcdefghijklmnop"), Some("cdefghijklmnop"));
        assert_eq!(delete_prefix("x", "abcdefghijklmnop"), None);
        assert_eq!(delete_prefix("", "abcdefghijklmnop"), Some("abcdefghijklmnop"));
        assert_eq!(delete_prefix("abc", ""), None);
    }
}
