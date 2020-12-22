use crate::constants;
use anyhow::{ensure, Result};
use byteorder::{ByteOrder, LittleEndian, NativeEndian};
use std::{convert::TryInto, u32};

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

#[derive(Debug)]
pub(crate) struct Header {
	pub(crate) first_usable_lba: u64,
	pub(crate) last_usable_lba: u64,
}

impl Header {
	pub(crate) fn from(slice: [u8; 512]) -> Result<Self> {
		// GPT signature is in the first 8 bytes of the header
		let signature = NativeEndian::read_u64(&slice[0..8]);

		ensure!(
			signature == constants::GPT_HEADER_SIG,
			"Failed to verify GPT Signature"
		);

		// First usable LBA is at a 40-byte offset and is 8 bytes in length
		let first_usable_lba = NativeEndian::read_u64(&slice[40..48]);

		// Last usable LBA is at a 48-byte offset and is 8 bytes in length
		let last_usable_lba = NativeEndian::read_u64(&slice[48..56]);

		Ok(Self {
			first_usable_lba,
			last_usable_lba,
		})
	}
}

#[derive(Debug)]
pub(crate) struct Guid {
	data_1: u32,
	data_2: u16,
	data_3: u16,
	data_4: [u8; 8],
}

impl Guid {
	fn from_bytes(buffer: [u8; 16]) -> Self {
		let data_1 = NativeEndian::read_u32(&buffer[..4]);
		let data_2 = NativeEndian::read_u16(&buffer[4..6]);
		let data_3 = NativeEndian::read_u16(&buffer[6..8]);

		let mut data_4 = [0; 8];
		data_4.copy_from_slice(&buffer[8..]);

		Self {
			data_1,
			data_2,
			data_3,
			data_4,
		}
	}

	pub(crate) fn as_bytes(&self) -> [u8; 16] {
		let data_1 = u32::to_be_bytes(self.data_1);
		let data_2 = u16::to_be_bytes(self.data_2);
		let data_3 = u16::to_be_bytes(self.data_3);

		let mut bytes = [0u8; 16];
		bytes[0..4].copy_from_slice(&data_1);
		bytes[4..6].copy_from_slice(&data_2);
		bytes[6..8].copy_from_slice(&data_3);
		bytes[8..16].copy_from_slice(&self.data_4);

		bytes
	}
}

#[derive(Debug)]
pub(crate) struct Partition {
	// 16-byte GUID
	pub(crate) ty_guid: Guid,
	pub(crate) first_lba: u64,
	pub(crate) last_lba: u64,
}

impl From<&[u8; 128]> for Partition {
	fn from(slice: &[u8; 128]) -> Self {
		// Partition Type GUIDs are in Mixed Endian, and are 16 bytes in
		// length. According to Microsoft
		// (https://docs.microsoft.com/en-us/windows/win32/api/guiddef/ns-guiddef-guid),
		// there are 4 separate parts to this GUID. The 4th "part" represents
		// the last 12 hexadecimal digits, which, when printed, are typically
		// specified as two "parts", one 4-digit (2-byte) section and one
		// 8-digit (4-byte) section.
		let guid = Guid::from_bytes(slice[0..16].try_into().unwrap());

		// Skip first 32 bytes of slice. First 16 bytes correspond to the type
		// GUID and the next 16 correspond to the unqiue GUID (PARTUUID)
		// First LBA is ALWAYS in Little Endian
		let first_lba = LittleEndian::read_u64(&slice[32..40]);

		// Set offset to 32 + 8 bytes for first-lba offset
		let last_lba = NativeEndian::read_u64(&slice[40..48]);

		Self {
			ty_guid: guid,
			first_lba,
			last_lba,
		}
	}
}
