mod str_tree;
use str_tree::StrTree;

fn main() {
	let mut _tree = StrTree::init();

	match _tree.fill_with_file("../pyScrabble/scrabbleWords.txt") {
		Ok(nb_words) => println!("File read ! {} words found", nb_words),
		Err(e) => println!("File not read: {e:?}")
	};

	println!("{}", _tree.is_word("prout"));
	println!("{}", _tree.is_word("wu"));
	println!("{}", _tree.is_word("hjklhjkl"));
	println!("{}", _tree.is_word("mmm"));
	println!("{}", _tree.is_word("acquitterait"));


        for w in _tree.get_anagrams(String::from("abc").chars().collect()) {
                println!("{w:?}");
        }

}
