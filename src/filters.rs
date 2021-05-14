use serde::{Deserialize, Serialize};

use crate::common::{DynResult, BusExtraction};

use common_scale::get_best_common_scale;
use linear_growth::filter_non_linear_growth;
use oscillating_sensor::filter_non_oscillating_sensors;

pub mod common_scale;
pub mod linear_growth;
pub mod oscillating_sensor;

#[derive(Serialize, Deserialize, Debug)]
pub enum Filter {
	CommonScale {
		min: f64,
		max: f64,
	},
	StrictLinear {
		is_falling: bool,
	},
	OscillatingSensor {
		min_oscillating_ratio: f64,
	},
}

pub const FILTER_SAMPLES: &'static [Filter] = &[
	Filter::CommonScale {
		min: 42.0,
		max: 1337.0,
	},
	Filter::StrictLinear {
		is_falling: false,
	},
	Filter::OscillatingSensor {
		min_oscillating_ratio: 0.5,
	},
];

pub fn apply_filter(filter: &Filter, target: &mut BusExtraction) -> DynResult<()> {
	let confidence = match filter {
		Filter::CommonScale {min, max} => {
			let (confidence, scale) = get_best_common_scale(&target.values, *min, *max);
			target.scale = scale;
			for val in &mut target.values {
				*val *= scale;
			}
			confidence
		},
		Filter::StrictLinear {is_falling} => filter_non_linear_growth(&target.values),
		Filter::OscillatingSensor {min_oscillating_ratio} => filter_non_oscillating_sensors(&target.values),
	};

	target.confidence *= confidence;

	Ok(())
}
