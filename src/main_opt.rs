mod str_tree;
use str_tree::Dictionnary;

mod board;
use board::{Board, BoardService};

fn main() {
	match str_tree::build_dict_from_file("../pyScrabble/scrabbleWords.txt") {
		Err(e) => {println!("File not read: {e:?}");},
		Ok(tree) => {
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
	};

	let mut str_board = "".to_string();
	// str_board.push_str("6__2___6___2__6");
	// str_board.push_str("_5___3___3___5_");
	// str_board.push_str("__5___2_2___5__");
	// str_board.push_str("2__5___2___5__2");
	// str_board.push_str("____5_____5____");
	// str_board.push_str("_3___3___3___3_");
	// str_board.push_str("__2___2_2___2__");
	// str_board.push_str("6__2___5___2__6");
	// str_board.push_str("__2___2_2___2__");
	// str_board.push_str("_3___3___3___3_");
	// str_board.push_str("____5_____5____");
	// str_board.push_str("2__5___2___5__2");
	// str_board.push_str("__5___2_2___5__");
	// str_board.push_str("_5___3___3___5_");
	// str_board.push_str("6__2___6___2__6");
	str_board.push_str("6__2___6___2__6");
	str_board.push_str("_5___3___3___5_");
	str_board.push_str("__5___2_2___5__");
	str_board.push_str("2__5___2___5__2");
	str_board.push_str("____5_____5____");
	str_board.push_str("_3___3___3___3_");
	str_board.push_str("__2___2_2___2__");
	str_board.push_str("6__2___a___2__6");
	str_board.push_str("__2___2r2___2__");
	str_board.push_str("_3___3_b_3___3_");
	str_board.push_str("____5__r__5____");
	str_board.push_str("2__5___e___5__2");
	str_board.push_str("__5___2_2___5__");
	str_board.push_str("_5___3___3___5_");
	str_board.push_str("6__2___6___2__6");

	let board = Board::deserialize(str_board).expect("Error when deserializing board message");
	board.print();
}
