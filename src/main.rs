mod str_tree;
use str_tree::Dictionnary;

fn main() {
	match str_tree::build_dict_from_file("../pyScrabble/scrabbleWords.txt") {
		Err(e) => {println!("File not read: {e:?}"); return;},
		Ok(dict) => println!("{0:?}", dict.get_anagrams("a00"))
	};
}
