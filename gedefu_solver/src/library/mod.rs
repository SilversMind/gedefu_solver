use std::collections::HashMap;
use itertools::Itertools;
pub mod util;
pub mod permutation;

pub fn encrypt_adfgvx(plaintext: String, secret: String) -> String {
	
	let ciphered_string: String = polybe_encryption(plaintext);
	let mut column_vec: Vec<util::CharColumn> = vec![];
	column_vec = init_column_vectors(column_vec, &secret);
	column_vec = fill_column_vectors(column_vec, ciphered_string, &secret);
	
	&column_vec.sort();
	let mut ciphered_text = String::new();
	for vec in column_vec {
		for _char in vec.vec_char() {
			ciphered_text.push(*_char);
		}
	}
	ciphered_text
}


fn create_polybe_square() -> HashMap<char, Vec<char>> {
    let vec: Vec<char> = vec!['A', 'D', 'F', 'G', 'V', 'X'];
    let polybe_square = permutation::permutations(&vec, 2);
    let mut polybe_vec : Vec<_> = polybe_square.into_iter().collect();
    &polybe_vec.sort();
    let alphabet: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0','1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let translate_dictionary: HashMap<_, _> =
        alphabet.into_iter().zip_eq(polybe_vec.into_iter()).collect();
    translate_dictionary
}

fn polybe_encryption(mut plaintext: String) -> String {
	util::string_operation::remove_whitespace(&mut plaintext);
	let translate_dictionary: HashMap<_, _> = create_polybe_square();
	
	let mut ciphered_string = String::new();
	for character in plaintext.chars() {
		let resulting_vec_char: &Vec<char> = translate_dictionary.get(&character).unwrap();
		let result_string: String = resulting_vec_char.iter().cloned().collect::<String>	();
		ciphered_string.push_str(&result_string);
	}
	return ciphered_string;
}

fn init_column_vectors(mut column_vec: Vec<util::CharColumn> ,secret: &String) -> Vec<util::CharColumn> {
    for char in secret.chars(){
        column_vec.push(util::CharColumn::new(char));
    }
    column_vec
}

fn fill_column_vectors(mut column_vec: Vec<util::CharColumn>, ciphered_string: String, secret: &String) -> Vec<util::CharColumn> {

    let mut cpt: usize = 0;
    while (cpt < ciphered_string.len()) || ((cpt%secret.len()) != 0 ){

        // While not all the ciphered string has been stored in vectors
        // and while all vector has not the same size (i.e counter not 
        //	multiple of secret length
        
        let mut temp: char = 'X';
        if cpt < ciphered_string.len() {
            temp = ciphered_string.as_bytes()[cpt] as char;
        }

        let column_to_modify: &mut util::CharColumn = &mut column_vec[cpt%(secret.len())];
        column_to_modify.add_char_to_vec(temp);
        cpt = cpt + 1;
    }
    column_vec
}
