mod combinations;


fn main() 
{
	let plaintext = String::from("LES POTES DE YANNICK SONT ULTRA GENANTS");

	let vec: Vec<char> = vec!['A', 'D', 'F', 'G', 'V', 'X'];

    for comb in combinations::CombinationsWithRepetitions::new(&vec, 2) {
        for item in &comb {
            print!("{} ", item)
        }
        println!()
    }
}
