/*
3. Single-character XOR Cipher

The hex encoded string:

      1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736

... has been XOR'd against a single character. Find the key, decrypt
the message.

Write code to do this for you. How? Devise some method for "scoring" a
piece of English plaintext. (Character frequency is a good metric.)
Evaluate each output and choose the one with the best score.

Tune your algorithm until this works.

*/
extern crate collections;
extern crate serialize;
extern crate std;

use collections::hashmap::HashMap;
use collections::trie::TrieMap;
use serialize::hex::{FromHex, ToHex};
use std::{str,slice};
use std::char::from_u32;
use std::num::abs;

mod fixed_xor;

pub struct Bucket {
    c: ~str,
    s: ~str
}

pub struct Candidate(uint, ~str, ~str);

fn frequency_analysis(string: &str) -> int {
	let mut score: f32 = 0.0;
	let mut map = HashMap::<char, int>::new();	
	let freq: ~[f32] = ~[8.167,1.492,2.782,4.253,12.70,2.228,2.015,6.094,6.966,0.153,0.772,4.025,2.406,6.749,7.507,1.929,0.095,5.987,6.327,9.056,2.758,0.978,2.360,0.150,1.974,0.074,21.467];	
	let mut freq_per: ~[f32] = slice::from_elem(27, 0.0f32);;
	
	let res_string = convert_to_lowercase(string);
	
	let n = res_string.len();	
 	for i in range(0, n) {
		let c = res_string[i] as char;
		map.insert_or_update_with(c, 1, |_, val| *val += 1);
	}

	for i in range(0, 27) {
		let c = i as u32;
        let cr = c + 'a' as u32;
		let mut ch = from_u32(cr).unwrap();
		let mut mapv_f32 = 0.0;
		if i == 26 {
			ch = ' '; 
		}
		if map.contains_key(&ch){
			let map_val = map.get(&ch);
			mapv_f32 = *map_val as f32;
		}
		
		let n_f32 = n as f32;
		freq_per[i] = 100.0 * mapv_f32 / n_f32;
		//println(fmt!("%c %9.6f", ch, freq_per[i] ));
		score +=  abs(freq[i] - freq_per[i]);	
	}
	
/*
	for (k, v) in map.iter() {
	   println!("{} -> {:?}", *k, *v);
	}
*/
	//println(fmt!("%d", score.round() as int ));	
	score.round() as int	
}

fn convert_to_lowercase(string: &str) -> ~str {
 	let n = string.len();	
	let mut xs: ~[u8]  = slice::from_elem(n, 0u8);
	for i in range(0, n) {
		let c = string[i] as char;
		//if !c.is_ascii() {
		//	 c = '0';
		//}
		let cl = c.to_ascii().to_lower().to_byte();
		xs[i] = cl;
	}
		
	let result_str = str::from_utf8_owned(xs).unwrap();
	//println(fmt!("%s",result_str));
 
	result_str
}

fn get_char_from_int( num: int) -> ~str{
	let num_byte = num as u8;
	let num_char = num_byte as char;
	let s: ~str = str::from_char(num_char);
	s
}


fn single_xor(cyphertext: &str, num: int) -> ~str {
	let n = cyphertext.len();
	let s = get_char_from_int(num);
	let temp: ~str = s.repeat(n/ 2);
	let hex_string = temp.as_bytes().to_hex();			
	let res: ~str = fixed_xor::xor_vectors(cyphertext, hex_string );
	res
}

fn replace_invalid_utf8( bytes: &mut [u8]) {
	let mut i = 0;
	let size = bytes.len();
	
	while i < size {		
		if !str::is_utf8([bytes[i]]){
			//println(format!("invalid {:d}", num as int ));	
			bytes[i] = '0' as u8;
		} 	
	i+=1;
	}
}

pub fn xor_cipher_break(cyphertext: &str) -> Candidate  {
	let mut m = TrieMap::new();
	for i in range(0, 128) {
		let mut res: ~str = single_xor(cyphertext, i);		
		if res.len() % 2 != 0 {
			res = str::from_char('0') + res;
		}
	
		let mut bytes = res.from_hex().unwrap();
		replace_invalid_utf8(bytes);
		let result_str = str::from_utf8_owned(bytes).unwrap();		 	
		let score = frequency_analysis( result_str );
		let key = score as uint;
		let value =  Bucket { c: get_char_from_int(i), s: result_str };
		m.insert(key, value);

	}
	
	if !m.is_empty(){
		let mut it = m.iter().take(1);
		let mytup = it.next().unwrap().clone();
		match mytup {
		  (a, b) => {
			let c: Candidate = Candidate(a, (b.c).clone(), (b.s).clone() );
			return c;
			}
		}
	} else {
		fail!(~"Fail! No data");	
	}
}

fn main(){
	let string = ~"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
	//let string = ~"7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f";
	let res: Candidate = xor_cipher_break(string);
	match res {
	  Candidate(a, b, c) => println!("{} -> {:?} -> {:?}", a, b, c) 
	}	
}