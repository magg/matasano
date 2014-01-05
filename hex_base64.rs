/*
1. Convert hex to base64 and back.

The string:

  49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d

should produce:

  SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t

Now use this code everywhere for the rest of the exercises. Here's a
simple rule of thumb:

  Always operate on raw bytes, never on encoded strings. Only use hex
  and base64 for pretty-printing.
*/

extern mod extra;
extern mod std;

use std::{vec,str};
use std::num::from_str_radix;
use extra::base64::{ToBase64,STANDARD};
use extra::base64::FromBase64;

pub fn hex_to_base64(hex_string: ~str) -> ~str {
	let n = hex_string.len();
	let tot = n / 2;
	let mut xs: ~[u8]  = vec::from_elem(tot, 0u8);

	let mut i = 0;

	while i < n {
		let x: Option<int> = from_str_radix( hex_string.slice(i,i+2), 16);
		match x {
	        Some(number_string) => xs[i/2] = number_string as u8,
	        _ => { println("Error: bad value"); break;}
	   }

		i += 2;
	}
	//xs
	let string = xs.to_base64(STANDARD);
	string
	//println(fmt!("%s", string));

}

pub fn base64_to_hex(base64_string: ~str) -> ~str {
	let res = base64_string.from_base64();
	let bytes = res.unwrap();
	let n = bytes.len();
	let tot = n * 2;
	let mut i = 0;
	let mut v;
	let mut xs: ~[char]  = vec::from_elem(tot, '0');
	let hex = @['0','1','2','3','4','5','6','7','8','9','A','B','C','D','E','F'];

	while i < n {
		v = bytes[i] & 0xFF;
		xs[i * 2] = hex[v >> 4];
		xs[i * 2 + 1] = hex[v & 0x0F];
		i += 1;
	}
	str::from_chars(xs)
	//println( str::from_chars(xs) ); 
}

fn main () {
    let string1 = ~"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
	let string2 = ~"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
	println( hex_to_base64(string1) );
	println( base64_to_hex(string2) );
	
}