mod board;
use board::Board;

mod values;
mod tile;

pub use crate::constraints::WordToFill;
pub use crate::constraints::PotentialWordConditionsBuilder;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum DeserializingError {
	UnknownSymbol,
	WrongLength
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum WordError {
	TileOccupied,
	UnexpectedUnderscore,
	UnknownChar
} 

pub mod transposition
{
	pub struct Transposed;
	pub struct NotTransposed;
	pub trait TransposedState {
		fn transposed_coord(x: usize, y: usize) -> (usize, usize);
	}
	impl TransposedState for Transposed {
		fn transposed_coord(x: usize, y: usize) -> (usize, usize) { (y,x) }
	}
	impl TransposedState for NotTransposed {
		fn transposed_coord(x: usize, y: usize) -> (usize, usize) { (x,y) }
	}
}

pub trait BoardService {
	fn serialize<T: transposition::TransposedState>(&self) -> String;
	fn deserialize(message: String) -> Result<Board, DeserializingError>;
	fn get_conditions<T: transposition::TransposedState, PWCB>(&self, x: usize, y: usize, conditions: &mut PWCB)
	where PWCB: PotentialWordConditionsBuilder;
	fn get_score<T: transposition::TransposedState>(&self, word: &str, x: usize, y: usize) -> Result<usize, WordError>;
}

pub fn deserialize(message: String) -> Result<Board, DeserializingError> {
	return Board::deserialize(message.clone());
}
