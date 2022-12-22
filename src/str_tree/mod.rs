mod read_file;
pub use read_file::cnt_lines;
pub use read_file::read_lines;

mod tree_building;
pub use tree_building::StrTree;

pub use crate::constraints::{ConstraintNbLetters, ConstraintLetters};

pub trait Dictionnary<T> {
	fn build_dict_from_file(filename: &str) -> std::io::Result<T>;
	
	fn get_anagrams<CNbL, CL>(&self, letter_set: &str, nb_letter: CNbL, letter_constraints: CL) -> Vec<String>
	where CNbL: ConstraintNbLetters, CL: ConstraintLetters;

	fn add_word(&mut self, new_word: &str);
	fn is_word(&self, word: &str) -> bool;
}

pub fn build_dict_from_file(filename: &str) -> std::io::Result<StrTree> {
	return StrTree::build_dict_from_file(filename);
}
