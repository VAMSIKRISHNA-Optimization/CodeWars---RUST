fn high(input: &str) -> &str 
{
    let mut best_word = "";
    let mut high_score = 0;
​
    for word in input.split_whitespace() {
        let score = word.chars().map(|c| (c as u32 & 31)).sum();
        if score > high_score {
            high_score = score;
            best_word  = word;
        }
    }
    best_word
}