fn camel_case(str: &str) -> String 
{
    str.split_whitespace()
    .map(|wrd| format!("{}{}", wrd.chars().nth(0).expect("FAIL").to_uppercase(), wrd.chars().skip(1).collect::<String>()) )
    .collect::<Vec<_>>()
    .join("")
}