use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("adfgvx.txt").expect("Couldn't open the file");
    let mut content = String::new();
    file.read_to_string(content);
    println!("{}", content);
}
