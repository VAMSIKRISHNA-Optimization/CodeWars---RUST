fn smash(words: &[&str]) -> String {
    words.iter().map(|wrd| wrd.to_string()).collect::<Vec<String>>().join(" ")
}