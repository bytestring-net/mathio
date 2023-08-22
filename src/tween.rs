/// # Tween
/// Very simple function for linear interpolation between 2 values.
/// 
/// * `slide` ranges from 0.0 to 1.0
pub fn tween(value_1: f32, value_2: f32, slide: f32) -> f32 {
    let diff = value_2 - value_1;
    value_1 + diff * slide
}