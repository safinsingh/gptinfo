use crate::{entry::Entry, fmt::format_bytes};

pub(crate) struct Table {
	columns: usize,
	cells: Vec<String>,
}

impl<'a> Table {
	pub(crate) fn from<I>(iter: I) -> Table
	where
		I: Iterator<Item = Entry>,
	{
		let mut new = Self {
			columns: 5,
			cells: Vec::new(),
		};

		// Table labels
		new.cells.push("Start".into());
		new.cells.push("End".into());
		new.cells.push("Sectors".into());
		new.cells.push("Size".into());
		new.cells.push("Type".into());

		for entry in iter {
			new.cells.push(entry.start.to_string());
			new.cells.push(entry.end.to_string());
			// The number of sectors is the total number of LBAs between the
			// upper and lower bounds of the partition
			new.cells.push((entry.end - entry.start + 1).to_string());
			new.cells.push(format_bytes(entry.end, entry.start));
			new.cells.push(entry.ty.to_string());
		}

		new
	}

	pub(crate) fn render(&self) {
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

		print!("╭");
		for (col, row_width) in widths.iter().enumerate() {
			for _ in 0..*row_width + 1 {
				print!("─")
			}
			if col != self.columns - 1 {
				print!("┬");
			}
		}
		print!("╮\n");

		for (idx, cell) in self.cells.iter().enumerate() {
			// Print separator after first row
			if idx == self.columns {
				print!("├");
				for (idx, len) in widths.iter().enumerate() {
					for _ in 0..*len + 1 {
						print!("─")
					}
					if idx != self.columns - 1 {
						print!("┼")
					} else {
						print!("┤");
					}
				}
				print!("\n");
			}

			let col = {
				if idx == self.columns - 1 {
					idx
				} else {
					idx % self.columns
				}
			};

			print!("│ {}", cell);
			for _ in 0..(widths[col] - cell.len()) {
				print!(" ");
			}

			if col == self.columns - 1 {
				print!("│\n");
			}
		}

		print!("╰");
		for (col, row_width) in widths.iter().enumerate() {
			for _ in 0..*row_width + 1 {
				print!("─")
			}
			if col != self.columns - 1 {
				print!("┴");
			}
		}
		println!("╯");
	}
}
