fn main() {
	#[cfg(not(unix))]
	compile_error!("This crate cannot be compiled on non-Unix-based systems!");
}
