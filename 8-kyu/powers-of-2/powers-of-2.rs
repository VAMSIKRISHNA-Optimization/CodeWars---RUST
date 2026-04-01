fn powers_of_two(n: u8) -> Vec<u128> 
{
    (0..=n).map(|v| (2 as u128).pow(v as u32)).collect::<Vec<u128>>()
}