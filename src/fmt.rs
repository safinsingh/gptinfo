pub(crate) fn format_bytes(end: u64, start: u64) -> String {
	let raw = (end - start + 1) * 512;
	match raw {
		// 1024 bytes in a MB
		1..=1023 => {
			format!("{}B", raw)
		}
		// 1048576 bytes in a KB
		1024..=1048575 => {
			format!("{}K", raw / 1024)
		}
		// 1073741824 bytes in a GB
		1048576..=1073741823 => {
			format!("{}M", raw / 1024 / 1024)
		}
		// 1099511627776 bytes in a TB
		1073741824..=1099511627775 => {
			format!("{}G", raw / 1024 / 1024 / 1024)
		}
		// 1125899906842624 bytes in a PB
		1099511627776..=1125899906842624 => {
			format!("{}T", raw / 1024 / 1024 / 1024 / 1024)
		}
		_ => panic!("Partition size exceeds 1023T and cannot be displayed!"),
	}
}
