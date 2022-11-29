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

	pub fn get_anagrams(&self, mut letter_set: Vec<char>) -> Vec<String> {
		let mut ret = Vec::<String>::new();

		let it = AnagramIterator::new(&mut letter_set);
		for w in it {
			ret.push(w);
		}

		return ret;
	}
}

struct AnagramIterator<'a> {
	letter_set: &'a mut Vec<char>,
	size: usize,
	idx: Vec<usize>,
	last_updated_idx: usize,
	word: String
}

impl<'a> AnagramIterator<'a> {
	pub fn new(set: &'a mut Vec<char>) -> AnagramIterator<'a> {
		let s = set.len();
		let v = vec![0;s];
		let c:char = *set.last().unwrap();
		return Self{letter_set: set, size: s, idx: v, last_updated_idx: 0, word: c.to_string()};
	}

	fn enter_idx(&mut self, n: usize) {
		self.word.push(self.letter_set[self.idx[n]]);
		self.letter_set.swap(self.idx[n], self.size-n-1);
	}
	fn exit_idx(&mut self, n: usize) {
		self.word.pop();
		self.letter_set.swap(self.idx[n], self.size-n-1);
	}

	#[allow(dead_code)]
	fn enter(&mut self) {
		for n in 0..self.size {
			self.enter_idx(n);
		}
	}
	#[allow(dead_code)]
	fn exit(&mut self) {
		for n in (0..self.size).rev() {
			self.exit_idx(n);
		}
	}

	fn increment_idx(&mut self, n: usize) -> bool {
		for i in (n+1)..self.size {
			assert!(self.idx[i] == 0);
		}

		self.exit_idx(n);
		self.idx[n] += 1;
		self.last_updated_idx = n;
		if self.idx[n] < self.size - n {
			self.enter_idx(n);
			return true;
		} else {
			return false;
		}
	}

	fn increment(&mut self) {
		if self.last_updated_idx < self.size - 1 {
			self.last_updated_idx = 1 + self.last_updated_idx;
			self.enter_idx(self.last_updated_idx);
			return;
		}

		let mut n = self.size-1;

		while !self.increment_idx(n) && n > 0 {
			self.idx[n] = 0;
			n -= 1;
		}
	}
}

impl Iterator for AnagramIterator<'_> {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		if self.idx[0] == self.size {
			return None;
		}

		let w = self.word.clone();
		self.increment();
		return Some(w);
	}
}
