// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    const VAL: u16 = 221;
    let result: u16 = speed as u16 * VAL;
    match speed {
        1..=4 => return result as f64,
        5..=8 => return result as f64 * 0.9,
        9..=10 => return result as f64 * 0.77,
        _ => return 0.0,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    return (production_rate_per_hour(speed) / 60.0) as u32;
}
