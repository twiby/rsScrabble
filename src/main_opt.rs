mod str_tree;
use str_tree::Dictionnary;

fn main() {
	let tree = match str_tree::build_dict_from_file("../pyScrabble/scrabbleWords.txt") {
		Err(e) => {println!("File not read: {e:?}"); return;},
		Ok(dict) => dict
	};
	println!("{0:?}", tree.get_anagrams("catastrophe", Some(vec![2,3]), None));
	println!("{0:?}", tree.get_anagrams("a00", None, None).len());
	println!("{0:?}", tree.get_anagrams("a00", Some(vec![2]), None).len());
	println!("{0:?}", tree.get_anagrams("a00", Some(vec![3]), None).len());
	println!("{0:?}", tree.is_word("njklhjkl"));
	println!("{0:?}", tree.is_word("ud"));
	println!("{0:?}", tree.is_word("woh"));

	println!("{0:?}", tree.get_anagrams("arbe", None, Some(vec![(3, 'r')])));
	println!("{0:?}", tree.get_anagrams("rbr", None, Some(vec![(0, 'a'), (4, 'e')])));
}
