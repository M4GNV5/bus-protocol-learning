pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

pub struct BusExtraction {
	// information on where/how to extract the value
	pub header: u64,
	pub index: usize,
	pub size: usize,

	// extracted/found values
	pub raw_values: Vec<u64>,
	pub values: Vec<f64>,
	pub confidence: f64,

	// extracted/found parameter
	pub scale: f64,
	pub offset: f64,
}

pub fn extract_bit(message: &Vec<u8>, index: usize) -> bool {
	let byte = index / 8;
	let mask = 1 << (7 - index % 8);
	message[byte] & mask != 0
}

pub fn extract_value(message: &Vec<u8>, start_bit: usize, end_bit: usize) -> u64 {
	let mut value = 0;
	for i in start_bit..end_bit {
		value <<= 1;
		if extract_bit(&message, i) {
			value |= 1;
		}
	}

	value
}
