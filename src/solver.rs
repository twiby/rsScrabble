use crate::str_tree::Dictionnary;
use crate::board::BoardService;
use crate::constraints::{PotentialWord, PotentialWordConditions, PotentialWordConditionsBuilder};

pub fn find_best_word_at<B, D>(letter_set: &str, x: usize, y: usize, board: &mut B, dict: &D) -> String 
where B: BoardService, D: Dictionnary {
	println!("{}", board.serialize());

	let mut pw = PotentialWord::new();

	board.get_conditions(x, y, &mut pw);

	for word in dict.get_anagrams(
		letter_set, 
		pw.get_constraint_nb_letters(),
		pw.get_constraint_letters(),
		pw.get_constraint_words()) {
		println!("{}, {:?}", word, board.get_score(&word, x, y));
	}
	return "".to_string();
}