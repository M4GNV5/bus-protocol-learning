pub fn filter_non_oscillating_sensors(values: &Vec<f64>, min_ratio: f64) -> f64 {
	let mut prev = values[0];
	let mut change_count = 0;
	let mut oscillating_count = 0;
	let mut curr_oscillating = 0;
	for val in values {
		if *val != prev {
			curr_oscillating += 1;
		}
		else if curr_oscillating == 1 {
			change_count += 1;
			curr_oscillating = 0;
		}
		else if curr_oscillating > 1 {
			oscillating_count += 1;
			change_count += 1;
			curr_oscillating = 0;
		}

		prev = *val;
	}

	let ratio = oscillating_count as f64 / change_count as f64;
	if ratio > min_ratio {
		1f64
	}
	else {
		0f64
	}
}
