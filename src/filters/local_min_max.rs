pub fn require_local_min_max(values: &Vec<f64>, window_size: usize, required_count: usize) -> f64 {
	let mut prev = values[0];
	let mut rise_count = 0;
	let mut fall_count = 0;
	let mut minmax_count = 0;

	for val in values {
		if *val > prev {
			rise_count += 1;
			if rise_count > window_size / 2 {
				if fall_count > window_size {
					minmax_count += 1;
				}
				fall_count = 0;
			}
		}
		else {
			fall_count += 1;
			if fall_count > window_size / 2 {
				if rise_count > window_size {
					minmax_count += 1;
				}
				rise_count = 0;
			}
		}

		prev = *val;
	}

	/*let mut minmax_count = 0;
	let mut had_minmax = false;
	for i in 0..values.len() {
		if i < window_size {
			continue;
		}

		let slope = values[i] / values[i - window_size];
		if slope < 0.01 {
			if !had_minmax {
				minmax_count += 1;
				had_minmax = true;
			}
		}
		else {
			had_minmax = false;
		}
	}*/

	if minmax_count < required_count {
		0f64
	}
	else {
		1f64
	}
}
