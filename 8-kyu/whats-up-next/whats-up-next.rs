fn next_item<T: PartialEq<T> + Clone>(slice: &[T], find: T) -> Option<T> 
{
    let mut iter = slice.iter();
    
    // 1. Advance the iterator until we find the target
    iter.find(|&x| *x == find)?;
    
    // 2. Call next() again to get the following item
    iter.next().cloned()
}