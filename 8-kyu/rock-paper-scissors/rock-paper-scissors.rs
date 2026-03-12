fn rps(p1: &str, p2: &str) -> &'static str  {
    match (p1, p2)
    {
        (val1, val2)  if val1 == val2           => return "Draw!",
        
        ("rock",val2) if val2 == "scissors"     => return "Player 1 won!", 
        ("rock",val2) if val2 == "paper"        => return "Player 2 won!", 
        
        ("paper",val2) if val2 == "rock"        => return "Player 1 won!", 
        ("paper",val2) if val2 == "scissors"    => return "Player 2 won!", 
                
        ("scissors",val2) if val2 == "paper"    => return "Player 1 won!", 
        ("scissors",val2) if val2 == "rock"     => return "Player 2 won!",
        
        _=> return "Null"
    }
}