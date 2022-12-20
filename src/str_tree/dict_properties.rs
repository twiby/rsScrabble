pub trait Dictionnary<T> {
	fn build_dict_from_file(filename: &str) -> std::io::Result<T>;
	fn get_anagrams(&self, letter_set: &str, nb_letter: Option<Vec<u8>>, letter_constraints: Option<Vec<(u8, char)>>) -> Vec<String>;
	fn add_word(&mut self, new_word: &str);
	fn is_word(&self, word: &str) -> bool;
}

pub type ConstraintNbLetters = Option<Vec<u8>>;
pub type ConstraintLetters = Option<Vec<(u8, char)>>;

pub trait ConstraintNbLettersTrait {
	fn sort_and_fuse(&mut self);
	fn decrease(&mut self) -> bool;
	fn valid(&self) -> bool;
}
pub trait ConstraintLettersTrait {
	fn sort_and_fuse(&mut self);
	fn decrease(&mut self) -> Option<char>;
}

impl ConstraintNbLettersTrait for ConstraintNbLetters {
	fn sort_and_fuse(&mut self) {
		match self {
			None => (),
			Some(ref mut vec) => {
				vec.sort_unstable();
				vec.reverse();
				vec.dedup();
			}
		};
	}

	fn decrease(&mut self) -> bool {
		match self {
			None => true,
			Some(ref mut vec) => {
				if vec.last() == Some(&0) {
					vec.pop();
				}
				for el in vec.into_iter() {
					*el -= 1;
				}
				!vec.is_empty()
			}
		}
	}

	fn valid(&self) -> bool {
		match self {
			None => true,
			Some(vec) => vec.last() == Some(&0)
		}
	}
}

impl ConstraintLettersTrait for ConstraintLetters {
	fn sort_and_fuse(&mut self) {
		match self {
			None => (),
			Some(ref mut vec) => {
				vec.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
				vec.reverse();
				vec.dedup_by(|a, b| a.0.eq(&b.0));
			}
		}
	}

	fn decrease(&mut self) -> Option<char> {
		match self {
			None => None,
			Some(ref mut vec) => {
				if vec.is_empty() {
					return None;
				}
				let (i,c) = *vec.last().unwrap();
				let ret = match i {
					0 => {vec.pop(); Some(c)},
					_ => None
				};

				for (idx, _) in vec.into_iter() {
					*idx -= 1;
				}
				return ret;
			}
		}
	}
}
