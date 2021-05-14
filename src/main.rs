use std::{collections::HashMap};
use clap::{Arg, App};

pub mod hex;
pub mod common;
pub mod filters;

use hex::{read_hex_file};
use common::{DynResult, BusExtraction, extract_value};
use filters::{Filter, FILTER_SAMPLES, apply_filter};

pub type BusPackets = HashMap<u64, Vec<Vec<u8>>>;

fn find_value(packets: &BusPackets, start: usize, align: usize, size: usize, filters: Vec<Filter>) -> DynResult<()> {

	let mut finds = vec![];

	for (header, messages) in packets {
		let mut index = start;
		let msg_len = messages[0].len() * 8; // XXX might cause panics for varying sized packets

		while index + size <= msg_len {
			let raw_values: Vec<u64> = messages
				.iter()
				.map(|x| extract_value(x, index, index + size))
				.collect();
			let values = raw_values
				.iter()
				.map(|x| *x as f64)
				.collect();

			let mut target = BusExtraction {
				header: *header,
				index: index,
				size: size,
				raw_values: raw_values,
				values: values,
				confidence: 1f64,
				scale: 1f64,
				offset: 0f64,
			};

			for filter in &filters {
				apply_filter(&filter, &mut target)?;
			}

			if target.confidence > 0.9 {
				finds.push(target);
			}

			index += align;
		}

		println!("Applied {} filters on {} entries of packet {:03x} with size {}",
			filters.len(), messages.len(), header, size);
	}

	if finds.len() == 0 {
		println!("ERROR: Unable to find value :(");
		return Ok(()); // derp
		//return Err("Unable to find value :(");
	}

	finds.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
	for i in 0..20 {
		if i >= finds.len() {
			break
		}

		println!("FIND: confidence = {}, msg = {:03x}, index = {}, scale = {}, offset = {}",
			finds[i].confidence, finds[i].header, finds[i].index, finds[i].scale, finds[i].offset);
	}

	Ok(())
}

fn main() -> DynResult<()> {
	let matches = App::new("bus-learning")
		.version("0.0")
		.author("Jakob Löw <jakob@löw.com>")
		.about("Tries to learn bus protocols based on given ground truths")
		.arg(Arg::with_name("show-filter-samples")
			.long("show-filter-samples")
			.help("Outputs a JSON list with example filters")
			.takes_value(false)
			.required(false)
		)
		.arg(Arg::with_name("input")
			.short("i")
			.long("input")
			.value_name("FILE")
			.help("Sets the input bus dump file")
			.takes_value(true)
			.required(true)
		)
		.arg(Arg::with_name("header-length")
			.long("header-length")
			.value_name("LENGTH")
			.help("How many bits at the beginning of each message are the header")
			.takes_value(true)
			.required(true)
		)
		.arg(Arg::with_name("value-min")
			.long("value-min")
			.value_name("VALUE-MIN")
			.help("The maximum value of the entry we are searching for on the bus")
			.takes_value(true)
			.required(true)
		)
		.arg(Arg::with_name("value-max")
			.long("value-max")
			.value_name("VALUE-MAX")
			.help("The maximum value of the entry we are searching for on the bus")
			.takes_value(true)
			.required(true)
		)
		.arg(Arg::with_name("value-size")
			.long("value-size")
			.value_name("VALUE SIZE")
			.help("The size of the value in bits")
			.takes_value(true)
			.required(true)
		)
		.arg(Arg::with_name("value-align")
			.long("value-align")
			.value_name("VALUE ALIGNMENT")
			.help("The alignment of the value in bits (default: 8)")
			.takes_value(true)
		)
		.arg(Arg::with_name("is-oscillating")
			.long("is-oscillating")
			.value_name("IS OSCILLATING")
			.help("Wether to filter non-oscillating values")
			.takes_value(false)
		)
		.arg(Arg::with_name("is-linear")
			.long("is-linear")
			.value_name("IS LINEAR")
			.help("Wether to filter non-linear values")
			.takes_value(false)
		)
		.get_matches();

	if matches.is_present("show-filter-samples") {
		let samples = serde_json::to_string_pretty(FILTER_SAMPLES)?;
		println!("{}", samples);
		return Ok(());
	}

	let input = matches.value_of("input").unwrap();
	let target_min = matches.value_of("value-min").unwrap().parse::<f64>()?;
	let target_max = matches.value_of("value-max").unwrap().parse::<f64>()?;
	let value_size = matches.value_of("value-size").unwrap().parse::<usize>()?;
	let value_align = matches.value_of("value-align").unwrap_or("8").parse::<usize>()?;
	let header_len = matches.value_of("header-length").unwrap().parse::<usize>()?;
	let header_byte_len = ((header_len + 7) & !0x7) / 8;

	let mut filter = vec![
		Filter::CommonScale {
			min: target_min,
			max: target_max,
		},
	];
	if matches.is_present("is-linear") {
		filter.push(Filter::StrictLinear{
			is_falling: false,
		});
	}
	if matches.is_present("is-oscillating") {
		filter.push(Filter::OscillatingSensor{
			min_oscillating_ratio: 0.5,
		});
	}

	let lines = read_hex_file(input.to_string())?;
	let mut packets: BusPackets = HashMap::new();
	let mut varying = vec![];

	for msg in lines {
		if msg.len() < header_byte_len {
			continue;
		}

		let header = extract_value(&msg, 0, header_len);
		let others = packets.entry(header).or_insert(vec![]);
		if others.len() > 0 && others[0].len() != msg.len() && !varying.contains(&header) {
			println!("Message {:04x} has varying size!", header);
			varying.push(header);
		}

		others.push(msg);
	}

	find_value(&packets, header_len, value_align, value_size, filter)?;

	Ok(())
}
