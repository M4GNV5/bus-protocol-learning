use crate::common::{DynResult, BusExtractionError, BusExtraction};

pub fn strip_bus_initialization(target: &mut BusExtraction) -> DynResult<f64> {
	if target.raw_values.len() != target.values.len() {
		return BusExtractionError::create("Cannot strip initialization on an already stripped extraction");
	}

	let mut ones = 0;
	for _ in 0..target.size {
		ones <<= 1;
		ones |= 1;
	}

	let mut prefix = 0;
	for val in &target.raw_values {
		if *val != ones && *val != 0 {
			break;
		}
		prefix += 1;
	}

	if prefix != target.raw_values.len() {
		target.values.drain(0..prefix);
	}

	Ok(1f64)
}
