const SIDE: usize = 15;
const SIZE: usize = SIDE * SIDE;

#[derive(Copy)]
#[derive(Clone)]
enum Tile{
	EmptyTile,
	LetterTile(char),
	LetterBonusTile(u8),
	WordBonusTile(u8)
}

pub trait BoardService {
	fn deserialize(message: String) -> Option<Board>;
}

pub struct Board {
	tiles: [Tile; SIZE]
}

impl BoardService for Board {
	fn deserialize(message: String) -> Option<Board> {
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
					if !c.is_ascii_lowercase() { return None; }
					Tile::LetterTile(c)
				}
			};
			tile_nb += 1;
		}
		if tile_nb != SIZE {
			return None;
		}

		return Some(board);
	}
}

impl Board {
	fn new_empty() -> Board {
		return Board{tiles: [Tile::EmptyTile; SIZE]};
	}

	pub fn print(&self) {
		for x in 0..SIDE {
			let mut line = "".to_string();
			for y in 0..SIDE {
				line.push( match self.tiles[x*SIDE + y] {
					Tile::EmptyTile => '_',
					Tile::LetterTile(c) => c,
					Tile::WordBonusTile(n) => (n+3).to_string().chars().nth(0).unwrap(),
					Tile::LetterBonusTile(n) => n.to_string().chars().nth(0).unwrap()
				});
				line.push(' ');
			}
			println!("{0:?}", line);
		}
	}
}

