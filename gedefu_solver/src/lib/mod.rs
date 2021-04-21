use std::collections::HashMap;
use itertools::Itertools;
pub mod util;


fn create_polybe_square() -> HashMap<char, Vec<char>> {
    let vec: Vec<char> = vec!['A', 'D', 'F', 'G', 'V', 'X'];
    let polybe_square = util::Permutations::permutations(&vec, 2);
    let polybe_vec : Vec<_> = polybe_square.into_iter().collect();
    let alphabet: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0','1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let translate_dictionary: HashMap<_, _> =
        alphabet.into_iter().zip_eq(polybe_vec.into_iter()).collect();
    return translate_dictionary;
}

pub fn polybe_encryption(mut plaintext: String) -> String {
	util::remove_whitespace(&mut plaintext);
	let translate_dictionary: HashMap<_, _> = create_polybe_square();
	
	let mut ciphered_string = String::new();
	for character in plaintext.chars() {
		let resulting_vec_char: &Vec<char> = translate_dictionary.get(&character).unwrap();
		let result_string: String = resulting_vec_char.iter().cloned().collect::<String>	();
		ciphered_string.push_str(&result_string);
	}
	return ciphered_string;
}
