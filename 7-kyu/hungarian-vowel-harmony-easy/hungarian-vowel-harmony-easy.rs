fn dative(word: &str) -> String 
{
    /* MY SOLUTION ( Time: O(N) , Space: O (1) ): 1201 ms*/ 
    for c in word.chars().rev() 
    {
        match c 
        {
            'e' | 'é' | 'i' | 'í' | 'ö' | 'ő' | 'ü' | 'ű' => 
            {
                return format!("{}{}", word, "nek");
            }
            'a' | 'á' | 'o' | 'ó' | 'u' | 'ú' => 
            {
                return format!("{}{}", word, "nak");
            }
​
            _ => continue,
        }
    }
    
    word.to_string()
    
//     /* THE MOST EFFICIENT SOLUTION ( Time: O(N) , Space: O(1) ): 1220 ms*/ 
//     for c in word.chars().rev() {
//         // Match the first vowel we hit from the back
//         let suffix = match c {
//             'e' | 'é' | 'i' | 'í' | 'ö' | 'ő' | 'ü' | 'ű' => "nek",
//             'a' | 'á' | 'o' | 'ó' | 'u' | 'ú'             => "nak",
//             _ => continue,
//         };
​
//         // Pre-allocate the exact space required: original length + 3 bytes for suffix
//         let mut result = String::with_capacity(word.len() + 3);
//         result.push_str(word);
//         result.push_str(suffix);
//         return result;
//     }
​
//     word.to_string()
}