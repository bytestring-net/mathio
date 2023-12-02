/// # Tween
/// Very simple function for linear interpolation between 2 values.
/// 
/// * `slide` ranges from 0.0 to 1.0
pub fn tween(value_1: f32, value_2: f32, slider: f32) -> f32 {
    value_1 + (value_2 - value_1) * slider
}

/// # Tween Trait
/// Trait for implementing tweening logic for data types.
pub trait Tween {
    /// # Tween
    /// Function for linear interpolation between 2 values.
    /// 
    /// * `slide` ranges from 0.0 to 1.0
    fn tween(&mut self, other: &Self, slider: f32);
}