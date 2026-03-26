fn solve(s: &str) -> String 
{
    let (ucc, lcc) =     s.chars().fold((0, 0), |(upper, lower), c| 
                                            {
                                                if c.is_uppercase() {
                                                    (upper + 1, lower)
                                                } else if c.is_lowercase() {
                                                    (upper, lower + 1)
                                                } else {
                                                    (upper, lower)
                                                }
                                            });
    match (ucc,lcc)
    {
        (ucc,lcc) if ucc>lcc  => s.to_uppercase(),
        (ucc,lcc) if ucc<lcc  => s.to_lowercase(),
        (ucc,lcc) if ucc==lcc => s.to_lowercase(),
        _ => unreachable!()
        
    }
    
}