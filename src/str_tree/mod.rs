mod read_file;
pub use read_file::read_lines;


pub struct StrTree {
	data: Option<char>,
	nb_letters: u8,
	is_word: bool,
	children: Vec<StrTree>
}

impl StrTree {
	pub fn init() -> Self {
		return Self{
			data: None, 
			nb_letters: 0, 
			is_word: false, 
			children: Vec::new()};
	}

	//// TO REMOVE
	pub fn check_child(&self, c: char) -> bool {
		for child in &self.children {
			if child.data == Some(c) {
				return true;
			}
		}
		return false;
	}

	fn get_child_idx(&self, c: char) -> Option<usize> {
		for i in 0..self.children.len() {
			if self.children[i].data == Some(c) {
				return Some(i);
			}
		}
		return None;
	}

	#[allow(dead_code)]
	pub fn get_child<'a>(&'a self, c: char) -> Option<&'a StrTree> {
		if let Some(i) = self.get_child_idx(c) {
			return Some(&self.children[i]);
		} else {
			return None;
		}
	}
	#[allow(dead_code)]
	fn get_child_mut<'a>(&'a mut self, c: char) -> Option<&'a mut StrTree> {
		if let Some(i) = self.get_child_idx(c) {
			return Some(&mut self.children[i]);
		} else {
			return None;
		}
	}

	fn get_or_make_child(&mut self, c:char) -> &mut StrTree {
		if let Some(idx) = self.get_child_idx(c) {
			return &mut self.children[idx];
		} else {
			return self.add_child(c);
		}
	}

	fn add_child<'a>(&'a mut self, c: char) -> &'a mut StrTree {
		println!("Creating child {} for parent {:?}", c, self.data);
		let new_tree = StrTree{
			data: Some(c),
			nb_letters: self.nb_letters + 1,
			is_word: false,
			children: Vec::new()
		};
		self.children.push(new_tree);
		return self.children.last_mut().unwrap();
	}

	pub fn add_word(&mut self, word: &str) {
		let mut letter_idx: usize = 0;
		let mut node = self;

		while let Some(c) = word.chars().nth(letter_idx) {
			node = node.get_or_make_child(c);
			letter_idx += 1;
		}
		node.is_word = true;
	}

	// The output is wrapped in a Result to allow matching on errors
	// Returns an Iterator to the Reader of the lines of the file.
	pub fn fill_with_file<P>(&mut self, filename: P) -> Option<u32>
	where P: AsRef<std::path::Path>, {
		if let Ok(reader) = read_lines(filename) {
			println!("File found");

			let mut nb_errors:u32 = 0;
			for line in reader.take(3) {
				if let Ok(word) = line {
					println!("{}", word);
				} else {
					nb_errors += 1;
				}
			}

			return Some(nb_errors);
		} else {
			return None;
		}
	}
}

