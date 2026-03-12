pub fn gould() -> impl Iterator<Item = u8> 
{
    let mut Odd_Count = vec![0];
    
    for val in 1..1000000
    {
        Odd_Count.push(((val as u32).count_ones())as u8);
    }
    
    Odd_Count.into_iter()
}