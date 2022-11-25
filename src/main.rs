mod str_tree;
use str_tree::StrTree;

fn main() {
	let mut _tree = StrTree::init();

	if let Some(nb_words) = _tree.fill_with_file("../pyScrabble/scrabbleWords.txt") {
		println!("File read ! {} words found reported in reading lines", nb_words);
	} else {
		println!("File not found");
	}

	println!("{}", _tree.is_word("prout"));
	println!("{}", _tree.is_word("wu"));
	println!("{}", _tree.is_word("hjklhjkl"));
	println!("{}", _tree.is_word("mmm"));
	println!("{}", _tree.is_word("acquitterait"));


}
