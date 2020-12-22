#[allow(unreachable_patterns)]

// Get partition type given 16-byte GUID
pub(crate) fn get_str_from_ty_uuid(uuid: &[u8]) -> Option<&'static str> {
	match uuid {
		&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] => {
			None
		}
		&[0x02, 0x4D, 0xEE, 0x41, 0x33, 0xE7, 0x11, 0xD3, 0x9D, 0x69, 0x00, 0x08, 0xC7, 0x81, 0xF3, 0x9F] => {
			Some("MBR partition scheme")
		}
		&[0xC1, 0x2A, 0x73, 0x28, 0xF8, 0x1F, 0x11, 0xD2, 0xBA, 0x4B, 0x00, 0xA0, 0xC9, 0x3E, 0xC9, 0x3B] => {
			Some("EFI System partition")
		}
		&[0x21, 0x68, 0x61, 0x48, 0x64, 0x49, 0x6E, 0x6F, 0x74, 0x4E, 0x65, 0x65, 0x64, 0x45, 0x46, 0x49] => {
			Some("BIOS boot partition")
		}
		&[0xD3, 0xBF, 0xE2, 0xDE, 0x3D, 0xAF, 0x11, 0xDF, 0xBA, 0x40, 0xE3, 0xA5, 0x56, 0xD8, 0x95, 0x93] => {
			Some(
				"Intel Fast Flash (iFFS) partition (for Intel Rapid Start \
				 technology)",
			)
		}
		&[0xF4, 0x01, 0x97, 0x32, 0x06, 0x6E, 0x4E, 0x12, 0x82, 0x73, 0x34, 0x6C, 0x56, 0x41, 0x49, 0x4F] => {
			Some("Sony boot partition")
		}
		&[0xBF, 0xBF, 0xAF, 0xE7, 0xA3, 0x4F, 0x44, 0x8A, 0x9A, 0x5B, 0x62, 0x13, 0xEB, 0x73, 0x6C, 0x22] => {
			Some("Lenovo boot partition")
		}
		&[0xE3, 0xC9, 0xE3, 0x16, 0x0B, 0x5C, 0x4D, 0xB8, 0x81, 0x7D, 0xF9, 0x2D, 0xF0, 0x02, 0x15, 0xAE] => {
			Some("Windows Microsoft Reserved Partition ")
		}
		&[0xEB, 0xD0, 0xA0, 0xA2, 0xB9, 0xE5, 0x44, 0x33, 0x87, 0xC0, 0x68, 0xB6, 0xB7, 0x26, 0x99, 0xC7] => {
			Some("Windows Basic data partition")
		}
		&[0x58, 0x08, 0xC8, 0xAA, 0x7E, 0x8F, 0x42, 0xE0, 0x85, 0xD2, 0xE1, 0xE9, 0x04, 0x34, 0xCF, 0xB3] => {
			Some("Windows Logical Disk Manager (LDM) metadata partition")
		}
		&[0xAF, 0x9B, 0x60, 0xA0, 0x14, 0x31, 0x4F, 0x62, 0xBC, 0x68, 0x33, 0x11, 0x71, 0x4A, 0x69, 0xAD] => {
			Some("Windows Logical Disk Manager data partition")
		}
		&[0xDE, 0x94, 0xBB, 0xA4, 0x06, 0xD1, 0x4D, 0x40, 0xA1, 0x6A, 0xBF, 0xD5, 0x01, 0x79, 0xD6, 0xAC] => {
			Some("Windows Recovery Environment")
		}
		&[0x37, 0xAF, 0xFC, 0x90, 0xEF, 0x7D, 0x4E, 0x96, 0x91, 0xC3, 0x2D, 0x7A, 0xE0, 0x55, 0xB1, 0x74] => {
			Some("Windows IBM General Parallel File System (GPFS) partition")
		}
		&[0xE7, 0x5C, 0xAF, 0x8F, 0xF6, 0x80, 0x4C, 0xEE, 0xAF, 0xA3, 0xB0, 0x01, 0xE5, 0x6E, 0xFC, 0x2D] => {
			Some("Windows Storage Spaces partition")
		}
		&[0x55, 0x8D, 0x43, 0xC5, 0xA1, 0xAC, 0x43, 0xC0, 0xAA, 0xC8, 0xD1, 0x47, 0x2B, 0x29, 0x23, 0xD1] => {
			Some("Windows Storage Replica partition")
		}
		&[0x75, 0x89, 0x4C, 0x1E, 0x3A, 0xEB, 0x11, 0xD3, 0xB7, 0xC1, 0x7B, 0x03, 0xA0, 0x00, 0x00, 0x00] => {
			Some("HP-UX Data partition")
		}
		&[0xE2, 0xA1, 0xE7, 0x28, 0x32, 0xE3, 0x11, 0xD6, 0xA6, 0x82, 0x7B, 0x03, 0xA0, 0x00, 0x00, 0x00] => {
			Some("HP-UX Service partition")
		}
		&[0x0F, 0xC6, 0x3D, 0xAF, 0x84, 0x83, 0x47, 0x72, 0x8E, 0x79, 0x3D, 0x69, 0xD8, 0x47, 0x7D, 0xE4] => {
			Some("Linux filesystem data")
		}
		&[0xA1, 0x9D, 0x88, 0x0F, 0x05, 0xFC, 0x4D, 0x3B, 0xA0, 0x06, 0x74, 0x3F, 0x0F, 0x84, 0x91, 0x1E] => {
			Some("Linux RAID partition")
		}
		&[0x44, 0x47, 0x95, 0x40, 0xF2, 0x97, 0x41, 0xB2, 0x9A, 0xF7, 0xD1, 0x31, 0xD5, 0xF0, 0x45, 0x8A] => {
			Some("Linux Root partition (x86)")
		}
		&[0x4F, 0x68, 0xBC, 0xE3, 0xE8, 0xCD, 0x4D, 0xB1, 0x96, 0xE7, 0xFB, 0xCA, 0xF9, 0x84, 0xB7, 0x09] => {
			Some("Linux Root partition (x86-64)")
		}
		&[0x69, 0xDA, 0xD7, 0x10, 0x2C, 0xE4, 0x4E, 0x3C, 0xB1, 0x6C, 0x21, 0xA1, 0xD4, 0x9A, 0xBE, 0xD3] => {
			Some("Linux Root partition (32-bit ARM)")
		}
		&[0xB9, 0x21, 0xB0, 0x45, 0x1D, 0xF0, 0x41, 0xC3, 0xAF, 0x44, 0x4C, 0x6F, 0x28, 0x0D, 0x3F, 0xAE] => {
			Some("Linux Root partition (64-bit ARM/AArch64)")
		}
		&[0xBC, 0x13, 0xC2, 0xFF, 0x59, 0xE6, 0x42, 0x62, 0xA3, 0x52, 0xB2, 0x75, 0xFD, 0x6F, 0x71, 0x72] => {
			Some("Linux /boot partition")
		}
		&[0x06, 0x57, 0xFD, 0x6D, 0xA4, 0xAB, 0x43, 0xC4, 0x84, 0xE5, 0x09, 0x33, 0xC8, 0x4B, 0x4F, 0x4F] => {
			Some("Linux Swap partition")
		}
		&[0xE6, 0xD6, 0xD3, 0x79, 0xF5, 0x07, 0x44, 0xC2, 0xA2, 0x3C, 0x23, 0x8F, 0x2A, 0x3D, 0xF9, 0x28] => {
			Some("Linux Logical Volume Manager (LVM) partition")
		}
		&[0x93, 0x3A, 0xC7, 0xE1, 0x2E, 0xB4, 0x4F, 0x13, 0xB8, 0x44, 0x0E, 0x14, 0xE2, 0xAE, 0xF9, 0x15] => {
			Some("Linux /home partition")
		}
		&[0x3B, 0x8F, 0x84, 0x25, 0x20, 0xE0, 0x4F, 0x3B, 0x90, 0x7F, 0x1A, 0x25, 0xA7, 0x6F, 0x98, 0xE8] => {
			Some("Linux /srv (server data) partition")
		}
		&[0x7F, 0xFE, 0xC5, 0xC9, 0x2D, 0x00, 0x49, 0xB7, 0x89, 0x41, 0x3E, 0xA1, 0x0A, 0x55, 0x86, 0xB7] => {
			Some("Linux Plain dm-crypt partition")
		}
		&[0xCA, 0x7D, 0x7C, 0xCB, 0x63, 0xED, 0x4C, 0x53, 0x86, 0x1C, 0x17, 0x42, 0x53, 0x60, 0x59, 0xCC] => {
			Some("Linux LUKS partition")
		}
		&[0x8D, 0xA6, 0x33, 0x39, 0x00, 0x07, 0x60, 0xC0, 0xC4, 0x36, 0x08, 0x3A, 0xC8, 0x23, 0x09, 0x08] => {
			Some("Linux Reserved")
		}
		&[0x83, 0xBD, 0x6B, 0x9D, 0x7F, 0x41, 0x11, 0xDC, 0xBE, 0x0B, 0x00, 0x15, 0x60, 0xB8, 0x4F, 0x0F] => {
			Some("FreeBSD Boot partition")
		}
		&[0x51, 0x6E, 0x7C, 0xB4, 0x6E, 0xCF, 0x11, 0xD6, 0x8F, 0xF8, 0x00, 0x02, 0x2D, 0x09, 0x71, 0x2B] => {
			Some("FreeBSD Data partition")
		}
		&[0x51, 0x6E, 0x7C, 0xB5, 0x6E, 0xCF, 0x11, 0xD6, 0x8F, 0xF8, 0x00, 0x02, 0x2D, 0x09, 0x71, 0x2B] => {
			Some("FreeBSD Swap partition")
		}
		&[0x51, 0x6E, 0x7C, 0xB6, 0x6E, 0xCF, 0x11, 0xD6, 0x8F, 0xF8, 0x00, 0x02, 0x2D, 0x09, 0x71, 0x2B] => {
			Some("FreeBSD Unix File System (UFS) partition")
		}
		&[0x51, 0x6E, 0x7C, 0xB8, 0x6E, 0xCF, 0x11, 0xD6, 0x8F, 0xF8, 0x00, 0x02, 0x2D, 0x09, 0x71, 0x2B] => {
			Some("FreeBSD Vinum volume manager partition")
		}
		&[0x51, 0x6E, 0x7C, 0xBA, 0x6E, 0xCF, 0x11, 0xD6, 0x8F, 0xF8, 0x00, 0x02, 0x2D, 0x09, 0x71, 0x2B] => {
			Some("FreeBSD ZFS partition")
		}
		&[0x48, 0x46, 0x53, 0x00, 0x00, 0x00, 0x11, 0xAA, 0xAA, 0x11, 0x00, 0x30, 0x65, 0x43, 0xEC, 0xAC] => {
			Some("Darwin Hierarchical File System Plus (HFS+) partition")
		}
		&[0x7C, 0x34, 0x57, 0xEF, 0x00, 0x00, 0x11, 0xAA, 0xAA, 0x11, 0x00, 0x30, 0x65, 0x43, 0xEC, 0xAC] => {
			Some("Darwin Apple APFS container")
		}
		&[0x7C, 0x34, 0x57, 0xEF, 0x00, 0x00, 0x11, 0xAA, 0xAA, 0x11, 0x00, 0x30, 0x65, 0x43, 0xEC, 0xAC] => {
			Some("Darwin APFS FileVault volume container")
		}
		&[0x55, 0x46, 0x53, 0x00, 0x00, 0x00, 0x11, 0xAA, 0xAA, 0x11, 0x00, 0x30, 0x65, 0x43, 0xEC, 0xAC] => {
			Some("Darwin Apple UFS container")
		}
		&[0x6A, 0x89, 0x8C, 0xC3, 0x1D, 0xD2, 0x11, 0xB2, 0x99, 0xA6, 0x08, 0x00, 0x20, 0x73, 0x66, 0x31] => {
			Some("Darwin ZFS")
		}
		&[0x52, 0x41, 0x49, 0x44, 0x00, 0x00, 0x11, 0xAA, 0xAA, 0x11, 0x00, 0x30, 0x65, 0x43, 0xEC, 0xAC] => {
			Some("Darwin Apple RAID partition")
		}
		&[0x52, 0x41, 0x49, 0x44, 0x5F, 0x4F, 0x11, 0xAA, 0xAA, 0x11, 0x00, 0x30, 0x65, 0x43, 0xEC, 0xAC] => {
			Some("Darwin Apple RAID partition, offline")
		}
		&[0x42, 0x6F, 0x6F, 0x74, 0x00, 0x00, 0x11, 0xAA, 0xAA, 0x11, 0x00, 0x30, 0x65, 0x43, 0xEC, 0xAC] => {
			Some("Darwin Apple Boot partition (Recovery HD)")
		}
		&[0x4C, 0x61, 0x62, 0x65, 0x6C, 0x00, 0x11, 0xAA, 0xAA, 0x11, 0x00, 0x30, 0x65, 0x43, 0xEC, 0xAC] => {
			Some("Darwin Apple Label")
		}
		&[0x52, 0x65, 0x63, 0x6F, 0x76, 0x65, 0x11, 0xAA, 0xAA, 0x11, 0x00, 0x30, 0x65, 0x43, 0xEC, 0xAC] => {
			Some("Darwin Apple TV Recovery partition")
		}
		&[0x53, 0x74, 0x6F, 0x72, 0x61, 0x67, 0x11, 0xAA, 0xAA, 0x11, 0x00, 0x30, 0x65, 0x43, 0xEC, 0xAC] => {
			Some("Darwin Apple Core Storage Container")
		}
		&[0x53, 0x74, 0x6F, 0x72, 0x61, 0x67, 0x11, 0xAA, 0xAA, 0x11, 0x00, 0x30, 0x65, 0x43, 0xEC, 0xAC] => {
			Some("Darwin HFS+ FileVault volume container")
		}
		&[0xB6, 0xFA, 0x30, 0xDA, 0x92, 0xD2, 0x4A, 0x9A, 0x96, 0xF1, 0x87, 0x1E, 0xC6, 0x48, 0x62, 0x00] => {
			Some("Darwin SoftRAID_Status")
		}
		&[0x2E, 0x31, 0x34, 0x65, 0x19, 0xB9, 0x46, 0x3F, 0x81, 0x26, 0x8A, 0x79, 0x93, 0x77, 0x38, 0x01] => {
			Some("Darwin SoftRAID_Scratch")
		}
		&[0xFA, 0x70, 0x9C, 0x7E, 0x65, 0xB1, 0x45, 0x93, 0xBF, 0xD5, 0xE7, 0x1D, 0x61, 0xDE, 0x9B, 0x02] => {
			Some("Darwin SoftRAID_Volume")
		}
		&[0xBB, 0xBA, 0x6D, 0xF5, 0xF4, 0x6F, 0x4A, 0x89, 0x8F, 0x59, 0x87, 0x65, 0xB2, 0x72, 0x75, 0x03] => {
			Some("Darwin SoftRAID_Cache")
		}
		&[0x6A, 0x82, 0xCB, 0x45, 0x1D, 0xD2, 0x11, 0xB2, 0x99, 0xA6, 0x08, 0x00, 0x20, 0x73, 0x66, 0x31] => {
			Some("Solaris illumos Boot partition")
		}
		&[0x6A, 0x85, 0xCF, 0x4D, 0x1D, 0xD2, 0x11, 0xB2, 0x99, 0xA6, 0x08, 0x00, 0x20, 0x73, 0x66, 0x31] => {
			Some("Solaris Root partition")
		}
		&[0x6A, 0x87, 0xC4, 0x6F, 0x1D, 0xD2, 0x11, 0xB2, 0x99, 0xA6, 0x08, 0x00, 0x20, 0x73, 0x66, 0x31] => {
			Some("Solaris Swap partition")
		}
		&[0x6A, 0x8B, 0x64, 0x2B, 0x1D, 0xD2, 0x11, 0xB2, 0x99, 0xA6, 0x08, 0x00, 0x20, 0x73, 0x66, 0x31] => {
			Some("Solaris Backup partition")
		}
		&[0x6A, 0x89, 0x8C, 0xC3, 0x1D, 0xD2, 0x11, 0xB2, 0x99, 0xA6, 0x08, 0x00, 0x20, 0x73, 0x66, 0x31] => {
			Some("Solaris /usr partition")
		}
		&[0x6A, 0x8E, 0xF2, 0xE9, 0x1D, 0xD2, 0x11, 0xB2, 0x99, 0xA6, 0x08, 0x00, 0x20, 0x73, 0x66, 0x31] => {
			Some("Solaris /var partition")
		}
		&[0x6A, 0x90, 0xBA, 0x39, 0x1D, 0xD2, 0x11, 0xB2, 0x99, 0xA6, 0x08, 0x00, 0x20, 0x73, 0x66, 0x31] => {
			Some("Solaris /home partition")
		}
		&[0x6A, 0x92, 0x83, 0xA5, 0x1D, 0xD2, 0x11, 0xB2, 0x99, 0xA6, 0x08, 0x00, 0x20, 0x73, 0x66, 0x31] => {
			Some("Solaris Alternate sector")
		}
		&[0x6A, 0x94, 0x5A, 0x3B, 0x1D, 0xD2, 0x11, 0xB2, 0x99, 0xA6, 0x08, 0x00, 0x20, 0x73, 0x66, 0x31] => {
			Some("Solaris Reserved partition")
		}
		&[0x6A, 0x96, 0x30, 0xD1, 0x1D, 0xD2, 0x11, 0xB2, 0x99, 0xA6, 0x08, 0x00, 0x20, 0x73, 0x66, 0x31] => {
			Some("Solaris Reserved partition")
		}
		&[0x6A, 0x98, 0x07, 0x67, 0x1D, 0xD2, 0x11, 0xB2, 0x99, 0xA6, 0x08, 0x00, 0x20, 0x73, 0x66, 0x31] => {
			Some("Solaris Reserved partition")
		}
		&[0x6A, 0x96, 0x23, 0x7F, 0x1D, 0xD2, 0x11, 0xB2, 0x99, 0xA6, 0x08, 0x00, 0x20, 0x73, 0x66, 0x31] => {
			Some("Solaris Reserved partition")
		}
		&[0x6A, 0x8D, 0x2A, 0xC7, 0x1D, 0xD2, 0x11, 0xB2, 0x99, 0xA6, 0x08, 0x00, 0x20, 0x73, 0x66, 0x31] => {
			Some("Solaris Reserved partition")
		}
		&[0x49, 0xF4, 0x8D, 0x32, 0xB1, 0x0E, 0x11, 0xDC, 0xB9, 0x9B, 0x00, 0x19, 0xD1, 0x87, 0x96, 0x48] => {
			Some("NetBSD Swap partition")
		}
		&[0x49, 0xF4, 0x8D, 0x5A, 0xB1, 0x0E, 0x11, 0xDC, 0xB9, 0x9B, 0x00, 0x19, 0xD1, 0x87, 0x96, 0x48] => {
			Some("NetBSD FFS partition")
		}
		&[0x49, 0xF4, 0x8D, 0x82, 0xB1, 0x0E, 0x11, 0xDC, 0xB9, 0x9B, 0x00, 0x19, 0xD1, 0x87, 0x96, 0x48] => {
			Some("NetBSD LFS partition")
		}
		&[0x49, 0xF4, 0x8D, 0xAA, 0xB1, 0x0E, 0x11, 0xDC, 0xB9, 0x9B, 0x00, 0x19, 0xD1, 0x87, 0x96, 0x48] => {
			Some("NetBSD RAID partition")
		}
		&[0x2D, 0xB5, 0x19, 0xC4, 0xB1, 0x0F, 0x11, 0xDC, 0xB9, 0x9B, 0x00, 0x19, 0xD1, 0x87, 0x96, 0x48] => {
			Some("NetBSD Concatenated partition")
		}
		&[0x2D, 0xB5, 0x19, 0xEC, 0xB1, 0x0F, 0x11, 0xDC, 0xB9, 0x9B, 0x00, 0x19, 0xD1, 0x87, 0x96, 0x48] => {
			Some("NetBSD Encrypted partition")
		}
		&[0xFE, 0x3A, 0x2A, 0x5D, 0x4F, 0x32, 0x41, 0xA7, 0xB7, 0x25, 0xAC, 0xCC, 0x32, 0x85, 0xA3, 0x09] => {
			Some("Chrome OS kernel")
		}
		&[0x3C, 0xB8, 0xE2, 0x02, 0x3B, 0x7E, 0x47, 0xDD, 0x8A, 0x3C, 0x7F, 0xF2, 0xA1, 0x3C, 0xFC, 0xEC] => {
			Some("Chrome OS rootfs")
		}
		&[0x2E, 0x0A, 0x75, 0x3D, 0x9E, 0x48, 0x43, 0xB0, 0x83, 0x37, 0xB1, 0x51, 0x92, 0xCB, 0x1B, 0x5E] => {
			Some("Chrome OS future use")
		}
		&[0x5D, 0xFB, 0xF5, 0xF4, 0x28, 0x48, 0x4B, 0xAC, 0xAA, 0x5E, 0x0D, 0x9A, 0x20, 0xB7, 0x45, 0xA6] => {
			Some("Container Linux by CoreOS /usr partition (coreos-usr)")
		}
		&[0x38, 0x84, 0xDD, 0x41, 0x85, 0x82, 0x44, 0x04, 0xB9, 0xA8, 0xE9, 0xB8, 0x4F, 0x2D, 0xF5, 0x0E] => {
			Some("Container Linux by CoreOS Resizable rootfs (coreos-resize)")
		}
		&[0xC9, 0x5D, 0xC2, 0x1A, 0xDF, 0x0E, 0x43, 0x40, 0x8D, 0x7B, 0x26, 0xCB, 0xFA, 0x9A, 0x03, 0xE0] => {
			Some(
				"Container Linux by CoreOS OEM customizations \
				 (coreos-reserved)",
			)
		}
		&[0xBE, 0x90, 0x67, 0xB9, 0xEA, 0x49, 0x4F, 0x15, 0xB4, 0xF6, 0xF3, 0x6F, 0x8C, 0x9E, 0x18, 0x18] => {
			Some(
				"Container Linux by CoreOS Root filesystem on RAID \
				 (coreos-root-raid)",
			)
		}
		&[0x42, 0x46, 0x53, 0x31, 0x3B, 0xA3, 0x10, 0xF1, 0x80, 0x2A, 0x48, 0x61, 0x69, 0x6B, 0x75, 0x21] => {
			Some("Haiku BFS")
		}
		&[0x85, 0xD5, 0xE4, 0x5E, 0x23, 0x7C, 0x11, 0xE1, 0xB4, 0xB3, 0xE8, 0x9A, 0x8F, 0x7F, 0xC3, 0xA7] => {
			Some("MidnightBSD Boot partition")
		}
		&[0x85, 0xD5, 0xE4, 0x5A, 0x23, 0x7C, 0x11, 0xE1, 0xB4, 0xB3, 0xE8, 0x9A, 0x8F, 0x7F, 0xC3, 0xA7] => {
			Some("MidnightBSD Data partition")
		}
		&[0x85, 0xD5, 0xE4, 0x5B, 0x23, 0x7C, 0x11, 0xE1, 0xB4, 0xB3, 0xE8, 0x9A, 0x8F, 0x7F, 0xC3, 0xA7] => {
			Some("MidnightBSD Swap partition")
		}
		&[0x03, 0x94, 0xEF, 0x8B, 0x23, 0x7E, 0x11, 0xE1, 0xB4, 0xB3, 0xE8, 0x9A, 0x8F, 0x7F, 0xC3, 0xA7] => {
			Some("MidnightBSD Unix File System (UFS) partition")
		}
		&[0x85, 0xD5, 0xE4, 0x5C, 0x23, 0x7C, 0x11, 0xE1, 0xB4, 0xB3, 0xE8, 0x9A, 0x8F, 0x7F, 0xC3, 0xA7] => {
			Some("MidnightBSD Vinum volume manager partition")
		}
		&[0x85, 0xD5, 0xE4, 0x5D, 0x23, 0x7C, 0x11, 0xE1, 0xB4, 0xB3, 0xE8, 0x9A, 0x8F, 0x7F, 0xC3, 0xA7] => {
			Some("MidnightBSD ZFS partition")
		}
		&[0x45, 0xB0, 0x96, 0x9E, 0x9B, 0x03, 0x4F, 0x30, 0xB4, 0xC6, 0xB4, 0xB8, 0x0C, 0xEF, 0xF1, 0x06] => {
			Some("Ceph Journal")
		}
		&[0x45, 0xB0, 0x96, 0x9E, 0x9B, 0x03, 0x4F, 0x30, 0xB4, 0xC6, 0x5E, 0xC0, 0x0C, 0xEF, 0xF1, 0x06] => {
			Some("Ceph dm-crypt journal")
		}
		&[0x4F, 0xBD, 0x7E, 0x29, 0x9D, 0x25, 0x41, 0xB8, 0xAF, 0xD0, 0x06, 0x2C, 0x0C, 0xEF, 0xF0, 0x5D] => {
			Some("Ceph OSD")
		}
		&[0x4F, 0xBD, 0x7E, 0x29, 0x9D, 0x25, 0x41, 0xB8, 0xAF, 0xD0, 0x5E, 0xC0, 0x0C, 0xEF, 0xF0, 0x5D] => {
			Some("Ceph dm-crypt OSD")
		}
		&[0x89, 0xC5, 0x7F, 0x98, 0x2F, 0xE5, 0x4D, 0xC0, 0x89, 0xC1, 0xF3, 0xAD, 0x0C, 0xEF, 0xF2, 0xBE] => {
			Some("Ceph Disk in creation")
		}
		&[0x89, 0xC5, 0x7F, 0x98, 0x2F, 0xE5, 0x4D, 0xC0, 0x89, 0xC1, 0x5E, 0xC0, 0x0C, 0xEF, 0xF2, 0xBE] => {
			Some("Ceph dm-crypt disk in creation")
		}
		&[0xCA, 0xFE, 0xCA, 0xFE, 0x9B, 0x03, 0x4F, 0x30, 0xB4, 0xC6, 0xB4, 0xB8, 0x0C, 0xEF, 0xF1, 0x06] => {
			Some("Ceph Block")
		}
		&[0x30, 0xCD, 0x08, 0x09, 0xC2, 0xB2, 0x49, 0x9C, 0x88, 0x79, 0x2D, 0x6B, 0x78, 0x52, 0x98, 0x76] => {
			Some("Ceph Block DB")
		}
		&[0x5C, 0xE1, 0x7F, 0xCE, 0x40, 0x87, 0x41, 0x69, 0xB7, 0xFF, 0x05, 0x6C, 0xC5, 0x84, 0x73, 0xF9] => {
			Some("Ceph Block write-ahead log")
		}
		&[0xFB, 0x3A, 0xAB, 0xF9, 0xD2, 0x5F, 0x47, 0xCC, 0xBF, 0x5E, 0x72, 0x1D, 0x18, 0x16, 0x49, 0x6B] => {
			Some("Ceph Lockbox for dm-crypt keys")
		}
		&[0x4F, 0xBD, 0x7E, 0x29, 0x8A, 0xE0, 0x49, 0x82, 0xBF, 0x9D, 0x5A, 0x8D, 0x86, 0x7A, 0xF5, 0x60] => {
			Some("Ceph Multipath OSD")
		}
		&[0x45, 0xB0, 0x96, 0x9E, 0x8A, 0xE0, 0x49, 0x82, 0xBF, 0x9D, 0x5A, 0x8D, 0x86, 0x7A, 0xF5, 0x60] => {
			Some("Ceph Multipath journal")
		}
		&[0xCA, 0xFE, 0xCA, 0xFE, 0x8A, 0xE0, 0x49, 0x82, 0xBF, 0x9D, 0x5A, 0x8D, 0x86, 0x7A, 0xF5, 0x60] => {
			Some("Ceph Multipath block")
		}
		&[0x7F, 0x4A, 0x66, 0x6A, 0x16, 0xF3, 0x47, 0xA2, 0x84, 0x45, 0x15, 0x2E, 0xF4, 0xD0, 0x3F, 0x6C] => {
			Some("Ceph Multipath block")
		}
		&[0xEC, 0x6D, 0x63, 0x85, 0xE3, 0x46, 0x45, 0xDC, 0xBE, 0x91, 0xDA, 0x2A, 0x7C, 0x8B, 0x32, 0x61] => {
			Some("Ceph Multipath block DB")
		}
		&[0x01, 0xB4, 0x1E, 0x1B, 0x00, 0x2A, 0x45, 0x3C, 0x9F, 0x17, 0x88, 0x79, 0x39, 0x89, 0xFF, 0x8F] => {
			Some("Ceph Multipath block write-ahead log")
		}
		&[0xCA, 0xFE, 0xCA, 0xFE, 0x9B, 0x03, 0x4F, 0x30, 0xB4, 0xC6, 0x5E, 0xC0, 0x0C, 0xEF, 0xF1, 0x06] => {
			Some("Ceph dm-crypt block")
		}
		&[0x93, 0xB0, 0x05, 0x2D, 0x02, 0xD9, 0x4D, 0x8A, 0xA4, 0x3B, 0x33, 0xA3, 0xEE, 0x4D, 0xFB, 0xC3] => {
			Some("Ceph dm-crypt block DB")
		}
		&[0x30, 0x6E, 0x86, 0x83, 0x4F, 0xE2, 0x43, 0x30, 0xB7, 0xC0, 0x00, 0xA9, 0x17, 0xC1, 0x69, 0x66] => {
			Some("Ceph dm-crypt block write-ahead log")
		}
		&[0x45, 0xB0, 0x96, 0x9E, 0x9B, 0x03, 0x4F, 0x30, 0xB4, 0xC6, 0x35, 0x86, 0x5C, 0xEF, 0xF1, 0x06] => {
			Some("Ceph dm-crypt LUKS journal")
		}
		&[0xCA, 0xFE, 0xCA, 0xFE, 0x9B, 0x03, 0x4F, 0x30, 0xB4, 0xC6, 0x35, 0x86, 0x5C, 0xEF, 0xF1, 0x06] => {
			Some("Ceph dm-crypt LUKS block")
		}
		&[0x16, 0x64, 0x18, 0xDA, 0xC4, 0x69, 0x40, 0x22, 0xAD, 0xF4, 0xB3, 0x0A, 0xFD, 0x37, 0xF1, 0x76] => {
			Some("Ceph dm-crypt LUKS block DB")
		}
		&[0x86, 0xA3, 0x20, 0x90, 0x36, 0x47, 0x40, 0xB9, 0xBB, 0xBD, 0x38, 0xD8, 0xC5, 0x73, 0xAA, 0x86] => {
			Some("Ceph dm-crypt LUKS block write-ahead log")
		}
		&[0x4F, 0xBD, 0x7E, 0x29, 0x9D, 0x25, 0x41, 0xB8, 0xAF, 0xD0, 0x35, 0x86, 0x5C, 0xEF, 0xF0, 0x5D] => {
			Some("Ceph dm-crypt LUKS OSD")
		}
		&[0x82, 0x4C, 0xC7, 0xA0, 0x36, 0xA8, 0x11, 0xE3, 0x89, 0x0A, 0x95, 0x25, 0x19, 0xAD, 0x3F, 0x61] => {
			Some("OpenBSD Data partition")
		}
		&[0xCE, 0xF5, 0xA9, 0xAD, 0x73, 0xBC, 0x46, 0x01, 0x89, 0xF3, 0xCD, 0xEE, 0xEE, 0xE3, 0x21, 0xA1] => {
			Some("QNX Power-safe (QNX6) file system")
		}
		&[0xC9, 0x18, 0x18, 0xF9, 0x80, 0x25, 0x47, 0xAF, 0x89, 0xD2, 0xF0, 0x30, 0xD7, 0x00, 0x0C, 0x2C] => {
			Some("Plan 9 Plan 9 partition")
		}
		&[0x9D, 0x27, 0x53, 0x80, 0x40, 0xAD, 0x11, 0xDB, 0xBF, 0x97, 0x00, 0x0C, 0x29, 0x11, 0xD1, 0xB8] => {
			Some("VMware ESX vmkcore (coredump partition)")
		}
		&[0xAA, 0x31, 0xE0, 0x2A, 0x40, 0x0F, 0x11, 0xDB, 0x95, 0x90, 0x00, 0x0C, 0x29, 0x11, 0xD1, 0xB8] => {
			Some("VMware VMFS filesystem partition")
		}
		&[0x91, 0x98, 0xEF, 0xFC, 0x31, 0xC0, 0x11, 0xDB, 0x8F, 0x78, 0x00, 0x0C, 0x29, 0x11, 0xD1, 0xB8] => {
			Some("VMware Reserved")
		}
		&[0x25, 0x68, 0x84, 0x5D, 0x23, 0x32, 0x46, 0x75, 0xBC, 0x39, 0x8F, 0xA5, 0xA4, 0x74, 0x8D, 0x15] => {
			Some("Android-IA Bootloader")
		}
		&[0x11, 0x4E, 0xAF, 0xFE, 0x15, 0x52, 0x40, 0x22, 0xB2, 0x6E, 0x9B, 0x05, 0x36, 0x04, 0xCF, 0x84] => {
			Some("Android-IA Bootloader2")
		}
		&[0x49, 0xA4, 0xD1, 0x7F, 0x93, 0xA3, 0x45, 0xC1, 0xA0, 0xDE, 0xF5, 0x0B, 0x2E, 0xBE, 0x25, 0x99] => {
			Some("Android-IA Boot")
		}
		&[0x41, 0x77, 0xC7, 0x22, 0x9E, 0x92, 0x4A, 0xAB, 0x86, 0x44, 0x43, 0x50, 0x2B, 0xFD, 0x55, 0x06] => {
			Some("Android-IA Recovery")
		}
		&[0xEF, 0x32, 0xA3, 0x3B, 0xA4, 0x09, 0x48, 0x6C, 0x91, 0x41, 0x9F, 0xFB, 0x71, 0x1F, 0x62, 0x66] => {
			Some("Android-IA Misc")
		}
		&[0x20, 0xAC, 0x26, 0xBE, 0x20, 0xB7, 0x11, 0xE3, 0x84, 0xC5, 0x6C, 0xFD, 0xB9, 0x47, 0x11, 0xE9] => {
			Some("Android-IA Metadata")
		}
		&[0x38, 0xF4, 0x28, 0xE6, 0xD3, 0x26, 0x42, 0x5D, 0x91, 0x40, 0x6E, 0x0E, 0xA1, 0x33, 0x64, 0x7C] => {
			Some("Android-IA System")
		}
		&[0xA8, 0x93, 0xEF, 0x21, 0xE4, 0x28, 0x47, 0x0A, 0x9E, 0x55, 0x06, 0x68, 0xFD, 0x91, 0xA2, 0xD9] => {
			Some("Android-IA Cache")
		}
		&[0xDC, 0x76, 0xDD, 0xA9, 0x5A, 0xC1, 0x49, 0x1C, 0xAF, 0x42, 0xA8, 0x25, 0x91, 0x58, 0x0C, 0x0D] => {
			Some("Android-IA Data")
		}
		&[0xEB, 0xC5, 0x97, 0xD0, 0x20, 0x53, 0x4B, 0x15, 0x8B, 0x64, 0xE0, 0xAA, 0xC7, 0x5F, 0x4D, 0xB1] => {
			Some("Android-IA Persistent")
		}
		&[0xC5, 0xA0, 0xAE, 0xEC, 0x13, 0xEA, 0x11, 0xE5, 0xA1, 0xB1, 0x00, 0x1E, 0x67, 0xCA, 0x0C, 0x3C] => {
			Some("Vendor")
		}
		&[0xBD, 0x59, 0x40, 0x8B, 0x45, 0x14, 0x49, 0x0D, 0xBF, 0x12, 0x98, 0x78, 0xD9, 0x63, 0xF3, 0x78] => {
			Some("Android-IA Config")
		}
		&[0x8F, 0x68, 0xCC, 0x74, 0xC5, 0xE5, 0x48, 0xDA, 0xBE, 0x91, 0xA0, 0xC8, 0xC1, 0x5E, 0x9C, 0x80] => {
			Some("Android-IA Factory")
		}
		&[0x9F, 0xDA, 0xA6, 0xEF, 0x4B, 0x3F, 0x40, 0xD2, 0xBA, 0x8D, 0xBF, 0xF1, 0x6B, 0xFB, 0x88, 0x7B] => {
			Some("Android-IA Factory (alt)")
		}
		&[0x76, 0x79, 0x41, 0xD0, 0x20, 0x85, 0x11, 0xE3, 0xAD, 0x3B, 0x6C, 0xFD, 0xB9, 0x47, 0x11, 0xE9] => {
			Some("Android-IA Fastboot / Tertiary")
		}
		&[0xAC, 0x6D, 0x79, 0x24, 0xEB, 0x71, 0x4D, 0xF8, 0xB4, 0x8D, 0xE2, 0x67, 0xB2, 0x71, 0x48, 0xFF] => {
			Some("Android-IA OEM")
		}
		&[0x19, 0xA7, 0x10, 0xA2, 0xB3, 0xCA, 0x11, 0xE4, 0xB0, 0x26, 0x10, 0x60, 0x4B, 0x88, 0x9D, 0xCF] => {
			Some("Android 6.0+ ARM Android Meta")
		}
		&[0x19, 0x3D, 0x1E, 0xA4, 0xB3, 0xCA, 0x11, 0xE4, 0xB0, 0x75, 0x10, 0x60, 0x4B, 0x88, 0x9D, 0xCF] => {
			Some("Android EXT")
		}
		&[0x74, 0x12, 0xF7, 0xD5, 0xA1, 0x56, 0x4B, 0x13, 0x81, 0xDC, 0x86, 0x71, 0x74, 0x92, 0x93, 0x25] => {
			Some("Open Network Install Environment (ONIE) Boot")
		}
		&[0xD4, 0xE6, 0xE2, 0xCD, 0x44, 0x69, 0x46, 0xF3, 0xB5, 0xCB, 0x1B, 0xFF, 0x57, 0xAF, 0xC1, 0x49] => {
			Some("Open Network Install Environment (ONIE) Config")
		}
		&[0x9E, 0x1A, 0x2D, 0x38, 0xC6, 0x12, 0x43, 0x16, 0xAA, 0x26, 0x8B, 0x49, 0x52, 0x1E, 0x5A, 0x8B] => {
			Some("PowerPC PReP boot")
		}
		&[0xBC, 0x13, 0xC2, 0xFF, 0x59, 0xE6, 0x42, 0x62, 0xA3, 0x52, 0xB2, 0x75, 0xFD, 0x6F, 0x71, 0x72] => {
			Some(
				"freedesktop.org OSes (Linux, etc.) Shared boot loader \
				 configuration",
			)
		}
		&[0x73, 0x4E, 0x5A, 0xFE, 0xF6, 0x1A, 0x11, 0xE6, 0xBC, 0x64, 0x92, 0x36, 0x1F, 0x00, 0x26, 0x71] => {
			Some("Atari TOS Basic data partition (GEM, BGM, F32)")
		}
		&[0x8C, 0x8F, 0x8E, 0xFF, 0xAC, 0x95, 0x47, 0x70, 0x81, 0x4A, 0x21, 0x99, 0x4F, 0x2D, 0xBC, 0x8F] => {
			Some("VeraCrypt Encrypted data partition")
		}
		&[0x90, 0xB6, 0xFF, 0x38, 0xB9, 0x8F, 0x43, 0x58, 0xA2, 0x1F, 0x48, 0xF3, 0x5B, 0x4A, 0x8A, 0xD3] => {
			Some("OS/2 ArcaOS Type 1")
		}
		_ => Some("Unknown"),
	}
}
