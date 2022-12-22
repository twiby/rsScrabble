mod board;
use board::Board;

use crate::str_tree::{ConstraintNbLetters, ConstraintLetters};

#[derive(Debug)]
pub enum DeserializingError {
	UnknownSymbol,
	WrongLength
}

pub trait BoardService {
	fn serialize(&self) -> String;
	fn deserialize(message: String) -> Result<Board, DeserializingError>;
}

pub trait PotentialWord {
	fn get_constraint_nb_letters<CNbL>(&self) -> CNbL where CNbL: ConstraintNbLetters;
	fn get_constraint_letters<CL>(&self) -> CL where CL: ConstraintLetters;
}

pub fn deserialize(message: String) -> Result<Board, DeserializingError> {
	return Board::deserialize(message.clone());
}
