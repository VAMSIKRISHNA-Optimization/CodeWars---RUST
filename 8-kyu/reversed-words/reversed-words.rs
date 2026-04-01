fn reverse_words(words: &str) -> String {
    words.split_whitespace().rev().map(|wrd| wrd.to_string()).collect::<Vec<String>>().join(" ")
}