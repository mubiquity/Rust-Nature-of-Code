#![deny(missing_docs)]
//! Simple utility functions useful all over the place

// TODO: Make this generic
/// Maps a value from the range (in_min, in_max) to (out_min, out_max)
pub fn map_to_range(value: &mut f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) {
    *value = (((*value - in_min) * (out_max - out_min)) / (in_max - in_min)) + out_min
}