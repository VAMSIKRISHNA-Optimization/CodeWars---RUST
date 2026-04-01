fn better_than_average(class_points: &[u16], your_points: u16) -> bool {
    if (class_points.iter().sum::<u16>()) / (class_points.len() as u16) < your_points { true } else { false }
}