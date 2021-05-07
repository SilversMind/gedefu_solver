mod library;
use std::fs;
use itertools::Itertools;



fn main() {
	let key_lengths: usize = 10;
	let alphabet = String::from("ABCDEFGHIJ");
	let mut keys: Vec<String> = vec![];
	for key_length in 1..key_lengths{
		let to_permute = &alphabet[0..key_length];
		for key in to_permute.chars().permutations(key_length).unique(){
			keys.push(key.iter().collect());
		}
	}


	// println!("{:?}", &perms);
    let contents = &mut fs::read_to_string("/Users/bouzdi/Hacking/chall_rm/gedefu_solver/gedefu_solver/src/adfgvx.txt")
        .expect("Something went wrong reading the file");

	let secret = String::from("HOCUS");
	let ref ciphered_text = library::encrypt_adfgvx(&contents, &secret);
	let mut frequency: Vec<(char, u32)>;
	let mut current_key_size: usize = 0;
	for key in keys {
		if current_key_size != key.len(){
			println!("Testing key of size {}", key.len());
		}
		current_key_size = key.len();


		let deciphered_text = library::decrypt_adfgvx(&contents, &key);
		frequency = library::return_frequence(&deciphered_text);
		if frequency.first().unwrap().1 > 14 {
			println!("Values {:?} for key {}", frequency, key);
		}
	}

}





	
	


