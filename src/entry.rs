#[derive(Debug)]
pub(crate) struct Entry {
	pub(crate) ty: String,
	pub(crate) start: u64,
	pub(crate) end: u64,
}

impl Entry {
	pub(crate) fn new(ty: String, start: u64, end: u64) -> Entry {
		Self { ty, start, end }
	}
}
