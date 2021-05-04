mod library;

pub fn decrypt_adfgvx(ciphertext: String, secret: String) -> String{
	let polybe_square = library::create_polybe_square();
	let mut deciphered_text = String::new();
	let pre_ciphered_text = library::build_preciphered_string(ciphertext, &secret);
	for i in (0..pre_ciphered_text.len() - 1).step_by(2){
		let char_: Vec<char> =  pre_ciphered_text[i..i+2].chars().collect();
		let deciphered_char = library::find_key_for_value(&polybe_square, &char_).unwrap();
		deciphered_text.push(*deciphered_char)
	}
	
	deciphered_text
}


fn main() {
	let ciphered_text = String::from("FGFAD GDDFF FXFFX");
	let secret = String::from("HOCUS");
	println!("{}", decrypt_adfgvx(ciphered_text, secret))

	




	

}





	
	


