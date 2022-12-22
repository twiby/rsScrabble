mod board;
use board::Board;

#[derive(Debug)]
pub enum DeserializingError {
	UnknownSymbol,
	WrongLength
}

pub trait TileInfo {
	fn is_occupied(&self) -> bool;
}

pub trait BoardService {
	fn serialize(&self) -> String;
	fn deserialize(message: String) -> Result<Board, DeserializingError>;
}

pub fn deserialize(message: String) -> Result<Board, DeserializingError> {
	return Board::deserialize(message.clone());
}
