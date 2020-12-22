use crate::{
	constants, entry::Entry, guid::get_str_from_ty_uuid, header::Header,
	partition::Partition,
};
use anyhow::{Context as _, Result};
use std::{fs::File, os::unix::fs::FileExt, str};

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

pub(crate) struct Reader<'a> {
	location: &'a str,
	entries: Vec<Entry>,
}

impl<'a> Reader<'a> {
	pub(crate) fn new(location: &'a str) -> Reader<'a> {
		Self {
			location,
			entries: Vec::new(),
		}
	}

	pub(crate) fn analyze(mut self) -> Result<Vec<Entry>> {
		for lba in 1..=33 {
			self.analyze_lba(lba)?;
		}

		Ok(self.entries)
	}

	fn analyze_lba(&mut self, lba: u64) -> Result<()> {
		Ok(match lba {
			// The zero-th LBA is the protective MBR. Because we don't need to
			// look over this, we can skip over to the next LBA, which is the
			// Primary GPT Header
			1 => {
				// The Primary GPT Header is always 512 bytes
				let mut bytes = [0u8; 512];
				read_block_at_offset(
					self.location,
					&mut bytes,
					constants::MBR_OFFSET,
				)?;

				let header = Header::from(&bytes)?;
				let loc = self.location.clone();
				let name = format!("Block device ({})", loc);

				self.entries.push(Entry::new(
					name,
					header.first_usable_lba,
					header.last_usable_lba,
				));
			}
			2..=33 => {
				// Partition entries are always 128 bytes
				let mut bytes = [0u8; 128];
				read_block_at_offset(
					self.location,
					&mut bytes,
					// Protective MBR (512B) + GPT Header (512) + LBA relative
					// to GPT Header (current LBA - 2)
					constants::MBR_OFFSET + 512 + ((lba - 2) * 128) as u64,
				)?;

				let partition = Partition::from(&mut bytes)?;

				if let Some(ty) = get_str_from_ty_uuid(partition.ty_guid) {
					self.entries.push(Entry::new(
						ty.to_string(),
						partition.first_lba,
						partition.last_lba,
					));
				}
			}
			_ => unreachable!(),
		})
	}
}
