use crate::str_tree::Dictionnary;

use crate::board::BoardService;
use crate::board::WordError;
use crate::board::transposition::*;

use crate::constraints::{PotentialWord, PotentialWordConditions, PotentialWordConditionsBuilder};

use pyo3::prelude::{pyclass, pymethods};

type WordSearchResult = Result<Option<BestWord>, WordError>;

#[derive(Debug)]
#[derive(PartialEq)]
#[pyclass]
pub struct BestWord {
	#[pyo3(get)]
	pub vertical: bool,
	#[pyo3(get)]
	pub coord: (usize, usize),
	#[pyo3(get)]
	pub word: String,
	#[pyo3(get)]
	pub score: usize
}
#[pymethods]
impl BestWord {
	fn __str__(&self) -> pyo3::PyResult<String> {
		let mut ret = "[".to_string();
		ret.push_str(&self.word);
		ret.push(']');
		ret.push_str(" at: (");
		ret.push_str(&self.coord.0.to_string());
		ret.push_str(", ");
		ret.push_str(&self.coord.1.to_string());
		ret.push_str(") ");
		if self.vertical {
			ret.push_str("vertically -> ");
		} else {
			ret.push_str("horizontally -> ");
		}
		ret.push_str(&self.score.to_string());
		return Ok(ret);
	}
}

pub trait TransposedBool {
	fn get_transposition_as_orientation() -> bool;
}
impl TransposedBool for Transposed {
	fn get_transposition_as_orientation() -> bool {true}
}
impl TransposedBool for NotTransposed {
	fn get_transposition_as_orientation() -> bool {false}
}

fn _find_best_word_at<T, B, D>(
	letter_set: &str, 
	x: usize, y: usize, 
	board: &B, 
	dict: &D, 
	pw: &mut PotentialWord) 
-> WordSearchResult
where B: BoardService, D: Dictionnary, T: TransposedState + TransposedBool {
	let mut best_word: String = "".to_string();
	let mut best_score = 0;

	board.get_conditions::<T, _>(x, y, pw);

	for word in dict.get_anagrams(
		letter_set, 
		pw.get_constraint_nb_letters(),
		pw.get_constraint_letters(),
		pw.get_constraint_words()) {

		let score = board.get_score::<T>(&word, x, y)?;
		if score > best_score {
			best_score = score;
			best_word = word.clone();
		}
	}

	match best_score {
		0 => Ok(None),
		_ => Ok(Some(BestWord{
			vertical: T::get_transposition_as_orientation(),
			coord: T::transposed_coord(x, y),
			word: best_word,
			score: best_score
		}))
	}
}

fn find_best_word_at<B, D>(
	letter_set: &str, 
	x: usize, y: usize, 
	board: &B, 
	dict: &D, 
	pw: &mut PotentialWord) 
-> WordSearchResult
where B: BoardService, D: Dictionnary {
	let bw_horizontal = _find_best_word_at::<NotTransposed, _, _>(letter_set, x, y, board, dict, pw)?;
	let bw_vertical = _find_best_word_at::<Transposed, _, _>(letter_set, x, y, board, dict, pw)?;
	
	match (&bw_horizontal, &bw_vertical) {
		(None, None) => Ok(None),
		(None, _) => Ok(bw_vertical),
		(_, None) => Ok(bw_horizontal),
		(Some(ref b1), Some(ref b2)) => {
			if b1.score > b2.score {
				Ok(bw_horizontal)
			} else {
				Ok(bw_vertical)
			}
		}
	}
}

pub fn find_best_word<B, D>(
	letter_set: &str, 
	board: &B, 
	dict: &D) 
-> WordSearchResult
where B: BoardService, D: Dictionnary {
	let mut best_word:Option<BestWord> = None;
	let mut pw = PotentialWord::new();

	for x in 0..crate::board::SIDE {
		for y in 0..crate::board::SIDE {
			if let Some(bw) = find_best_word_at(letter_set, x, y, board, dict, &mut pw)? {
				best_word = match best_word {
					None => Some(bw),
					Some(ref word) => {
						if word.score < bw.score {
							Some(bw)
						} else {
							best_word
						}
					}
				};
			}
		}
	}

	return Ok(best_word)
}