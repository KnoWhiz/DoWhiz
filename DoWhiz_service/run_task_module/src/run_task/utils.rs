pub(super) fn tail_string(input: &str, max_len: usize) -> String {
    let trimmed = input.trim();
    if trimmed.len() <= max_len {
        return trimmed.to_string();
    }
    let mut start = trimmed.len().saturating_sub(max_len);
    while start < trimmed.len() && !trimmed.is_char_boundary(start) {
        start += 1;
    }
    trimmed[start..].to_string()
}
