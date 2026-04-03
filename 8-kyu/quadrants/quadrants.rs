fn quadrant(x: i32, y: i32) -> i32 
{
    match (x, y)
    {
        (x, y) if x >= 0 && y >= 0 => 1,
        (x, y) if x <  0 && y >= 0 => 2,
        (x, y) if x <  0 && y <  0 => 3,
        (x, y) if x >=0  && y <  0 => 4,
        (_,_) => unimplemented!()
    }
}