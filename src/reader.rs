use crate::{constants, guid::get_str_from_ty_uuid, partition::Partition};
use anyhow::{ensure, Context as _, Result};
use byteorder::{NativeEndian, ReadBytesExt};
use std::{fs::File, io::Cursor, os::unix::fs::FileExt, str};

fn read_block_at_offset<'a>(
	location: &'a str,
	slice: &'a mut [u8],
	offset: u64,
) -> Result<&'a [u8]> {
	File::open(location)
		.with_context(|| {
			format!("Failed to open block device at: {}", location)
		})?
		.read_at(slice, offset)
		.with_context(|| {
			format!("Failed to read block device: {}!", location)
		})?;

	Ok(slice)
}

#[derive(Debug)]
pub(crate) struct Entry {
	pub(crate) ty: String,
	pub(crate) start: u64,
	pub(crate) end: u64,
}

impl Entry {
	fn new(ty: String, start: u64, end: u64) -> Entry {
		Self { ty, start, end }
	}
}

#[derive(Debug)]
struct Header {
	first_usable_lba: u64,
	last_usable_lba: u64,
}

impl Header {
	fn from(slice: &[u8]) -> Result<Self> {
		let mut cursor = Cursor::new(slice);

		// GPT signature is in the first 8 bytes of the header
		let signature = cursor
			.read_u64::<NativeEndian>()
			.context("Failed to read GPT Header Signature from slice")?;

		ensure!(
			signature == constants::GPT_HEADER_SIG,
			"Failed to verify GPT Signature"
		);

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

pub(crate) fn analyze(location: &str) -> Result<Vec<Entry>> {
	let mut entries = Vec::new();
	for lba in 1..=33 {
		analyze_lba(location, lba, &mut entries)?;
	}

	Ok(entries)
}

fn analyze_lba(
	location: &str,
	lba: u64,
	entries: &mut Vec<Entry>,
) -> Result<()> {
	match lba {
		// The zero-th LBA is the protective MBR. Because we don't need to
		// look over this, we can skip over to the next LBA, which is the
		// Primary GPT Header
		1 => {
			// The Primary GPT Header is always 512 bytes
			let mut bytes = [0u8; 512];
			read_block_at_offset(location, &mut bytes, constants::MBR_OFFSET)?;

			let header = Header::from(&bytes)?;
			let name = format!("Block device ({})", location);

			entries.push(Entry::new(
				name,
				header.first_usable_lba,
				header.last_usable_lba,
			));
		}
		2..=33 => {
			// Partition entries are always 128 bytes
			let mut bytes = [0u8; 128];
			read_block_at_offset(
				location,
				&mut bytes,
				// Protective MBR (512B) + GPT Header (512) + LBA relative
				// to GPT Header (current LBA - 2)
				constants::MBR_OFFSET + 512 + ((lba - 2) * 128) as u64,
			)?;

			let partition = Partition::from(&bytes);

			if let Some(ty) =
				get_str_from_ty_uuid(&partition.ty_guid.as_bytes())
			{
				entries.push(Entry::new(
					ty.to_string(),
					partition.first_lba,
					partition.last_lba,
				));
			}
		}
		_ => unreachable!(),
	};
	Ok(())
}
