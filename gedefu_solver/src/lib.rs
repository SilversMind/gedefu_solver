use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn openfile(filepath: &Path) { // Need: let path = Path::new("adfgvx.txt");
    // Create a path to the desired file
    let display = filepath.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&filepath) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}