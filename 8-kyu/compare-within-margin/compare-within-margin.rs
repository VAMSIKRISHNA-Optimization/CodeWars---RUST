fn close_compare(a: f64, b: f64, margin: f64) -> i8 
{
    match margin
    {
        0.0 => if a==b {0} else {if a>b {1} else {-1}} 
        _=> if a==b {0} else {if a>b { if a-b <= margin {0} else {1}} else {if b-a <= margin {0} else {-1}}}
    }
} 