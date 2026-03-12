fn find_nb(m: u64) -> i32 
{
    let mut num = 1;
    let mut sum = 0;
    
    while sum <= m
    {
        sum += num*num*num;
        if sum == m { return num as i32;}
        num += 1;
        
        
    }
    -1
}