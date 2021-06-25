use std::fs::File;
use serde::{Deserialize, Serialize};

use crate::common::{DynResult, BusExtraction};
#[derive(Serialize, Deserialize, Debug)]
struct BusValidation {
	header: u64,
	index: usize,
	size: usize,
	scale: f64,
	offset: f64,
}

pub struct ValidationResult {
	pub correct_count: usize,
	pub fail_count: usize,

	pub first_valid_index: usize,
	pub last_valid_index: usize,
}

fn is_validation_for(validation: &BusValidation, find: &BusExtraction) -> bool {
	find.header == validation.header
		&& find.index == validation.index
		&& find.size == validation.size
		&& find.scale == validation.scale
		&& find.offset == validation.offset
}

pub fn validate_finds(finds: &Vec<BusExtraction>, validation_file: &str) -> DynResult<ValidationResult> {
	let f = File::open(validation_file)?;
	let validations: Vec<BusValidation> = serde_json::from_reader(f)?;

	let mut missing = 0;
	let mut included = 0;
	for validation in &validations {
		if finds.iter().any(|find| is_validation_for(validation, find)) {
			included += 1;
		}
		else {
			println!("Missing msg {:03x} index {} size {} scale {}",
				validation.header, validation.index, validation.size, validation.scale);
			missing += 1;
		}
	}

	let mut min_index = finds.len();
	let mut max_index = 0;
	for (i, find) in finds.iter().enumerate() {
		if validations.iter().any(|validation| is_validation_for(validation, find)) {
			if i < min_index {
				min_index = i;
			}
			if i > max_index {
				max_index = i;
			}
		}
	}

	Ok(ValidationResult {
		correct_count: included,
		fail_count: missing,

		first_valid_index: min_index,
		last_valid_index: max_index + 1,
	})
}