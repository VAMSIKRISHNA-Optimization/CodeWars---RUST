fn closing_in_sum(n: u64) -> u32 
{
    let dig_vec: Vec<u32> = n.to_string()
                                .chars()
                                .map(|d| d.to_digit(10).unwrap())
                                .collect();
    dig_vec
    .iter()
    .take((dig_vec.len() + 1) / 2)
    .zip(dig_vec.iter().rev()) 
    .enumerate()
    .map(|(i, (&f, &l))| 
        {
            if i == dig_vec.len() - 1 - i { f } else { f * 10 + l }
        })
    .sum()
}