fn min_value(mut digits: Vec<i32>) -> i32 
{
    digits.sort_unstable();
    digits.dedup();
    digits.iter().fold(0, |acc, &d| acc * 10 + d)
    
}