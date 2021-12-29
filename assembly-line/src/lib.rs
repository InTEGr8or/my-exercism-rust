// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
// #![warn(missing_docs)]
#![crate_name = "assembly_line"]

/// Gets the rate based on the speed
pub fn production_rate_per_hour(speed: u8) -> f64 {
    221.0 
    * f64::from(speed) 
    * match speed {
        0 => 0.0,
        1..=4 => 1.0,
        5..=8 => 0.9,
        9 | 10 => 0.77,
        _ => panic!("Speed input value is out of range"), 
    }
}

/// Returns the items-per-minute based on the speed
pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32 
}
