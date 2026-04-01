fn points(games: &[String]) -> u32 
{
    games.iter().map(|ss| 
                    {
                        let xp = ss.chars().nth(0).expect("FAIL").to_digit(10).unwrap();
                        let yp = ss.chars().nth(2).expect("FAIL").to_digit(10).unwrap();
                        let p  = if xp>yp {3} else if xp<yp{0} else {1};
                        p             
                    }).sum()
}
​