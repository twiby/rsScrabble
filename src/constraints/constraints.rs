use crate::constraints::{ConstraintNbLetters, ConstraintLetters, ConstraintWords};

#[derive(Clone)]
pub struct WordToFill {
	beginning: String,
	end: String
}
#[derive(Debug)]
pub struct NoWordToFillError;
impl WordToFill {
	pub fn new(begin: String, end: String) -> Result<Self, NoWordToFillError> {
		if begin == "" && end == "" {
			return Err(NoWordToFillError);
		}
		return Ok(Self{beginning: begin.clone(), end: end.clone()});
	}

	fn complete(&self, c: char) -> String {
		let mut ret = self.beginning.clone();
		ret.push(c);
		ret.push_str(&self.end);
		return ret;
	}
}

impl ConstraintWords for Option<Vec<(u8, WordToFill)>> {
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

	fn decrease(&mut self, c: char) -> Option<String> {
		match self {
			None => None,
			Some(ref mut vec) => {
				if vec.is_empty() {
					return None;
				}
				let (i,w) = (*vec.last().unwrap()).clone();
				let ret = match i {
					0 => {vec.pop(); Some(w.complete(c))},
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

impl ConstraintNbLetters for Option<Vec<u8>> {
	fn sort_and_fuse(&mut self) {
		match self {
			None => (),
			Some(ref mut vec) => {
				vec.sort_unstable();
				vec.reverse();
				vec.dedup();
				if vec.last() == Some(&0) {
					vec.pop();
				}
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

impl ConstraintLetters for Option<Vec<(u8, char)>> {
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
