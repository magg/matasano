/*
4. Detect single-character XOR

One of the 60-character strings at:

  https://gist.github.com/3132713

has been encrypted by single-character XOR. Find it. (Your code from
#3 should help.)

*/
extern mod extra;
extern mod std;

use std::trie::TrieMap;
use std::io::buffered::BufferedReader;
use std::io::File;

mod xor_cipher;

fn detect_xor() -> xor_cipher::Candidate { 
	let mut m = TrieMap::new();
	
	let path = Path::new("gistfile1.txt");
	let mut file = BufferedReader::new(File::open(&path));
	for line in file.lines() {
		let l = line.slice_to(line.len() - 1);	
		let res:  xor_cipher::Candidate = xor_cipher::xor_cipher_break(l);
		match res {
		   xor_cipher::Candidate(a, b, c) => {
			let value =  xor_cipher::Bucket { c: b, s: c };
			m.insert(a, value);
			//println!("{} -> {:?} -> {:?}", a, b, c);
			} 
		}
	}
	
	if !m.is_empty(){
		let mut it = m.iter().take(1);
		let mytup = it.next().unwrap().clone();
		match mytup {
		  (a, b) => {
			let c: xor_cipher::Candidate = xor_cipher::Candidate(a, (b.c).clone(), (b.s).clone() );
			return c;
			}
		}
	} else {
		fail!(~"Fail! No data");	
	}	
}

fn main () {
	let res = detect_xor();
	match res {
	  xor_cipher::Candidate(a, b, c) => println!("{} -> {:?} -> {:?}", a, b, c) 
	}
}