fn tower_builder(n: usize) -> Vec<String> 
{
    (0..n).enumerate().map(|(i,v)| 
    format!("{}{}{}", " ".repeat(((n-1-i)*2)/2), "*".repeat(1+(2*i)),  " ".repeat(((n-1-i)*2)/2)) )
    .collect::<Vec<String>>()
}