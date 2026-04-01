fn bin_to_decimal(inp: &str) -> i32 
{
    //inp.parse::<i32>().expect("Sorry")
    inp.chars().rev().enumerate().map(|(ind,c)| c.to_digit(2).unwrap() as i32*2_i32.pow(ind as u32)).sum()
}