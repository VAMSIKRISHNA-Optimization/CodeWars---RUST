fn shark(pontoon_distance: f64, shark_distance: f64, you_speed: f64, shark_speed: f64, dolphin: bool) -> String 
{
    /* My Solution (Time Complexity: O(1), Space Complexity: O(1)) : 1392 ms */
    let my_time     =  pontoon_distance / you_speed;
    let shart_time  =  if dolphin { shark_distance/(shark_speed*0.5)} else {shark_distance/shark_speed};
    
    if shart_time > my_time
    {
        "Alive!".to_string()
    }
    else
    {
        "Shark Bait!".to_string()
    }
    
    /* The Most Effective Solution (Time Complexity: O(1), Space Complexity: O(1)) : 1247 ms */
    // If dolphin is true, shark speed is halved
//     let effective_shark_speed = if dolphin { shark_speed * 0.5 } else { shark_speed };
​
//     // Use cross-multiplication: (d1 / s1 < d2 / s2) is equivalent to (d1 * s2 < d2 * s1)
//     // This avoids slow floating-point division.
//     if pontoon_distance * effective_shark_speed < shark_distance * you_speed {
//         "Alive!".to_string()
//     } else {
//         "Shark Bait!".to_string()
//     }
​
}