use anyhow::Result;
use byteorder::{LittleEndian, NativeEndian, ReadBytesExt};
use std::io::Cursor;

#[derive(Debug)]
pub(crate) struct Partition<'a> {
	// 16-byte GUID
	pub(crate) ty_guid: &'a [u8],
	pub(crate) first_lba: u64,
	pub(crate) last_lba: u64,
}

impl<'a> Partition<'a> {
	pub(crate) fn from(slice: &'a mut [u8]) -> Result<Self> {
		// Partition Type GUIDs are in Mixed Endian, and are 16 bytes in
		// length. According to Microsoft
		// (https://docs.microsoft.com/en-us/windows/win32/api/guiddef/ns-guiddef-guid),
		// there are 4 separate parts to this GUID. The 4th "part" represents
		// the last 12 hexadecimal digits, which, when printed, are typically
		// specified as two "parts", one 4-digit (2-byte) section and one
		// 8-digit (4-byte) section.
		slice[0..4].reverse();
		slice[4..6].reverse();
		slice[6..8].reverse();

		let mut cursor = Cursor::new(slice);

		// Skip first 32 bytes of slice. First 16 bytes correspond to the type
		// GUID and the next 16 correspond to the unqiue GUID (PARTUUID)
		cursor.set_position(32);

		// First LBA is ALWAYS in Little Endian
		let first_lba = cursor.read_u64::<LittleEndian>()?;

		// Set offset to 32 + 8 bytes for first-lba offset
		cursor.set_position(40);
		let last_lba = cursor.read_u64::<NativeEndian>()?;

		Ok(Self {
			ty_guid: &cursor.into_inner()[0..16],
			first_lba,
			last_lba,
		})
	}
}
