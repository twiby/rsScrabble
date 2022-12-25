use crate::board::BoardService;
use crate::board::{DeserializingError, DeserializingError::*};
use crate::board::{WordError, WordError::*};
use crate::board::WordToFill;
use crate::board::PotentialWordConditionsBuilder;

use crate::board::values::*;
use crate::board::tile::*;
use Tile::*;
use PlayedTile::*;
use BoardTile::*;

const SIDE: usize = 15;
const SIZE: usize = SIDE * SIDE;

#[derive(Debug)]
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
					Played(LetterTile(c)) => c,
					Played(JokerTile(c)) => c.to_ascii_uppercase(),
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
						Played(LetterTile(c))
					} else if c.is_ascii_uppercase() {
						Played(JokerTile(c.to_ascii_lowercase()) )
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
				conditions.add_letter(c.to_ascii_lowercase(), relative_y);
				continue;
			}

			// Find letters above and/or below: a word to fill
			match WordToFill::new(
				self.get_above(x, absolute_y).to_ascii_lowercase(), 
				self.get_below(x, absolute_y).to_ascii_lowercase()) {
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

	fn get_score(&self, word: &str, x: usize, y: usize) -> Result<usize, WordError> {
		let mut word_bonus: usize = 1;
		let mut word_value: usize = 0;
		let mut other_words_formed: usize = 0;

		for (c, relative_y) in word.chars().zip(0..word.len()) {
			let absolute_y = y + relative_y;
			let mut local_letter_bonus = 1;
			let mut local_word_bonus = 1;

			word_value += match (c, self.at(x, absolute_y)) {
				// Case of constraint: there must be a letter on the board
				('_', Played(JokerTile(_))) => {0; continue},
				('_', Played(LetterTile(c2))) => {get_value_lowercase(c2); continue},
				('_', _) => return Err(UnexpectedUnderscore),

				// Case of letter: there must be no letter on the board
				(_, Board(EmptyTile)) => get_value(c)?,
				(_, Board(LetterBonusTile(n))) => {
					local_letter_bonus = n as usize;
					local_letter_bonus * get_value(c)?
				},
				(_, Board(WordBonusTile(n))) => {
					local_word_bonus = n as usize;
					word_bonus *= local_word_bonus;
					get_value(c)?
				}

				(_,_) => return Err(TileOccupied)
			};

			// Find letters above and/or below: a word filled
			match WordToFill::new(self.get_above(x, absolute_y), self.get_below(x, absolute_y)) {
				Err(_) => (),
				Ok(word) => {
					other_words_formed += 
						local_word_bonus * get_str_value(&word.complete(c))? + 
						(local_letter_bonus-1) * get_value(c)?;
				}
			};
		}
		return Ok(word_value * word_bonus + other_words_formed);
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

	fn get_above(&self, x: usize, y: usize) -> String {
		let mut above = "".to_string();
		for xx in 1u8..(x as u8) {
			match self.at(x-xx as usize, y).letter() {
				Some(c) => above.push(c),
				None => break
			};
		}

		return above.chars().rev().collect::<String>();
	}

	fn get_below(&self, x: usize, y: usize) -> String {
		let mut below = "".to_string();
		for xx in 1u8..((SIDE-x) as u8) {
			match self.at(x+xx as usize, y).letter() {
				Some(c) => below.push(c),
				None => break
			};
		}
		return below;
	}
}
