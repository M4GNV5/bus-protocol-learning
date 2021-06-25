pub fn filter_not_in_range(values: &Vec<f64>, min: f64, max: f64, min_ratio: f64) -> f64 {
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
		1f64
	}
}
