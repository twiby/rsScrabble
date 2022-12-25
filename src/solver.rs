use crate::str_tree::Dictionnary;

use crate::board::BoardService;
use crate::board::transposition::*;

use crate::constraints::{PotentialWord, PotentialWordConditions, PotentialWordConditionsBuilder};

pub fn find_best_word_at<T: TransposedState, B, D>(letter_set: &str, x: usize, y: usize, board: &B, dict: &D) -> String 
where B: BoardService, D: Dictionnary {
	println!("{}", board.serialize::<T>());

	let mut pw = PotentialWord::new();

	board.get_conditions::<T, _>(x, y, &mut pw);

	for word in dict.get_anagrams(
		letter_set, 
		pw.get_constraint_nb_letters(),
		pw.get_constraint_letters(),
		pw.get_constraint_words()) {
		println!("{}, {:?}", word, board.get_score::<T>(&word, x, y));
	}
	return "".to_string();
}