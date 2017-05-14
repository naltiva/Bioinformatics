
pub fn pattern_count(text: &str, pattern: &str) -> i32 {
    let mut count = 0;
    for i in 0 ..(text.len() - pattern.len() + 1)  {
        let slice = &text[i..(i + pattern.len())];
        if slice == pattern {
            count += 1;
        }
    }
    return count;
}
