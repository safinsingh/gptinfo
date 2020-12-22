use std::env;

fn main() {
	if !env::var("TARGET")
		.unwrap_or_default()
		.ends_with("linux-gnu")
	{
		panic!("This crate cannot be compiled for non-Unix-based targets!");
	}
}
