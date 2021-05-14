use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

use crate::common::DynResult;

pub fn parse_hex(str: String) -> Vec<u8> {
	str
		.to_uppercase()
		.chars()
		.filter(|x| x.is_numeric() || (*x >= 'A' && *x <= 'F'))
		.map(|x| if x.is_numeric() {
			x as u8 - '0' as u8
		} else {
			x as u8 - 'A' as u8 + 10
		})
		.chunks(2)
		.into_iter()
		.map(|mut chunk| {
			match chunk.next_tuple() {
				Some((x, y)) => x << 4 | y,
				None => chunk.next().unwrap_or(0),
			}
		})
		.collect()
}

pub fn read_hex_file(filename: String) -> DynResult<Vec<Vec<u8>>> {
	let file = File::open(filename)?;
	let reader = BufReader::new(file);

	let lines = reader
		.lines()
		.enumerate()
		.map(|(_, liner)| match liner {
			Ok(line) => parse_hex(line),
			_ => vec![],
		})
		.collect();

	Ok(lines)
}
