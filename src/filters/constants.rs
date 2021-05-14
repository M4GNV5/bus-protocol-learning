pub fn filter_constant_values(values: &Vec<f64>, require_constant: bool) -> f64 {
	let val = values[0];
	let mut is_constant = true;
	for curr in values {
		if val != *curr {
			is_constant = false;
			break;
		}
	}

	if is_constant == require_constant {
		1f64
	}
	else {
		0f64
	}
}
