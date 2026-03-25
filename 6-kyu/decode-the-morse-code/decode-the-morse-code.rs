mod preloaded;
use preloaded::MORSE_CODE; // MORSE_CODE is `HashMap<String, String>`. e.g. ".-" -> "A".
​
fn decode_morse(encoded: &str) -> String 
{
        encoded
        .trim() // Remove leading/trailing whitespace
        .split("   ") // Split by 3 spaces to get words
        .map(|word| {
            word.split_whitespace() // Split by 1 space to get characters
                .filter_map(|seq| MORSE_CODE.get(seq)) // Lookup and ignore None
                .cloned() // Convert &String to String
                .collect::<String>() // Combine characters into a word
        })
        .collect::<Vec<_>>() // Collect words into a list
        .join(" ") // Join words with a single space
}