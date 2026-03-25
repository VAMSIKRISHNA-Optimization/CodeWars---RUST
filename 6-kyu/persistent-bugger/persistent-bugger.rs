fn persistence(num: u64) -> u64 
{
    if num < 10 { return 0; }
    
    let mut count: u64   = 0;
    let mut num_cpy: u64 = num; 
    
    while num_cpy > 9
    {
        count+=1;
        num_cpy = digit_product(num_cpy);
    }
    
    count
    
}
​
fn digit_product(mut n: u64) -> u64 
{
    if n == 0 { return 0; } // Product of the single digit '0' is 0
    let mut product = 1;
    while n > 0 {
        product *= n % 10;
        n /= 10;
    }
    product
}