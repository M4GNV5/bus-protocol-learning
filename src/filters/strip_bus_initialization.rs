pub fn strip_bus_initialization(values: &mut Vec<f64>, initialization_length: usize) -> f64 {
	values.drain(0..initialization_length);
	1f64
}
