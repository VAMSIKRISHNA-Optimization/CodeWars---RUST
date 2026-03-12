fn angle(n: u32) -> u32 
{
  ((n.checked_sub(2)).unwrap()*180) 
}