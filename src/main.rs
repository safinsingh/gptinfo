mod cli;
mod constants;
mod entry;
mod fmt;
mod guid;
mod header;
mod partition;
mod reader;
mod table;

use anyhow::Result;
use std::env;

fn main() -> Result<()> {
	cli::parse(env::args())
}
