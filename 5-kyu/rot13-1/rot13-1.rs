fn rot13(message: &str) -> String
{
    message.chars()
           .map(|c| 
           {
               if c.is_ascii_lowercase()
               {
                   (((c as u8 - 97 + 13) % 26) + 97) as char
               }
               else if c.is_ascii_uppercase()
               {
                   (((c as u8 - 65 + 13) % 26) + 65) as char
               }
               else
               {
                   c
               }
           })
           .collect::<String>()
}