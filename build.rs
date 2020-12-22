use std::env;

fn main() {
	if !env::var("TARGET")
		.unwrap_or_default()
		.ends_with("linux-gnu")
	{
		panic!("This crate cannot be compiled on non-Unix-based systems!");
	}
}
