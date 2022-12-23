mod board;
use board::Board;

pub use crate::constraints::WordToFill;
pub use crate::constraints::PotentialWordConditionsBuilder;

#[derive(Debug)]
pub enum DeserializingError {
	UnknownSymbol,
	WrongLength
}

pub trait BoardService {
	fn serialize(&self) -> String;
	fn deserialize(message: String) -> Result<Board, DeserializingError>;
	fn get_conditions<T>(&self, x: usize, y: usize, condotions: &mut T)
	where T: PotentialWordConditionsBuilder;
}

pub fn deserialize(message: String) -> Result<Board, DeserializingError> {
	return Board::deserialize(message.clone());
}
