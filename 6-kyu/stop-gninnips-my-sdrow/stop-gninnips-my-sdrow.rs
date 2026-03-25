fn spin_words(words: &str) -> String {
    words.split_whitespace().map(|wrd| if wrd.len() >= 5 { wrd.chars().rev().collect::<String>()} else{ wrd.to_string()}).collect::<Vec<String>>().join(" ")
}