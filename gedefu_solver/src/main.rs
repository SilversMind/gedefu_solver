mod lib;

fn main() 
{
	let plaintext = String::from("TON PULL DE JOLIE");
	let ciphered_string: String = lib::polybe_encryption(plaintext);
	println!("{}", ciphered_string);


}