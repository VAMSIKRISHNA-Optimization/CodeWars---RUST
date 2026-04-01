fn bmi(weight: u32, height: f32) -> &'static str 
{
    let bmi_Val =  (weight as f32) / (height * height);
    
    match bmi_Val {
        val if val <= 18.5 => "Underweight",
        val if val <= 25.0 => "Normal",
        val if val <= 30.0 => "Overweight",
        _ => "Obese", // The catch-all arm for everything > 30.0
    }
    
}