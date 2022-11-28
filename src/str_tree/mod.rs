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
	idx: Vec<i8>
}

impl<'a> AnagramIterator<'a> {
	pub fn new(set: &'a mut Vec<char>) -> AnagramIterator<'a> {
		let s = set.len();
		let mut v = vec![0;s];
		v[s-1] = -1;
		return Self{letter_set: set, size: s, idx: v};
	}
	fn word(&self, n:usize) -> String {
		return self.letter_set[..n].iter().cloned().collect::<String>();
	}

	fn enter_idx(&mut self, n: usize) {
		self.letter_set.swap(self.idx[n] as usize, self.size-n-1);
	}
	fn exit_idx(&mut self, n: usize) {
		self.letter_set.swap(self.idx[n] as usize, self.size-n-1);
	}

	fn enter(&mut self) {
		for n in 0..self.size {
			self.enter_idx(n);
		}
	}
	fn exit(&mut self) {
		for n in (0..self.size).rev() {
			self.exit_idx(n);
		}
	}

	fn increment(&mut self) -> bool {
		let mut n = self.size-1;

		self.idx[n] += 1;
		while self.idx[n] >= (self.size - n) as i8 {
			if n == 0 {
				return false;
			}
			self.idx[n] = 0;
			n -= 1;
			self.idx[n] += 1;
		}

		return true;
	}
}

impl Iterator for AnagramIterator<'_> {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		if !self.increment() {
			return None;
		}
		self.enter();
		let w = self.word(self.size);
		self.exit();
		return Some(w);
	}
}
