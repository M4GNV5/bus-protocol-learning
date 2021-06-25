pub fn filter_by_required_spread(values: &Vec<f64>, min: f64, max: f64, min_ratio: f64) -> f64 {
	/*
	let mut in_range: usize = 0;
	for val in values {
		if *val >= min && *val <= max {
			in_range += 1;
		}
	}

	let ratio = in_range as f64 / values.len() as f64;
	if ratio < min_ratio {
		0f64
	}
	else {
		ratio
	}
	*/

	let mut min_val = match values.iter().min_by(|a, b| a.partial_cmp(b).unwrap()) {
		None => return 0f64,
		Some(val) => *val,
	};
	let mut max_val = match values.iter().max_by(|a, b| a.partial_cmp(b).unwrap()) {
		None => return 0f64,
		Some(val) => *val,
	};

	if min_val < min {
		min_val = min;
	}
	if max_val > max {
		max_val = max;
	}

	let ratio = (max_val - min_val) / (max - min);
	if ratio < min_ratio {
		0f64
	}
	else {
		ratio
	}
}
