pub fn filter_non_monotonic_values(values: &Vec<f64>, max_change: f64, monotonic_ratio: f64) -> f64 {
	let mut prev = values[0];
	let mut non_monotonic_changes = 0;
	for val in values {
		if (val - prev).abs() > max_change {
			non_monotonic_changes += 1;
		}
		prev = *val;
	}

	let ratio = non_monotonic_changes as f64 / values.len() as f64;
	if monotonic_ratio < 0f64 {
		ratio
	}
	else if ratio > (1f64 - monotonic_ratio) {
		0f64
	}
	else {
		1f64
	}
}
