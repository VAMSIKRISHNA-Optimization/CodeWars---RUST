fn expressions_matter(a: u64, b: u64, c: u64) -> u64 
{
    let mut combos: [u64;4] = [0;4];
    
    combos[0] = a + b + c;
    combos[1] = (a + b) * c;
    combos[2] = a * (b + c);
    combos[3] = a * b * c;
    
    //println!("{:?}", combos);
    
    *combos.iter().max().unwrap()
}