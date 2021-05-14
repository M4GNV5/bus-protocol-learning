pub fn filter_non_linear_growth(values: &Vec<f64>) -> f64 {
	let mut prev = values[0];
	for val in values {
		if *val < prev {
			return 0f64
		}
		prev = *val;
	}

	1f64
}