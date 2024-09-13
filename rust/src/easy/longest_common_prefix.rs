pub fn longest_common_prefix(words: Vec<&str>) -> String {
    if words.is_empty() {
        return "".to_string();
    }

    let mut prefix = words[0].to_string();

    for w in words.iter().skip(1) {
        while !w.starts_with(&prefix) {
            prefix.pop();
            if prefix.is_empty() {
                return "".to_string();
            }
        }
    }

    return prefix;
}
