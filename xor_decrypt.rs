/*
6. Break repeating-key XOR

The buffer at the following location:

 https://gist.github.com/3132752

is base64-encoded repeating-key XOR. Break it.

Here's how:

a. Let KEYSIZE be the guessed length of the key; try values from 2 to
(say) 40.

b. Write a function to compute the edit distance/Hamming distance
between two strings. The Hamming distance is just the number of
differing bits. The distance between:

  this is a test

and:

  wokka wokka!!!

is 37.

c. For each KEYSIZE, take the FIRST KEYSIZE worth of bytes, and the
SECOND KEYSIZE worth of bytes, and find the edit distance between
them. Normalize this result by dividing by KEYSIZE.

d. The KEYSIZE with the smallest normalized edit distance is probably
the key. You could proceed perhaps with the smallest 2-3 KEYSIZE
values. Or take 4 KEYSIZE blocks instead of 2 and average the
distances.

e. Now that you probably know the KEYSIZE: break the ciphertext into
blocks of KEYSIZE length.

f. Now transpose the blocks: make a block that is the first byte of
every block, and a block that is the second byte of every block, and
so on.

g. Solve each block as if it was single-character XOR. You already
have code to do this.

e. For each block, the single-byte XOR key that produces the best
looking histogram is the repeating-key XOR key byte for that
block. Put them together and you have the key.
*/

extern crate serialize;
extern crate std;

use serialize::base64::FromBase64;
use serialize::hex::{ToHex};
use std::io::BufferedReader;
use std::io::File;

pub struct XorCandidate {
    k: int,
    d: int
}

fn hamming_distance(string1: &str, string2: &str) -> int {
	let mut distance = 0;
	let xs1: ~[u8] = string1.as_bytes().to_owned();
	let xs2: ~[u8] = string2.as_bytes().to_owned();
	
	for i in range(0, xs1.len() ) {
		let mut val = xs1[i] ^ xs2[i];
		while val != 0 {
			distance += 1;
		    val = val & (val - 1);
		}
	}
	distance
}

fn guess_key_size(string: &str) -> uint {
	
	let mut iterations : ~[XorCandidate] = ~[XorCandidate { k: 0, d: 9999 }] ;
	
	for keysize in range(2, 41) {
		let key = keysize as uint;	
		let first = string.slice(0, key);
		let second = string.slice(key, key*2);
		let edit_distance = hamming_distance(first, second);
		let normalized_edit_distance = edit_distance  / keysize;
		let values : XorCandidate  =  XorCandidate { k: keysize, d: normalized_edit_distance };
		iterations.push(values);
	}
	iterations.sort_by(|a, b| a.d.cmp(&b.d));
	
	iterations[0].k as uint
	//println!("{} -> {:?}", iterations[0].k, iterations[0].d);
	
	
}

fn make_blocks(bytes: &[u8], key_size: uint) -> Vec<Vec<u8>> {
	let mut res = Vec::new();
	let mut size = 0;
		
	for block in bytes.chunks(key_size){
		//res.push(block);
		size = block.len();
		break;
	}
	
	for block in bytes.chunks(key_size){
		let mut b = Vec::from_elem(size, 0u8);
		let mut i = 0;
		let tam = block.len();
		for j in range(0,tam) {
			*b.get_mut(i) = block[j];
			i+=1;
		}
		res.push(b);
	}
	
	for num in res.iter() {
	    println!("{}", *num);
	}
	
	res
}

fn transpose_blocks(mat: Vec<Vec<u8>>){
	let n = mat.len();
	let m = mat.get(0).len();
	
	let mut res = Vec::new();
	let mut cont = 0;
	while cont < m {
		let mut b = Vec::from_elem(n, 0u8);
		res.push(b);
		cont +=1; 
	}
	
	
	
	for i in range(0, n){
		let ren = mat.get(i);
		for j in range(0, m){
			*res.get_mut(j).get_mut(i) = *ren.get(j);
		}
	}
	
	
	for num in res.iter() {
	    println!("{}", *num);
	}
}

fn decrypt_xor() { 
	
	let path = Path::new("gistfile2.txt");
	let mut file = BufferedReader::new(File::open(&path));
	for line in file.lines() {
		let linea = line.unwrap();
		let based_line = linea.slice_to(linea.len() - 1);
		let res = based_line.from_base64();
		let bytes: ~[u8] = res.unwrap();
		let string = bytes.to_hex();
		//println(format!("{:s}", string)); 
		let key_size = guess_key_size(string);
		let matrix = make_blocks(bytes,key_size);
		transpose_blocks(matrix);
		
		//for block in bytes.chunks(key_size) {   
		//let transposed = transpose_blocks(bytes,key_size);
		//println!("{:?}", transposed.to_hex() );
		//}
		break;
	}
}

fn main () {
	let s1 = ~"this is a test";
	let s2 = ~"wokka wokka!!!";
	let d = hamming_distance(s1,s2);
	println!("{:d}", d); 
	decrypt_xor();
	
	
}