
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


pub fn frequent_words(text: &str, k: usize) -> Vec<&str> {
    let mut frequent = vec![ ];
    let mut count_pattern = vec![0 ; text.len() - k];
    for i in 0..(text.len() - k) {
        let read_pattern = &text[i..(i + k)];
        let c = pattern_count(&text, &read_pattern);
        count_pattern[i] = c;
    }
    let mut max = count_pattern.iter().max().unwrap();
    for i in 0..count_pattern.len() {
        if &count_pattern[i] == max {
            let word = &text[i..(i + k)];
            if !frequent.contains(&word){
                frequent.push(&word);
            }

        }
    }

    return frequent;
}