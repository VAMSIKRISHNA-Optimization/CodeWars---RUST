fn collinearity(x1: i32, y1: i32, x2: i32, y2: i32) -> bool 
{
    let vals = (x1, y1, x2, y2);
    
    match vals
    {
        (0, _, 0, _) => true,
        (_, 0, _, 0) => true,
        (0, 0, 0, 0) => true,
        (0, 0, _, _) => true,
        (_, _, 0, 0) => true,
        
        (_, _, 0, _) => false,
        (0, _, 0, _) => false,
        (0, _, _, 0) => false,
        
        (a,b,c,d) if a*d - b*c == 0  => true,
        (a,b,c,d) if a*d - b*c != 0  => false,
        
        _=>panic!("ERROR"),
    }
}