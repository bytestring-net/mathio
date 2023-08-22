/// # Periodical
/// Returns value that is normalized into a given period.
/// Allows you to easily clamp values with overflow.
///
/// The most common example would be normalizing degrees between 0 and 360.
/// 
/// # Examples
/// 
/// ```
///  let period = 360.0;
///  assert_eq!(315.0, periodical(period, -45.0));
///  assert_eq!(45.0, periodical(period, 45.0));
///  assert_eq!(0.0, periodical(period, 360.0));
///  assert_eq!(90.0, periodical(period, 450.0));
/// ```
pub fn periodical(period: f32, x: f32) -> f32 {
    let value = x % period;
    if value < 0.0 {
        value + period
    } else {
        value
    }
}

/// # Periodical Difference Short
/// Returns a difference between 2 periodical values.
/// Uses the shortest path.
///
/// The most common example would be getting a difference between 2 angles in degrees.
/// Because of the nature of trigonometry, you can sometimes get inner or outer angle depending on use case.
/// This function will always return the INNER angle.
/// 
/// # Examples
/// 
/// ```
///  let period = 360.0;
///  assert_eq!(120.0, periodical_difference_short(period, 0.0, 120.0));
///  assert_eq!(-90.0, periodical_difference_short(period, 0.0, 270.0)); //Always returns the inner angle
///  assert_eq!(45.0, periodical_difference_short(period, 45.0, 90.0));
///  assert_eq!(-45.0, periodical_difference_short(period, 90.0, 45.0));
/// ```
pub fn periodical_difference_short(period: f32, x1: f32, x2: f32) -> f32 {
    let difference = (periodical(period, x2) - periodical(period, x1)) % period;
    if difference > period / 2.0 {
        difference - period
    } else if difference < -period / 2.0 {
        difference + period
    } else {
        difference
    }
}

/// # Periodical Difference Long
/// Returns a difference between 2 periodical values.
/// Uses the longest path.
///
/// The most common example would be getting a difference between 2 angles in degrees.
/// Because of the nature of trigonometry, you can sometimes get inner or outer angle depending on use case. This function will always return the OUTER angle.
/// 
/// # Examples
/// 
/// ```
///  let period = 360.0;
///  assert_eq!(-240.0, periodical_difference_long(period, 0.0, 120.0)); //Always returns the outer angle
///  assert_eq!(270.0, periodical_difference_long(period, 0.0, 270.0));
///  assert_eq!(-315.0, periodical_difference_long(period, 45.0, 90.0));
///  assert_eq!(315.0, periodical_difference_long(period, 90.0, 45.0));
/// ```
pub fn periodical_difference_long(period: f32, x1: f32, x2: f32) -> f32 {
    let difference = (periodical(period, x2) - periodical(period, x1)) % period;
    if difference < 0.0 {
        difference + period
    } else if difference <= period / 2.0 {
        difference - period
    } else {
        difference
    }
}
