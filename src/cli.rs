use crate::{
	constants::{get_help_msg, DEFAULT_BLOCK_DEVICE, VERSION},
	reader,
	table::Table,
};
use anyhow::Result;
use std::io;

pub(crate) fn run(location: &str) -> Result<()> {
	let entries = reader::analyze(location)?;
	let table = entries.into_iter().collect::<Table>();

	table.render(io::stdout().lock())?;
	Ok(())
}

pub(crate) fn parse<I>(mut iter: I) -> Result<()>
where
	I: Iterator<Item = String>,
{
	if let Some(arg) = iter.nth(1) {
		match arg.as_str() {
			"-v" | "--version" => {
				println!("gptinfo {}", VERSION);
			}
			"-h" | "--help" => {
				println!("{}", get_help_msg());
			}
			"" => run(DEFAULT_BLOCK_DEVICE)?,
			loc => run(loc)?,
		}
	} else {
		run(DEFAULT_BLOCK_DEVICE)?;
	};

	Ok(())
}
