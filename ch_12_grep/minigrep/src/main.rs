use std::env;
use std::fs;

fn main() {
	// Reading the Command Line Argument Values
	let args: Vec<String> = env::args().collect();

	let query = &args[1];
	let file_path = &args[2];

	print!("Searching for {} \n", query);
	print!("In file {}", file_path);

	// Reading a File
	let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
	println!("With text:\n{contents}");
}
