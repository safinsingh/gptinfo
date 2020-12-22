use crate::{
	constants::{HELP_MSG, VERSION},
	reader::Reader,
	table::Table,
};
use anyhow::Result;

pub(crate) fn run(location: Option<&str>) -> Result<()> {
	let reader = Reader::new(location.unwrap_or("/dev/sda"));
	let entries = reader.analyze()?;
	let table = Table::from(entries.into_iter());

	table.render();
	Ok(())
}

pub(crate) fn parse<I>(mut iter: I) -> Result<()>
where
	I: Iterator<Item = String>,
{
	Ok(if let Some(arg) = iter.nth(1) {
		match arg.as_str() {
			"-v" | "--version" => {
				println!("gptinfo {}", VERSION);
			}
			"-h" | "--help" => {
				println!("{}", HELP_MSG);
			}
			"" => run(None)?,
			loc => run(Some(loc))?,
		}
	} else {
		run(None)?;
	})
}
