mod read_file;
pub use read_file::cnt_lines;
pub use read_file::read_lines;

mod dict_properties;
pub use dict_properties::Dictionnary;
pub use dict_properties::{ConstraintLetters, ConstraintNbLetters};
pub use dict_properties::{ConstraintLettersTrait, ConstraintNbLettersTrait};

mod tree_building;
pub use tree_building::StrTree;

pub fn build_dict_from_file(filename: &str) -> std::io::Result<StrTree> {
	return StrTree::build_dict_from_file(filename);
}
