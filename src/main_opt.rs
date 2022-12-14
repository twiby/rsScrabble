mod str_tree;
use str_tree::Dictionnary;

fn main() {
	let tree = match str_tree::build_dict_from_file("../pyScrabble/scrabbleWords.txt") {
		Err(e) => {println!("File not read: {e:?}"); return;},
		Ok(dict) => dict
	};
	println!("{0:?}", tree.get_anagrams("catastrophe", Some(vec![2,3])));
	println!("{0:?}", tree.get_anagrams("a00", None).len());
	println!("{0:?}", tree.get_anagrams("a00", Some(vec![2])).len());
	println!("{0:?}", tree.get_anagrams("a00", Some(vec![3])).len());
}
