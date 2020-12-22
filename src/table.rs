use crate::{fmt::convert as prettify_bytes, reader::Entry};
use anyhow::Result;
use std::{io::Write, iter::FromIterator};

pub(crate) struct Table {
	columns: usize,
	cells: Vec<String>,
}

impl FromIterator<Entry> for Table {
	fn from_iter<T: IntoIterator<Item = Entry>>(iter: T) -> Self {
		let mut new = Self {
			columns: 5,
			cells: vec![
				// Table labels
				String::from("Start"),
				String::from("End"),
				String::from("Sectors"),
				String::from("Size"),
				String::from("Type"),
			],
		};

		for entry in iter {
			new.cells.push(entry.start.to_string());
			new.cells.push(entry.end.to_string());
			// The number of sectors is the total number of LBAs between the
			// upper and lower bounds of the partition
			new.cells.push((entry.end - entry.start + 1).to_string());
			new.cells.push(prettify_bytes(
				((entry.end - entry.start + 1) * 512) as f64,
			));
			new.cells.push(entry.ty.to_string());
		}

		new
	}
}

impl Table {
	pub(crate) fn render<W: Write>(&self, mut writer: W) -> Result<()> {
		let mut widths = vec![0usize; self.columns];

		// Calculate maximum widths for each column
		for (idx, cell) in self.cells.iter().enumerate() {
			let col = {
				if idx == self.columns - 1 {
					idx
				} else {
					idx % self.columns
				}
			};

			if widths[col] < cell.len() {
				widths[col] = cell.len() + 1
			}
		}

		write!(writer, "╭")?;
		for (col, row_width) in widths.iter().enumerate() {
			for _ in 0..*row_width + 1 {
				write!(writer, "─")?;
			}
			if col != self.columns - 1 {
				write!(writer, "┬")?;
			}
		}
		writeln!(writer, "╮")?;

		for (idx, cell) in self.cells.iter().enumerate() {
			// Print separator after first row
			if idx == self.columns {
				write!(writer, "├")?;
				for (idx, len) in widths.iter().enumerate() {
					for _ in 0..*len + 1 {
						write!(writer, "─")?;
					}
					if idx != self.columns - 1 {
						write!(writer, "┼")?;
					} else {
						write!(writer, "┤")?;
					}
				}
				writeln!(writer)?;
			}

			let col = {
				if idx == self.columns - 1 {
					idx
				} else {
					idx % self.columns
				}
			};

			write!(writer, "│ {}", cell)?;
			for _ in 0..(widths[col] - cell.len()) {
				write!(writer, " ")?;
			}

			if col == self.columns - 1 {
				writeln!(writer, "│")?;
			}
		}

		write!(writer, "╰")?;
		for (col, row_width) in widths.iter().enumerate() {
			for _ in 0..*row_width + 1 {
				write!(writer, "─")?;
			}
			if col != self.columns - 1 {
				write!(writer, "┴")?;
			}
		}
		writeln!(writer, "╯")?;

		Ok(())
	}
}
