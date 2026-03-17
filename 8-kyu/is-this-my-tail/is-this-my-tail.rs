fn correct_tail(body: &str, tail: char) -> bool 
{
    /* My Solution ( Time: O(n), Space: O(1) ) : 1532 ms */
    body.chars().last() == Some(tail)
    
    /* Most Efficient ( Time: O(1), Space: O(1) ) : 1505 ms */
//     body.ends_with(tail)
}
​