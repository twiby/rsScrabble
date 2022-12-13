mod str_tree;
use str_tree::Dictionnary;

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

#[pyclass]
struct TreeInterface {
	_tree: str_tree::StrTree
}

#[pymethods]
impl TreeInterface {
	#[new]
	fn new(filename: &str) -> PyResult<Self> {
		match str_tree::build_dict_from_file(filename) {
			Err(e) => return Err(PyErr::new::<PyValueError, _>(e)),
			Ok(tree) => return Ok(TreeInterface{_tree: tree})
		};
	}

	fn add_word(&mut self, new_word: &str) {
		self._tree.add_word(new_word);
	}

	fn get_anagrams(&self, letter_set: &str) -> Vec<String> {
		return self._tree.get_anagrams(letter_set);
	}
}


#[pymodule]
fn rusted_tree(_py: Python, m: &PyModule) -> PyResult<()> {
	m.add_class::<TreeInterface>()?;
	return Ok(());
}
