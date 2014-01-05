extern mod extra;
extern mod std;
use std::{u8};

mod hex_base64;

fn single_xor () {

}

fn main(){
	let string = ~"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
	let temp1: ~str =  hex_base64::hex_to_base64(string);
	
	let asc1: [int,int::range(32,33)];
	
}