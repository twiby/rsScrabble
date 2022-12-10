mod str_tree;
use str_tree::StrTree;

fn main() {
	let mut _tree = StrTree::init();

	match _tree.fill_with_file("../pyScrabble/scrabbleWords.txt") {
		Ok(nb_words) => println!("File read ! {} words found", nb_words),
		Err(e) => println!("File not read: {e:?}")
	};

	let words = _tree.get_anagrams(String::from("a00"));
	println!("{0:?}", words);
	for w in words {
		assert!(_tree.is_word(&w));
	}
	}
