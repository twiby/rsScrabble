mod read_file;
use read_file::read_lines;
use read_file::cnt_lines;


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
		let i = self.get_child_idx(c)?;
		return Some(&self.children[i]);
	}
	#[allow(dead_code)]
	fn get_child_mut<'a>(&'a mut self, c: char) -> Option<&'a mut StrTree> {
		let i = self.get_child_idx(c)?;
		return Some(&mut self.children[i]);
	}

	fn get_or_make_child(&mut self, c:char) -> &mut StrTree {
		match self.get_child_idx(c) {
			Some(idx) => return &mut self.children[idx],
			None => return self.add_child(c)
		};
	}

	fn add_child<'a>(&'a mut self, c: char) -> &'a mut StrTree {
		let new_tree = StrTree{
			data: Some(c),
			nb_letters: self.nb_letters + 1,
			is_word: false,
			children: Vec::new()
		};
		self.children.push(new_tree);
		return self.children.last_mut().unwrap();
	}

	fn add_word(&mut self, word: &str) {
		let mut letter_idx: usize = 0;
		let mut node = self;

		while let Some(c) = word.chars().nth(letter_idx) {
			node = node.get_or_make_child(c);
			letter_idx += 1;
		}
		node.is_word = true;
	}

	pub fn is_word(&self, word: &str) -> bool {
		let mut letter_idx: usize = 0;
		let mut node = self;
		while let Some(c) = word.chars().nth(letter_idx) {
			match node.get_child(c) {
				None => return false,
				Some(child) => node = child
			};
			letter_idx += 1;
		}
		return node.is_word;
	}

	// The output is wrapped in a Result to allow matching on errors
	pub fn fill_with_file<P>(&mut self, filename: P) -> std::io::Result<u32>
	where P: AsRef<std::path::Path>, {
		let nb_lines = cnt_lines(&filename)?;
		println!("reading {} words from file", nb_lines);


		let reader = read_lines(&filename)?;
		let mut nb_words:u32 = 0;
		for line in reader {
			if let Ok(word) = line {
				self.add_word(&word);
				nb_words += 1;
			}
		}

		return Ok(nb_words);
		
	}

	pub fn get_anagrams(&self, letter_set: Vec<char>) -> Vec<String> {
		let mut ret = Vec::<String>::new();

		let it = AnagramIterator::new(letter_set.clone());
		for w in it {
			ret.push(w);
		}

		return ret;
	}
}

struct AnagramIteratorNode {
	size: usize,
	idx : usize
}
impl AnagramIteratorNode {
	fn new(size: usize) -> Self {
		return Self{
			size: size,
			idx: size
		};
	}
	fn reset(&mut self) {
		self.idx = self.size;
	}
}
impl Iterator for AnagramIteratorNode {
	type Item = usize;
	fn next(&mut self) -> Option<usize> {
		match self.idx {
			0 => return None,
			_ => {self.idx -= 1; return Some(self.size - self.idx - 1);}
		};
	}
}

struct WordManager {
	letter_set: Vec<char>,
	indexes: Vec<usize>,
	word: String
}
impl WordManager {
	fn new(letter_set: Vec<char>) -> Self {
		return Self{
			letter_set: letter_set,
			indexes: Vec::new(),
			word: "".to_string()
		};
	}
	fn push(&mut self, idx: usize) {
		let size:usize = self.indexes.len();
		self.letter_set.swap(size, size+idx);
		self.word.push(self.letter_set[size]);
		self.indexes.push(idx);
	}
	fn pop(&mut self) -> Option<usize> {
		let size:usize = self.indexes.len() - 1;
		self.letter_set.swap(size, size+self.indexes.last().unwrap());
		self.word.pop();
		return self.indexes.pop();
	}
}

struct AnagramIterator {
	nodes: Vec<AnagramIteratorNode>,
	size: usize,
	word: WordManager,
	active_level: usize,
	end: bool
}
impl AnagramIterator {
	fn new(letter_set: Vec<char>) -> Self {
		let size = letter_set.len();
		let mut ret = Self{
			nodes: Vec::new(),
			size: size,
			word: WordManager::new(letter_set),
			active_level: 0,
			end: false
		};
		for i in 0..size {
			ret.nodes.push(AnagramIteratorNode::new(size - i));
		}
		ret.word.push(ret.nodes[0].next().unwrap());
		return ret;
	}
	fn validate(&self, _level: usize, _idx: usize) -> bool {
		// println!("{}, {}", _level, self.word.indexes.len());
		assert!(_level == self.word.indexes.len());
		return true;
	}
	fn next_idx_level(&mut self, level: usize) -> Option<usize> {
		loop {
			let next_idx:usize;
			match self.nodes[level].next() {
				None => return None,
				Some(i) => next_idx = i
			};
			if self.validate(level, next_idx) {
				return Some(next_idx);
			}
		}
	}
	fn try_go_down(&mut self) -> bool {
		if self.active_level == self.size-1 {
			return false;
		}
		self.nodes[self.active_level+1].reset();
		match self.next_idx_level(self.active_level+1) {
			None => return false,
			Some(idx) => {
				self.word.push(idx); 
				self.active_level += 1; 
				return true;
			}
		}
	}
	fn try_go_side(&mut self) -> bool {
		self.word.pop();
		match self.next_idx_level(self.active_level) {
			None => return false,
			Some(idx) => {
				self.word.push(idx);
				return true;
			}
		}
	}
	fn next(&mut self) {
		if self.try_go_down() {
			return;
		}

		loop {
			if self.try_go_side() {
				return;
			} else if self.active_level == 0 {
				self.end = true;
				return;
			}
			self.active_level -= 1;

		}
	}
}
impl Iterator for AnagramIterator {
	type Item = String;
	fn next(&mut self) -> Option<String> {
		if self.end {
			return None;
		}
		let w = self.word.word.clone();
		self.next();
		return Some(w);
	}
}
