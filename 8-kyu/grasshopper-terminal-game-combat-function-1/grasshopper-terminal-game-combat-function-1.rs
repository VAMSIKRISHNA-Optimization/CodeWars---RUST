fn combat(health: f32, damage: f32) -> f32 {
    0.0_f32.max(health - damage)
}