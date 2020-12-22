pub(crate) const MBR_OFFSET: u64 = 512;
pub(crate) const GPT_HEADER_SIG: u64 = 0x5452415020494645;
pub(crate) const VERSION: &str = "v2.0.0";
pub(crate) const HELP_MSG: &str = r#"gptinfo v2.0.0

USAGE:
	gptinfo [-h/-v] <device>

FLAGS:
	-h, --help    - display this help message
	-v, --version - print the current version of `gptinfo`

ARGUMENTS:
	device        - block device to read GPT from (default: /dev/sda)
"#;
