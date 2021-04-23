mod lib;

fn main() {
	let plaintext = String::from("TON PTIT PULL DE JOLIE");
	let secret = String::from("HOCUS");
	let ciphered_text = lib::encrypt_adfgvx(plaintext, secret);
	println!("Ciphered text is: {}", ciphered_text);
}





	
	


