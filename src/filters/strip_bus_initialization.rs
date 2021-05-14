use std::cmp::min;

pub fn strip_bus_initialization(values: &mut Vec<f64>, initialization_length: usize) -> f64 {
	let drain_len = min(initialization_length, values.len() - 1);
	values.drain(0..drain_len);
	1f64
}
