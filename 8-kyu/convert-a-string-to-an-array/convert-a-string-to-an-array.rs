fn string_to_array(s: &str) -> Vec<String> 
{
    s.split_whitespace().map(|sub_str| sub_str.to_string()).collect::<Vec<String>>()
}