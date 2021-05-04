use std::fmt;
use std::iter::Iterator;
use std::iter::FromIterator;

pub mod string_operation{

    pub fn remove_whitespace(s: &mut String) {
        s.retain(|c| !c.is_whitespace());
    }
    pub fn sort_str_by_alphabetical_order(s: &String) -> String {
        let s_slice: &str = &s[..];
    
        let mut chars: Vec<char> = s_slice.chars().collect();
        chars.sort_by(|a, b| a.cmp(b));
    
        let s_sorted: String = chars.into_iter().collect();
        s_sorted
    }

}
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CharColumn {
    key_char: char,
    vec_char: Vec<char>,
}
impl CharColumn {
    
    pub fn new(new_char: char) -> CharColumn {
        CharColumn {key_char: new_char, vec_char: vec![] }
    }

    pub fn key_char(&self) -> &char {
        &self.key_char
    }

    pub fn vec_char(&self) -> &Vec<char> {
        &self.vec_char
    }
    pub fn add_char_to_vec(&mut self, char_to_add: char) {
        &self.vec_char.push(char_to_add);
    }
}
impl std::fmt::Display for CharColumn{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{} {:?}", self.key_char, self.vec_char)
    }
}
    