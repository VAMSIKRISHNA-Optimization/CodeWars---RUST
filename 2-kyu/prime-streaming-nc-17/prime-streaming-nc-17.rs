//     }
// }
​
// // Implementation of Iterator trait fir PrimeStreamIterator_1Million
// impl Iterator for PrimeStreamIterator_1Million 
// {
//     type Item = u32;
​
//     fn next(&mut self) -> Option<Self::Item> 
//     {
//         if self.current_index < self.vec_of_primes.len() 
//         {
//             let prime_num = self.vec_of_primes[self.current_index];
//             self.current_index += 1;
//             Some(prime_num)
//         } 
//         else 
//         {
//             None 
//         }
//     }
// }
​
// fn stream() -> impl Iterator<Item = u32> 
// {
//     PrimeStreamIterator_1Million::new(1_000_000_000) 
// }
​