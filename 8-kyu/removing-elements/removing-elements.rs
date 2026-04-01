fn remove_every_other(arr: &[u8]) -> Vec<u8> {
    arr.iter().enumerate().filter(|&(k,_)| (k+1)%2 !=0 ).map(|(_, &val)| val).collect::<Vec<u8>>()
}