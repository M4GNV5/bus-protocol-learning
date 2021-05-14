pub fn filter_non_linear_growth(values: &Vec<f64>, is_falling: bool) -> f64 {
	let mut prev = values[0];
	for val in values {
		if !is_falling && *val < prev {
			return 0f64
		}
		if is_falling && *val > prev {
			return 0f64
		}
		prev = *val;
	}

	1f64
}
