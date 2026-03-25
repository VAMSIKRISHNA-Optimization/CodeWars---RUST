fn order(sentence: &str) -> String 
{
    if sentence.is_empty() { return "".to_string(); }
    let word_count = sentence.split_whitespace().count();
    let mut sent_num_vec: Vec<String> = vec![String::new(); word_count]; 
    
    sentence.split_whitespace().for_each(|wrd| 
    {
        let digit = wrd.chars()
                    .find(|c| c.is_numeric())
                    .and_then(|c| c.to_digit(10));
        sent_num_vec[(digit.unwrap()-1) as usize] = wrd.to_string();
    });
    sent_num_vec.join(" ")
}