/*
5. Repeating-key XOR Cipher

Write the code to encrypt the string:

  Burning 'em, if you ain't quick and nimble
  I go crazy when I hear a cymbal

Under the key "ICE", using repeating-key XOR. It should come out to:

  0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f

Encrypt a bunch of stuff using your repeating-key XOR function. Get a
feel for it.

*/

extern crate std;

use std::{slice,str};

fn xor_encrypt(plaintext: &str, key: &str) -> ~str {
	let bytes: ~[u8] = plaintext.as_bytes().to_owned();
	let n = bytes.len();
	let mut i = 0;
	let mut v;
	let tot = n * 2;
	let mut xs: ~[char]  = slice::from_elem(tot, '0');
	let hex = ['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f'];
	let len_k = key.len();
	
	while i < n {
		v = bytes[i] ^ key[i%len_k];
		xs[i * 2] = hex[v >> 4];
		xs[i * 2 + 1] = hex[v & 0x0F];
		i += 1;
	}
	str::from_chars(xs)
	//println( str::from_chars(xs) ); 
	
	
}

fn main () {
	let key = ~"ICE";
	let plain_text = 
~"Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
	println!( "{}", xor_encrypt(plain_text,key) );
	
}