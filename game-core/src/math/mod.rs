pub mod vector;

pub fn pulse_value(min: f32, max: f32, inc_value: f32,) -> f32 {
    assert!(min < max);
    let mut value = inc_value.sin();
    value += 1.0;
    value /= 2.0;
    // value is now between 0 and 1

    let domain = max - min;
    value *= domain;
    value += min;
    value
}
