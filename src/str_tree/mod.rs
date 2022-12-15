mod read_file;
mod tree_building;
pub use tree_building::StrTree;

pub trait Dictionnary {
	fn build_dict_from_file(filename: &str) -> std::io::Result<StrTree>;
	fn get_anagrams(&self, letter_set: &str, nb_letter: Option<Vec<u8>>, letter_constraints: Option<Vec<(u8, char)>>) -> Vec<String>;
	fn add_word(&mut self, new_word: &str);
	fn is_word(&self, word: &str) -> bool;
}

pub fn build_dict_from_file(filename: &str) -> std::io::Result<StrTree> {
	return StrTree::build_dict_from_file(filename);
}
