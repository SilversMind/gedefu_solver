use std::collections::HashMap;
use itertools::Itertools;
pub mod util;
pub mod permutation;


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

fn polybe_encryption(mut plaintext: &String) -> String {
    util::string_operation::remove_whitespace(&mut plaintext);
    let translate_dictionary: HashMap<_, _> = create_polybe_square();
    let mut ciphered_string = String::new();
    let mut is_present: bool;
    for character in plaintext.chars() {
        let resulting_vec_char: Option<&Vec<char>> = translate_dictionary.get(&character);
        match resulting_vec_char{
            Some(_) => is_present = true, // There is a corresponding character in the dictionnary
            None => is_present = false,
        }
        if is_present == true{
            let result_string: String = resulting_vec_char.unwrap().iter().cloned().collect::<String>();
            ciphered_string.push_str(&result_string);
        }
        else {println!("Special character {} excluded", &character);}
    }
    
    println!("");
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

pub fn encrypt_adfgvx(plaintext: &String, secret: &String) -> String {
	
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

fn get_vectors_from_ciphertext(mut ciphered_text: &String, secret: &String) -> Vec<util::CharColumn> {
    let mut is_present: bool;
    let ciphered_text_cleaned: String = util::string_operation::remove_whitespace(&ciphered_text);
    let mut column_vec: Vec<util::CharColumn> = vec![];
    column_vec = init_column_vectors(column_vec, &secret);
    let expected_char_by_vec = ciphered_text_cleaned.len()/secret.len();
    let mut vec_iter = column_vec.iter_mut();

    let mut current_vec: &mut util::CharColumn = vec_iter.next().unwrap();
    for _char in ciphered_text_cleaned.chars() {
        if current_vec.vec_char().len() >= expected_char_by_vec {
            let temp = vec_iter.next();
            match temp {
                Some(_) => is_present = true,
                None => is_present = false,
            }
            if is_present == true {
                current_vec = temp.unwrap();
            }
        }
        current_vec.add_char_to_vec(_char);
    }
    column_vec

}

fn build_preciphered_string(ciphered_text: &String, secret: &String) -> String {
	let mut pre_ciphered_text = String::new();
	let secret_scrambled = util::string_operation::sort_str_by_alphabetical_order(&secret);
	let column_vec: Vec<util::CharColumn> = get_vectors_from_ciphertext(ciphered_text, &secret_scrambled);
	for index in 0..column_vec.first().unwrap().vec_char().len() {
		for char_ in secret.chars(){
			for vec_char_ in &column_vec{
				if vec_char_.key_char() == &char_ {
					pre_ciphered_text.push(vec_char_.vec_char()[index])
				}
			}
		}
	}
    pre_ciphered_text
}

fn find_key_for_value<'a>(map: &'a HashMap<char, Vec<char>>, value: &Vec<char>) -> Option<&'a char> {
	let mut to_return: Option<&char> = None;
	for (key, val) in map.into_iter() {
		if value == val {
			to_return = Some(&key);
		}
	}
	to_return
}

pub fn decrypt_adfgvx(ciphertext: &String, secret: &String) -> String{
    let mut is_present: bool;
	let polybe_square = create_polybe_square();
	let mut deciphered_text = String::new();
	let pre_ciphered_text = build_preciphered_string(ciphertext, &secret);
	for i in (0..pre_ciphered_text.len() - 1).step_by(2){
		let char_: Vec<char> =  pre_ciphered_text[i..i+2].chars().collect();
		let deciphered_char = find_key_for_value(&polybe_square, &char_);
        match deciphered_char {
            Some(_) => is_present = true,
            None => is_present = false
        }
		if is_present == true {
            deciphered_text.push(*deciphered_char.unwrap())
        }
	}
	
	deciphered_text
}

pub fn return_frequence(string: &String) -> Vec<(char, u32)>{
	let mut dictionnary: HashMap<char, u32> = HashMap::new();
	for char_ in string.chars(){
		*dictionnary.entry(char_).or_insert(0) += 1;
	}
	let cpt: u32 = dictionnary.values().sum();
	let sorted_vec = dictionnary.drain();
    let mut result_vec = sorted_vec.map(|item| (item.0,(item.1 *100)/cpt)).collect::<Vec<_>>();
	result_vec.sort_by_key(|k| k.1);
    result_vec.reverse();
    result_vec
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_adfgvx() {
        assert_eq!("FGFADGDDFFFXFFX", encrypt_adfgvx(&String::from("BONJOUR"), &String::from("HOCUS")));
        assert_eq!("BONJOUR", decrypt_adfgvx(&String::from("FGFADGDDFFFXFFX"), &String::from("HOCUS")));

        assert_eq!("FGFFGGAFFDXFDFAVFDAFDGDGDXVDXGFFAAFAAAFFVAXVAAXFGAFGGAADAXFADXVF", encrypt_adfgvx(&String::from("ON EST LA FRERO ON EST LA TU VA FAIRE QUOI"), &String::from("ABCDEFGH")));
        assert_eq!("ONESTLAFREROONESTLATUVAFAIREQUOI", decrypt_adfgvx(&String::from("FGFFGGAFFDXFDFAVFDAFDGDGDXVDXGFFAAFAAAFFVAXVAAXFGAFGGAADAXFADXVF"), &String::from("ABCDEFGH")))
    }
}
