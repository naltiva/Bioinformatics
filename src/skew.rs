pub fn skew(text: &str) -> Vec<&str> {
    let mut count_gc = vec![0 ; text.len()];
    for i in 0..text.len() {
        if &text[i] = "G" {
            count_gc[i + 1] = count_gc[i] + 1;
        }
        if &text[i] = "C" {
            count_gc[i + 1] = count_gc[i] - 1;
        }
    }

    return count_gc;
}