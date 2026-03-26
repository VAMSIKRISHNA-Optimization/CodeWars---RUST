fn frog_contest(n: u32) -> String 
{
    let Chris = (1..=n).sum::<u32>();
    let Tom   = (1..=Chris/2).sum::<u32>();
    let Cat   = (1..=Chris+Tom).sum::<u32>();
    
    format!("Chris ate {} flies, Tom ate {} flies and Cat ate {} flies", Chris, Tom, Cat)
}