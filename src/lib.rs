#[cfg(test)]
mod test;

mod str_tree;
use str_tree::Dictionnary;

mod constraints;
mod board;

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

#[pyclass]
struct WordFinder {
	_tree: str_tree::StrTree
}

#[pymethods]
impl WordFinder {
	#[new]
	fn new(filename: &str) -> PyResult<Self> {
		match str_tree::build_dict_from_file(filename) {
			Err(e) => return Err(PyErr::new::<PyValueError, _>(e)),
			Ok(tree) => return Ok(WordFinder{_tree: tree})
		};
	}

	fn add_word(&mut self, new_word: &str) {
		self._tree.add_word(new_word);
	}

	fn is_word(&self, word: &str) -> bool {
		return self._tree.is_word(word);
	}

	fn get_anagrams(
		&self, 
		letter_set: &str, 
		nb_letters: Option<Vec<u8>>, 
		constraint_letters: Option<Vec<char>>,
		constraint_indices: Option<Vec<u8>>) 
	-> PyResult<Vec<String>> {
		// Sanitizing internal arguments for constraint letters
		if constraint_letters.is_none() != constraint_indices.is_none() {
			return Err(PyErr::new::<PyValueError, _>("WordFinder: provide both constraint letters and indices, or none"));
		}

		let constraints: Option<Vec<(u8, char)>> = match constraint_letters {
			None => None,
			Some(letters) => {
				if letters.len() != constraint_indices.as_ref().unwrap().len() {
					return Err(PyErr::new::<PyValueError, _>("WordFinder: constraint indices and letters must be the same length"));
				}
				Some(constraint_indices.unwrap().into_iter().zip(letters.into_iter()).collect())
			}
		};

		// Call internal function
		return Ok(self._tree.get_anagrams(letter_set, nb_letters, constraints, None));
	}
}


#[pymodule]
fn rusted_tree(_py: Python, m: &PyModule) -> PyResult<()> {
	m.add_class::<WordFinder>()?;
	return Ok(());
}
