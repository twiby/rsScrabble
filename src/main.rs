mod str_tree;
use str_tree::StrTree;

fn main() {
	let mut _tree = StrTree::init();


	_tree.add_word("arbre");
	_tree.add_word("arb");
	_tree.add_word("ame");

	println!("{}", _tree.is_word("arbre"));
	println!("{}", _tree.is_word("arbr"));
	println!("{}", _tree.is_word("arb"));
	println!("{}", _tree.is_word("pro"));
	println!("{}", _tree.is_word("ame"));



	if let Some(nb_errors) = _tree.fill_with_file("../pyScrabble/scrabbleWords.txt") {
		println!("File read ! {} errors reported in reading lines", nb_errors);
	} else {
		println!("File not found");
	}
}
