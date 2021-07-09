use serde::{Deserialize, Serialize};

use crate::common::{DynResult, BusExtraction};

use common_scale::get_best_common_scale;
use require_spread::filter_by_required_spread;
use require_in_range::filter_not_in_range;
use linear_growth::filter_non_linear_growth;
use local_min_max::require_local_min_max;
use monotonic_change::filter_non_monotonic_values;
use oscillating_sensor::filter_non_oscillating_sensors;
use constants::filter_constant_values;
use strip_bus_initialization::strip_bus_initialization;

mod common_scale;
mod require_spread;
mod require_in_range;
mod linear_growth;
mod local_min_max;
mod monotonic_change;
mod oscillating_sensor;
mod constants;
mod strip_bus_initialization;

#[derive(Serialize, Deserialize, Debug)]
pub enum Filter {
	CommonScale {
		min: f64,
		max: f64,
	},
	RequireSpread {
		min: f64,
		max: f64,
		ratio: f64,
	},
	RequireInRange {
		min: f64,
		max: f64,
		ratio: f64,
	},
	OscillatingSensor {
		min_oscillating_ratio: f64,
	},
	RequireLocalMinMax {
		window_size: usize,
		required_count: usize,
	},
	MonotonicChange {
		max_change: f64,
		required_monotonic_ratio: f64,
	},
	StrictRising,
	StrictFalling,
	RequireNonConstant,
	RequireConstant,
	StripInitialization,
}

pub const FILTER_SAMPLES: &'static [Filter] = &[
	Filter::CommonScale {
		min: 42.0,
		max: 1337.0,
	},
	Filter::OscillatingSensor {
		min_oscillating_ratio: 0.5,
	},
	Filter::StrictRising,
	Filter::StrictFalling,
	Filter::RequireNonConstant,
	Filter::RequireConstant,
	Filter::StripInitialization,
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
		Filter::RequireSpread {min, max, ratio} => {
			filter_by_required_spread(&target.values, *min, *max, *ratio)
		},
		Filter::RequireInRange {min, max, ratio} => {
			filter_not_in_range(&target.values, *min, *max, *ratio)
		}
		Filter::OscillatingSensor {min_oscillating_ratio} => {
			filter_non_oscillating_sensors(&target.values, *min_oscillating_ratio)
		},
		Filter::RequireLocalMinMax {window_size, required_count} => {
			require_local_min_max(&target.values, *window_size, *required_count)
		},
		Filter::MonotonicChange {max_change, required_monotonic_ratio} => {
			filter_non_monotonic_values(&target.values, *max_change, *required_monotonic_ratio)
		},
		Filter::StrictRising => filter_non_linear_growth(&target.values, false),
		Filter::StrictFalling => filter_non_linear_growth(&target.values, true),
		Filter::RequireNonConstant => filter_constant_values(&target.values, false),
		Filter::RequireConstant => filter_constant_values(&target.values, true),
		Filter::StripInitialization => strip_bus_initialization(target)?,
	};

	target.confidence *= confidence;

	Ok(())
}
