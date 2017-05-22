
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


//pub fn reverse_string(text: &str) -> &str {
//    let mut reverse = &text.chars().rev();
//    return reverse;
//}


//week2



pub fn skew(text: &str) -> Vec<&str> {
    let mut text = "CATGGGCATCGGCCATACGCC";
   // let mut skew_gc = vec![ ];
    let mut count_gc = vec![0 ; text.len()];
    for i in 0..text.len() {
        if &text[i] == "G" {
            count_gc == count_gc += 1;
        }
        if &text[i] == "C" {
            count_gc == count_gc -= 1;
            }
        }

    return count_gc;
}
