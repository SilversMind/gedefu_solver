use std::collections::HashMap;
use itertools::Itertools;
pub mod util;


pub fn create_polybe_square() -> HashMap<char, Vec<char>> {
    let vec: Vec<char> = vec!['A', 'D', 'F', 'G', 'V', 'X'];
    let polybe_square = util::Permutations::permutations(&vec, 2);
    let polybe_vec : Vec<_> = polybe_square.into_iter().collect();
    let alphabet: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0','1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let translate_dictionary: HashMap<_, _> =
        alphabet.into_iter().zip_eq(polybe_vec.into_iter()).collect();
    return translate_dictionary;
}