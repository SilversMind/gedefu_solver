mod library;
use std::fs;
use itertools::Itertools;



fn main() {

	let alphabet = String::from("ABCDEFGHIJ");
	let key_length: usize = 5;
	let to_permute = &alphabet[0..key_length];
	let mut keys: Vec<String> = vec![];
	for key in to_permute.chars().permutations(key_length).unique(){
		keys.push(key.iter().collect());
	}


	// println!("{:?}", &perms);
    let contents = &mut fs::read_to_string("/Users/bouzdi/Hacking/chall_rm/gedefu_solver/gedefu_solver/src/test.txt")
        .expect("Something went wrong reading the file");

	let secret = String::from("HOCUS");
	let secret1 = String::from("HOCUS");
	let ref ciphered_text = library::encrypt_adfgvx(&contents, &secret);
	let mut frequency: Vec<(char, u32)>;
	for key in keys {
		let deciphered_text = library::decrypt_adfgvx(ciphered_text, &key);
		frequency = library::return_frequence(&deciphered_text);
		if frequency.first().unwrap().1 > 10 {
			println!("Values {:?} for key {}", frequency, key);
		}
	}

}





	
	


