fn square_or_square_root(arr: &[u32]) -> Vec<u32> 
{
    arr.iter().map(|&n| 
    {
        let root = (n as f64).sqrt() as u32;
        if root * root == n
        {
            root
        }
        else
        {
            n*n
        }
    })
    .collect::<Vec<u32>>()
}