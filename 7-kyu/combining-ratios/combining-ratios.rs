fn merge_ratios(ratio1: &str, ratio2: &str) -> String 
{   
    let (r1, r2) = ratio1.split_once(':').unwrap();
    let (r3, r4) = ratio2.split_once(':').unwrap();
    
    let x: u64   = r1.trim().parse().unwrap();
    let y: u64   = r2.trim().parse().unwrap();
    let z: u64   = r3.trim().parse().unwrap();
    let w: u64   = r4.trim().parse().unwrap();
    
    let cd = gcd(gcd(x*z, y*z), y*w);
    
    format!("{}:{}:{}", x*z / cd, y*z / cd, y*w /cd)
}
​
​
fn gcd(mut a: u64, mut b: u64) -> u64 
{
    while b != 0 
    {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
​