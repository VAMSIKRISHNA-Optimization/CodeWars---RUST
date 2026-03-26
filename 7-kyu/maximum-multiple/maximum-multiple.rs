fn max_multiple(divisor: u32, bound: u32) -> u32 
{
   if bound%divisor == 0 { return bound; }
   else { return (bound/divisor) * divisor;}
}