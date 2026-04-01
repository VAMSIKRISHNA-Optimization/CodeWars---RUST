fn count_sheep(n: u32) -> String 
{
   match n
    {
        0 => "".to_string(),
        n => (1..=n).map(|num| format!("{num} sheep...")).collect::<Vec<String>>().join("")
    }
}