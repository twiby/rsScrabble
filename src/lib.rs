#![allow(non_snake_case)]

#[cfg(test)]
mod test;

mod str_tree;
use str_tree::Dictionnary;

mod constraints;

mod board;
use board::DeserializingError;
use board::DeserializingError::*;
use board::WordError;
use board::WordError::*;

mod solver;
use solver::BestWord;
use solver::WithoutTimer;

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

fn py_value_error(msg: &str) -> pyo3::PyErr {
	PyErr::new::<PyValueError, _>(msg.to_string())
}

// TODO: these error messages should be with the classe declaration and flow more naturally in PyErr
impl From<WordError> for pyo3::PyErr {
	fn from(e: WordError) -> Self {
		match e {
			TileOccupied => py_value_error("Solver: tile occupied"),
			UnknownChar => py_value_error("Solver: unknown char"),
			UnexpectedUnderscore => py_value_error("Solver: unexpected underscore")
		}
	}
}

impl From<DeserializingError> for pyo3::PyErr {
	fn from(e: DeserializingError) -> Self {
		match e {
			WrongLength => py_value_error("Deserialization: board message has wrong length"),
			UnknownSymbol => py_value_error("Deserialization: board message has non valid char")
		}
	}
}

#[pyclass]
struct WordFinder {
	_tree: str_tree::StrTree,
	_word_buffer: Vec<str_tree::StaticWord>
}

#[pymethods]
impl WordFinder {
	#[new]
	fn new(filename: &str) -> PyResult<Self> {
		match str_tree::build_dict_from_file(filename) {
			Err(e) => Err(PyErr::new::<PyValueError, _>(e)),
			Ok(tree) => Ok(WordFinder{
				_tree: tree, 
				_word_buffer: str_tree::initiate_word_buf(1000)})
		}
	}

	fn add_word(&mut self, new_word: &str) {
		self._tree.add_word(new_word);
	}

	fn is_word(&self, word: &str) -> bool {
		return self._tree.is_word(word);
	}

	fn get_best_play(&mut self, word: &str, board_msg: &str) -> PyResult<Option<BestWord>> {
		let board = board::deserialize(board_msg)?;
		let bw = solver::find_best_word::<WithoutTimer, _, _>(
			word, &board, &self._tree, Some(&mut self._word_buffer))?;
		return Ok(bw);
	}
}


#[pymodule]
fn rsScrabble(_py: Python, m: &PyModule) -> PyResult<()> {
	m.add_class::<WordFinder>()?;
	m.add_class::<BestWord>()?;
	return Ok(());
}
