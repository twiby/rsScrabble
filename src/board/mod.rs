const SIDE: usize = 15;
const SIZE: usize = SIDE * SIDE;

#[derive(Debug)]
pub enum DeserializingError {
	UnknownSymbol,
	WrongLength
}
use DeserializingError::*;

#[derive(Copy)]
#[derive(Clone)]
enum Tile{
	EmptyTile,
	LetterTile(char),
	LetterBonusTile(u8),
	WordBonusTile(u8)
}

pub trait BoardService {
	fn serialize(&self) -> String;
	fn deserialize(message: String) -> Result<Board, DeserializingError>;
}

pub struct Board {
	tiles: [Tile; SIZE]
}

impl BoardService for Board {
	fn serialize(&self) -> String {
		let mut message = "".to_string();
		for x in 0..SIDE {
			for y in 0..SIDE {
				message.push( match self.at(x, y) {
					Tile::EmptyTile => '_',
					Tile::LetterTile(c) => c,
					Tile::WordBonusTile(n) => (n+3).to_string().chars().nth(0).unwrap(),
					Tile::LetterBonusTile(n) => n.to_string().chars().nth(0).unwrap()
				});
				message.push(' ');
			}
			message.push('\n');
		}
		return message;
	}

	fn deserialize(message: String) -> Result<Board, DeserializingError> {
		let mut board = Board::new_empty();

		let mut tile_nb:usize = 0;
		for char in message.chars() {
			board.tiles[tile_nb] = match char {
				'_' => Tile::EmptyTile,
				'2' => Tile::LetterBonusTile(2),
				'3' => Tile::LetterBonusTile(3),
				'5' => Tile::WordBonusTile(2),
				'6' => Tile::WordBonusTile(3),
				c => {
					if !c.is_ascii_lowercase() { return Err(UnknownSymbol); }
					Tile::LetterTile(c)
				}
			};
			tile_nb += 1;
		}
		if tile_nb != SIZE {
			return Err(WrongLength);
		}

		return Ok(board);
	}
}

impl Board {
	fn new_empty() -> Board {
		return Board{tiles: [Tile::EmptyTile; SIZE]};
	}

	#[allow(dead_code)]
	fn at_nopanic(&self, x: usize, y: usize) -> Option<Tile> {
		if x >= SIDE || y >= SIDE {
			return None;
		}
		return Some(self.at(x, y));
	}
	fn at(&self, x: usize, y:usize) -> Tile {
		return self.tiles[x*SIDE + y];
	}
}

