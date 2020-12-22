use crate::constants;
use anyhow::{anyhow, Context as _, Result};
use byteorder::{NativeEndian, ReadBytesExt};
use std::io::Cursor;

#[derive(Debug)]
pub(crate) struct Header {
	pub(crate) first_usable_lba: u64,
	pub(crate) last_usable_lba: u64,
}

impl Header {
	pub(crate) fn from(slice: &[u8]) -> Result<Self> {
		let mut cursor = Cursor::new(slice);

		// GPT signature is in the first 8 bytes of the header
		let signature = cursor
			.read_u64::<NativeEndian>()
			.context("Failed to read GPT Header Signature from slice")?;

		if signature != constants::GPT_HEADER_SIG {
			return Err(anyhow!("Failed to verify GPT Signature"));
		}

		// First usable LBA is at a 40-byte offset and is 8 bytes in length
		cursor.set_position(40);
		let first_usable_lba = cursor
			.read_u64::<NativeEndian>()
			.context("Failed to read first usable LBA from slice")?;

		// Last usable LBA is at a 48-byte offset and is 8 bytes in length
		cursor.set_position(48);
		let last_usable_lba = cursor
			.read_u64::<NativeEndian>()
			.context("Failed to read first usable LBA from slice")?;

		Ok(Self {
			first_usable_lba,
			last_usable_lba,
		})
	}
}
