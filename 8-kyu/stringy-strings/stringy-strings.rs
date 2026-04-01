fn stringy(size: usize) -> String 
{
    (1..=size).enumerate().map(|(ind,val)| 
                                            {
                                                if ind % 2 == 0
                                                {
                                                    "1".to_string()
                                                }
                                                else
                                                {
                                                    "0".to_string()
                                                }
                                            })
                                            .collect::<Vec<String>>()
                                            .join("")
}