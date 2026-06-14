fn word_value(words: &[&str]) -> Vec<i32> 
{
    /* MY SOLUTION ( Time: O(N * M) , Space: O (1) ): 1704 ms*/ 
    // Also, the most efficient
    words
    .iter()
    .enumerate()
    .map(|(i, &s)| 
    {
        s
        .chars()
        .map(|c| 
        {
            if c == ' ' { 0 }
            else { (c.to_ascii_lowercase() as i32 - 'a' as i32) + 1 }
        }) 
        .sum::<i32>() * (i as i32 + 1)
    })
    .collect::<Vec<i32>>()
​
}
​