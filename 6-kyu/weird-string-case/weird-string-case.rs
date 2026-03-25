fn to_weird_case(s: &str) -> String 
{
    s
    .split_whitespace()
    .map(|ss| 
        {
            ss
            .chars()
            .enumerate()
            .map(|(i,c)| 
                {
                    if i%2 == 0 
                    { 
                        c.to_uppercase().to_string()
                    }
                    else
                    {
                        c.to_lowercase().to_string()
                    }
                })
                .collect::<String>()
        })
    .collect::<Vec<String>>()
    .join(" ")
​
}