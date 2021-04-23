mod lib;

fn main() 
{
	let plaintext = String::from("TON PULL DE JOLIE");
	let ciphered_string: String = lib::polybe_encryption(plaintext);
	println!("{}", ciphered_string);
	let secret = String::from("HOCUS");
	let mut column_vec: Vec<lib::util::CharColumn> = vec![];
	column_vec = lib::init_column_vectors(column_vec, &secret);
	column_vec = lib::fill_column_vectors(column_vec, ciphered_string, secret);

	for vec in column_vec{
		println!("{}", vec);
	}




	
	


}