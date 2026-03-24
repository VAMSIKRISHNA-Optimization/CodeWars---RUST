fn move_zeros(arr: &[u8]) -> Vec<u8> 
{
    let mut v = Vec::from(arr);
    let (mut rest, zeros): (Vec<u8>, Vec<u8>) = v.into_iter().partition(|&x| x != 0);
    rest.extend(zeros);
    rest
}