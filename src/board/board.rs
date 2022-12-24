use crate::board::BoardService;
use crate::board::{DeserializingError, DeserializingError::*};
use crate::board::WordToFill;
use crate::board::PotentialWordConditionsBuilder;

const SIDE: usize = 15;
const SIZE: usize = SIDE * SIDE;

#[derive(Copy)]
#[derive(Clone)]
pub enum PlayTile {
	LetterTile(char),
	JokerTile(char)
}
use PlayTile::*;
#[derive(Copy)]
#[derive(Clone)]
pub enum BoardTile {
	EmptyTile,
	LetterBonusTile(u8),
	WordBonusTile(u8)
}
use BoardTile::*;

#[derive(Copy)]
#[derive(Clone)]
pub enum Tile{
	Play(PlayTile),
	Board(BoardTile)
}
use Tile::*;
impl Tile {
	fn is_occupied(&self) -> bool {
		match self.letter() {
			None => false,
			Some(_) => true
		}
	}

	fn letter(&self) -> Option<char> {
		match self {
			Play(LetterTile(c)) | Play(JokerTile(c)) => Some(*c),
			_ => None
		}
	}
}

pub struct Board {
	tiles: [Tile; SIZE],
	transposed: bool
}

impl BoardService for Board {
	fn serialize(&self) -> String {
		let mut message = "".to_string();
		for x in 0..SIDE {
			for y in 0..SIDE {
				message.push( match self.at(x, y) {
					Board(EmptyTile) => '_',
					Play(LetterTile(c)) => c,
					Play(JokerTile(c)) => c.to_ascii_uppercase(),
					Board(WordBonusTile(n)) => (n+3).to_string().chars().nth(0).unwrap(),
					Board(LetterBonusTile(n)) => n.to_string().chars().nth(0).unwrap()
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
				'_' => Board(EmptyTile),
				'2' => Board(LetterBonusTile(2)),
				'3' => Board(LetterBonusTile(3)),
				'5' => Board(WordBonusTile(2)),
				'6' => Board(WordBonusTile(3)),
				c => {
					if c.is_ascii_lowercase() {
						Play(LetterTile(c))
					} else if c.is_ascii_uppercase() {
						Play(JokerTile(c.to_ascii_lowercase()) )
					} else { 
						return Err(UnknownSymbol); 
					}
				}
			};
			tile_nb += 1;
		}
		if tile_nb != SIZE {
			return Err(WrongLength);
		}

		return Ok(board);
	}

	fn get_conditions<T>(&self, x: usize, y: usize, conditions: &mut T)
	where T: PotentialWordConditionsBuilder {
		conditions.reset();

		if y > 0 && self.at(x, y-1).is_occupied() { return; }

		let mut nb_letters = 0;
		let mut at_least_one_constraints = false;

		for relative_y in 0u8..((SIDE-y) as u8) {
			let absolute_y = y + relative_y as usize;

			// Case: tile is occupied: register letter and continue
			if let Some(c) = self.at(x, absolute_y).letter() {
				// Special case: if first constraint, previous nb_letter is acceptable
				if !at_least_one_constraints {
					conditions.add_nb_letters(nb_letters);
				}
				at_least_one_constraints = true;
				conditions.add_letter(c, relative_y);
				continue;
			}

			// Find letters above and/or below: a word to fill
			let mut above = "".to_string();
			for xx in 1u8..(x as u8) {
				match self.at(x-xx as usize, absolute_y).letter() {
					Some(c) => above.push(c),
					None => break
				};
			}
			let mut below = "".to_string();
			for xx in 1u8..((SIDE-x) as u8) {
				match self.at(x+xx as usize, absolute_y).letter() {
					Some(c) => below.push(c),
					None => break
				};
			}
			match WordToFill::new(above.chars().rev().collect::<String>(), below) {
				Err(_) => (),
				Ok(word) => {
					at_least_one_constraints = true;
					conditions.add_word(word, relative_y)
				}
			};

			// add this possible number of letter if any constraint has already been met 
			nb_letters += 1;
			if at_least_one_constraints {
				conditions.add_nb_letters(nb_letters);
			}
		}
	}
}

impl Board {
	fn new_empty() -> Board {
		return Board{tiles: [Board(EmptyTile); SIZE], transposed: false};
	}

	// Accessors
	fn at(&self, x: usize, y:usize) -> Tile {
		let t = self.transposed as usize;
		let x_transposed = x*(1-t) + y*t;
		let y_transposed = x*t + y*(1-t);
		return self.tiles[x_transposed*SIDE + y_transposed];
	}
	#[allow(dead_code)]
	fn at_nopanic(&self, x: usize, y: usize) -> Option<Tile> {
		if x >= SIDE || y >= SIDE {
			return None;
		}
		return Some(self.at(x, y));
	}

	// fn transpose(&mut self) {
	// 	self.transposed = !self.transposed;
	// }
}
