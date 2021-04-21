use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

/*pub fn openfile(filepath: &Path) { // Need let path = Path::new("adfgvx.txt");
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
}*/

pub mod Permutations {
    pub struct PermutationIterator<'a, T: 'a> {
        universe: &'a [T],
        size: usize,
        prev: Option<Vec<usize>>,
    }

    pub fn permutations<T>(universe: &[T], size: usize) -> PermutationIterator<T> {
        PermutationIterator {
            universe,
            size,
            prev: None,
        }
    }

    pub fn map<T>(values: &[T], ixs: &[usize]) -> Vec<T>
    where
        T: Clone,
    {
        ixs.iter().map(|&i| values[i].clone()).collect()
    }

    impl<'a, T> Iterator for PermutationIterator<'a, T>
    where
        T: Clone,
    {
        type Item = Vec<T>;

        fn next(&mut self) -> Option<Vec<T>> {
            let n = self.universe.len();

            if n == 0 {
                return None;
            }

            match self.prev {
                None => {
                    let zeroes: Vec<usize> = std::iter::repeat(0).take(self.size).collect();
                    let result = Some(map(self.universe, &zeroes[..]));
                    self.prev = Some(zeroes);
                    result
                }
                Some(ref mut indexes) => match indexes.iter().position(|&i| i + 1 < n) {
                    None => None,
                    Some(position) => {
                        for index in indexes.iter_mut().take(position) {
                            *index = 0;
                        }
                        indexes[position] += 1;
                        Some(map(self.universe, &indexes[..]))
                    }
                },
            }
        }
    }
}
    