mod library;
use std::collections::HashMap;

fn find_key_for_value<'a>(map: &'a HashMap<char, Vec<char>>, value: &Vec<char>) -> Option<&'a char> {
	let mut to_return: Option<&char> = None;
	for (key, val) in map.into_iter() {
		if value == val {
			to_return = Some(&key);
		}
	}
	to_return
}


fn main() {
	let ciphered_text = String::from("FGFAD GDDFF FXFFX");
	let polybe_square = library::create_polybe_square();

	let secret = String::from("HOCUS");
	let pre_ciphered_text = library::build_preciphered_string(ciphered_text, &secret);
	println!("{}", pre_ciphered_text);
	let test: Vec<char> = pre_ciphered_text[0..2].chars().collect();
	let associated_char = find_key_for_value(&polybe_square, &test).unwrap();
	println!("{}", associated_char);




	

}





	
	


