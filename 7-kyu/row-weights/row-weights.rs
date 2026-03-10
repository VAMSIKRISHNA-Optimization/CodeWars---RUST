fn row_weights(array: Vec<u32>) -> (u32, u32) 
{
    let mut w1: u32 = 0;
    let mut w2: u32 = 0;
    
    for (ind,val) in array.iter().enumerate()
    {
        if ind%2 == 0
        {
            w1 += *val;
        }
        else
        {
            w2 += *val;
        }
    }
    (w1, w2)
    
}