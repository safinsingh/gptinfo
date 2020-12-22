use const_format::formatcp;

pub(crate) const MBR_OFFSET: u64 = 512;
pub(crate) const GPT_HEADER_SIG: u64 = 0x5452415020494645;
pub(crate) const DEFAULT_BLOCK_DEVICE: &str = "/dev/sda";

pub(crate) const VERSION: &str = "v2.1.0";
pub(crate) const fn get_help_msg() -> &'static str {
	formatcp!(
		r#"gptinfo {}

USAGE:
	gptinfo [-h/-v] <device>

FLAGS:
	-h, --help    - display this help message
	-v, --version - print the current version of `gptinfo`

ARGUMENTS:
	device        - block device to read GPT from (default: /dev/sda)
"#,
		VERSION
	)
}
