mod board;
use board::Board;

pub use crate::constraints::WordToFill;
pub use crate::constraints::PotentialWordConditionsBuilder;

#[derive(Debug)]
pub enum DeserializingError {
	UnknownSymbol,
	WrongLength
}

#[derive(Debug)]
pub enum WordError {
	TileOccupied,
	UnexpectedUnderscore,
	NonAsciiChar
} 

pub trait BoardService {
	fn serialize(&self) -> String;
	fn deserialize(message: String) -> Result<Board, DeserializingError>;
	fn get_conditions<T>(&self, x: usize, y: usize, conditions: &mut T)
	where T: PotentialWordConditionsBuilder;
	fn get_score(&self, word: &str, x: usize, y: usize) -> Result<usize, WordError>;
}

pub fn deserialize(message: String) -> Result<Board, DeserializingError> {
	return Board::deserialize(message.clone());
}
