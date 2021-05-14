const COMMON_SCALES: &'static [i32] = &[
	1,
	2,
	4,
	8,
	16,
	32,
	64,
	10,
	100,
	1000,
];

fn calculate_distance(values: &Vec<f64>, target_min: f64, target_max: f64, scale: f64) -> f64 {
	let mut dist = 0f64;
	for val in values {
		let curr = *val * scale;
		if curr < target_min {
			dist += target_min - curr;
		} 
		else if curr > target_max {
			dist += curr - target_max;
		}
	}

	dist / values.len() as f64
}

pub fn get_best_common_scale(values: &Vec<f64>, target_min: f64, target_max: f64) -> (f64, f64) {
	let mut best_distance = target_max;
	let mut best_scale = 0f64;

	for i in 0..COMMON_SCALES.len() {
		for j in i..COMMON_SCALES.len() {
			let scale = (COMMON_SCALES[i] * COMMON_SCALES[j]) as f64;
			let dist = calculate_distance(values, target_min, target_max, scale);
			if dist < best_distance {
				best_distance = dist;
				best_scale = scale;
			}

			let scale_inverted = 1f64 / scale;
			let dist2 = calculate_distance(values, target_min, target_max, scale_inverted);
			if dist2 < best_distance {
				best_distance = dist2;
				best_scale = scale_inverted;
			}
		}
	}

	let confidence = 1f64 - best_distance as f64 / target_max as f64;
	(confidence, best_scale)
}
