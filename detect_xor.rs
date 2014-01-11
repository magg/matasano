/*
4. Detect single-character XOR

One of the 60-character strings at:

  https://gist.github.com/3132713

has been encrypted by single-character XOR. Find it. (Your code from
#3 should help.)

*/

use std::io::buffered::BufferedReader;
use std::io::File;

fn detect_xor() { 
	
	let path = Path::new("gistfile1.txt");
	let mut file = BufferedReader::new(File::open(&path));
	for line in file.lines() {
	    print!("{}", line);
	}	
}

fn main () {
	 detect_xor();
}