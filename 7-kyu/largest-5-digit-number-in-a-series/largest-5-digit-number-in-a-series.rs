fn largest_five_digit_number(num: &str) -> u32 
{
    num.as_bytes()
    .windows(5)
    .map(|w| 
        {
            std::str::from_utf8(w).unwrap().parse::<u32>().unwrap()
        })
        .max()
        .unwrap_or(0)
}