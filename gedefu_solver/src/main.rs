mod library;


fn main() {
	let plaintext = String::from("LES COUILLES");
	let secret = String::from("HOCUS");
	let secret1 = String::from("HOCUS");
	let ciphered_text = library::encrypt_adfgvx(plaintext, secret);
	println!("{}", library::decrypt_adfgvx(ciphered_text, secret1))

	




	

}





	
	


