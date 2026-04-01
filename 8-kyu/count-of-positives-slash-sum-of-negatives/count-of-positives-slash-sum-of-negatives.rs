fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> 
{
    if !input.is_empty()
    {
        let (pos_count, neg_sum) = input.iter().fold((0,0), |(mut pcount, mut nsum), &num| 
                                                                            { 
                                                                                if num > 0 {pcount+=1;}
                                                                                else {nsum -= -num;}
                                                                                (pcount, nsum)
                                                                            });
        vec![pos_count,neg_sum] 
    }
    else { vec![] }
}