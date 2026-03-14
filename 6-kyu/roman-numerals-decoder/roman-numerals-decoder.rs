fn roman_as_num(roman: &str) -> u64 
{
    let mut roman_as_iter = roman.chars()
                            .map(roman_individual_symbol_decoder)
                            .peekable();
    let mut total_sum = 0;
    
    while let Some(current) = roman_as_iter.next() 
    {
        match roman_as_iter.peek() 
        {
            Some(&next) if current < next => 
            {
                total_sum += next - current;
                roman_as_iter.next(); 
            }
            _ => total_sum += current, 
        }
    }
    total_sum
    
}
​
fn roman_individual_symbol_decoder(c: char) -> u64
{
    match c
    {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _   => panic!("Invalid CHARACTER"),
        
    }
}