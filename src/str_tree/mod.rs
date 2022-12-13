mod read_file;
mod tree_building;
pub use tree_building::StrTree;

pub trait Dictionnary {
	fn build_dict_from_file(filename: &str) -> std::io::Result<StrTree>;
	fn get_anagrams(&self, letter_set: &str) -> Vec<String>;
	fn add_word(&mut self, new_word: &str);
}

pub fn build_dict_from_file(filename: &str) -> std::io::Result<StrTree> {
	return StrTree::build_dict_from_file(filename);
}
