fn most_frequent_word(text: &str) -> (String, usize) {
    
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut word_counts: Vec<(&str, usize)> = Vec::new();

    for &word in &words {
        let mut found = false;
        for count in &mut word_counts {
            if count.0 == word {
                count.1 += 1;
                found = true;
                break;
            }
        }
        if !found {
            word_counts.push((word, 1));
        }
    }

    let mut max_word = String::new();
    let mut max_count = 0;

    for &(word, count) in &word_counts {
        if count > max_count {
            max_count = count;
            max_word = word.to_string();
        }
    }

    (max_word, max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}