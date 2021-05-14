use serde::{Deserialize, Serialize};

use crate::common::{DynResult, BusExtraction};

use common_scale::get_best_common_scale;
use linear_growth::filter_non_linear_growth;
use oscillating_sensor::filter_non_oscillating_sensors;
use constants::filter_constant_values;
use strip_bus_initialization::strip_bus_initialization;

mod common_scale;
mod linear_growth;
mod oscillating_sensor;
mod constants;
mod strip_bus_initialization;

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
	Constant {
		require_constant: bool,
	},
	StripInitialization {
		initialization_length: usize,
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
	Filter::Constant {
		require_constant: false,
	},
	Filter::StripInitialization {
		initialization_length: 200,
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
		Filter::Constant {require_constant} => filter_constant_values(&target.values, *require_constant),
		Filter::StripInitialization {initialization_length} =>
			strip_bus_initialization(&mut target.values, *initialization_length),
	};

	target.confidence *= confidence;

	Ok(())
}
