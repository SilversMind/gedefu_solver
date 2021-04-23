mod library;

fn main() {
	let plaintext = String::from("TON PTIT PULL DE JOLIE");
	let secret = String::from("HOCUS");
	let ciphered_text = library::encrypt_adfgvx(plaintext, secret);
	println!("Ciphered text is: {}", ciphered_text);
}





	
	


