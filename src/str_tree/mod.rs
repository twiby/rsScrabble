mod read_file;
mod tree_building;
use tree_building::StrTree;

pub trait Dictionnary {
	fn build_dict_from_file(filename: &str) -> std::io::Result<StrTree>;
	fn get_anagrams(&self, letter_set: &str) -> Vec<String>;
}

pub fn build_dict_from_file(filename: &str) -> std::io::Result<StrTree> {
	return tree_building::StrTree::build_dict_from_file(filename);
}
