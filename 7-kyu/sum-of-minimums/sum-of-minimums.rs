fn sum_of_minimums(numbers: [[u8; 4]; 4]) -> u8 
{
    numbers.iter().fold(0, |acc, &sub_arr| acc + sub_arr.iter().min().unwrap())
}