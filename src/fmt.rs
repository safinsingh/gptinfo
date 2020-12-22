use std::cmp;

// Taken from https://github.com/banyan/rust-pretty-bytes/blob/master/src/converter.rs,
// but uses a delimeter of 1024 instead of 1000
pub(crate) fn convert(num: f64) -> String {
	let negative = if num.is_sign_positive() { "" } else { "-" };
	let num = num.abs();

	let units = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
	if num < 1_f64 {
		return format!("{}{} {}", negative, num, "B");
	}

	let delimiter = 1024_f64;
	let exponent = cmp::min(
		(num.ln() / delimiter.ln()).floor() as i32,
		(units.len() - 1) as i32,
	);

	let pretty_bytes = format!("{:.2}", num / delimiter.powi(exponent))
		.parse::<f64>()
		.unwrap()
		* 1_f64;
	let unit = units[exponent as usize];

	format!("{}{} {}", negative, pretty_bytes, unit)
}
