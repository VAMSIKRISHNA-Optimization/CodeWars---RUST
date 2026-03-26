fn trailing_zeros(n: i32) -> u32 
{
//     let bin_rep = format!("{:b}", n);
//     bin_rep.chars().rev().take_while(|&c| c == '0').count() as u32
    
    n.trailing_zeros()
}