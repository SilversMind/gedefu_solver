mod library;
use std::collections::HashMap;


fn main() {
	let plaintext = String::from("LES COUILLES");
	let secret = String::from("HOCUS");
	let secret1 = String::from("HOCUS");
	let ciphered_text = library::encrypt_adfgvx(plaintext, secret);
	let deciphered_text = library::decrypt_adfgvx(String::from("FGFFGGAFFDXFDFAVFDAFDGDGDXVDXGFFAAFAAAFFVAXVAAXFGAFGGAADAXFADXVF"), String::from("ABCDEFGH"));
	let mut dictionnary: HashMap<char, u16> = HashMap::new();
	for char_ in deciphered_text.chars(){
		*dictionnary.entry(char_).or_insert(0) += 1;
	}
	let cpt: u16 = dictionnary.values().sum();
	let mut sorted_vec = dictionnary.iter().map(|item| (item.0,(item.1 *100)/cpt));
	let temp = sorted_vec.collect::<Vec<_>>();
	println!("{:?}", temp);


	




	

}





	
	


