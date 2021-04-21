mod permutations;
mod lib;
use std::collections::HashMap;
use itertools::Itertools;



fn main() 
{
	let plaintext = String::from("TON PULL DE JOLIE");
	let translate_dictionary: HashMap<_, _> = lib::create_polybe_square();

	
	for (key, value) in &translate_dictionary {
		println!("{}, {:?}", key, value)
	}

}