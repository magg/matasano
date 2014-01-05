/*
2. Fixed XOR

Write a function that takes two equal-length buffers and produces
their XOR sum.

The string:

 1c0111001f010100061a024b53535009181c

... after hex decoding, when xor'd against:

 686974207468652062756c6c277320657965

... should produce:

 746865206b696420646f6e277420706c6179

*/

extern mod extra;
extern mod std;
use std::{u8};

mod hex_base64;

fn xor_vectors(string1: ~str, string2: ~str) {
	
	let n = string1.len();
	let mut i = 0;
	
	if n != string2.len(){
		fail!(~"Fail! No equal size");
	}
		
	let s1 = hex_base64::base64_to_hex(string1);
	let s2 = hex_base64::base64_to_hex(string2);
	
	let temp1: ~str =  hex_base64::hex_to_base64(s1);
	let xs1: ~[u8] = temp1.as_bytes().to_owned();
	let temp2: ~str =  hex_base64::hex_to_base64(s2);
	let xs2: ~[u8] = temp2.as_bytes().to_owned();
	
	let mut res: ~str = ~"";
	
	let tam = xs1.len();
	while i < tam {
		
		let slice1 = u8::parse_bytes(xs1.slice(i, i+2), 16);
		let slice2 = u8::parse_bytes(xs2.slice(i, i+2), 16);
	
		match (slice1, slice2) {
		      (Some(a), Some(b)) => {
						let x: u8 = a ^ b;
						let y: ~str = x.to_str_radix(16);
						res.push_str(y);
					},
		      _ => { println("Error: bad value"); break;}
		    }
		i += 2;
	} 
	println(fmt!("%s", res)); 	

}

fn main () {
	
	let string1 = ~"1c0111001f010100061a024b53535009181c";
	let string2 = ~"686974207468652062756c6c277320657965";
	xor_vectors(string1,string2);
		
}