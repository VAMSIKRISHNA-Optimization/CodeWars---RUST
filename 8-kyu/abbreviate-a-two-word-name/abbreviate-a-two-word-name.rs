fn abbrev_name(name: &str) -> String 
{
    name.split_whitespace().map(|chunk| chunk.chars().next().unwrap().to_uppercase().to_string()).collect::<Vec<String>>().join(".") 
}