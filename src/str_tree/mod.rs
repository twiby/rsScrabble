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

	fn get_child<'a>(&'a self, c: char) -> Option<&'a StrTree> {
		let i = self.get_child_idx(c)?;
		return Some(&self.children[i]);
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

	#[allow(dead_code)]
	pub fn is_word(&self, word: &str) -> bool {
		match self.get_node(word) {
			None => return false,
			Some(node) => return node.is_word
		};
	}

	#[allow(dead_code)]
	pub fn get_node<'a: 'b, 'b>(&'a self, word: &str) -> Option<&'b StrTree> {
		let mut letter_idx: usize = 0;
		let mut node = self;
		while let Some(c) = word.chars().nth(letter_idx) {
			match node.get_child(c) {
				None => return None,
				Some(child) => node = child
			};
			letter_idx += 1;
		}
		return Some(node);
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

	pub fn get_anagrams(&self, letter_set: String) -> Vec<String> {
		let mut letter_set_vec:Vec<char> = letter_set.chars().collect();
		letter_set_vec.sort_unstable();
		return self.get_anagrams_internal(letter_set_vec, "".to_string());
	}

	fn get_anagrams_internal(&self, letter_set: Vec<char>, current_word: String) -> Vec<String> {
		let mut ret = Vec::<String>::new();

		for i in 0..letter_set.len() {
			if i > 0 && letter_set[i-1]==letter_set[i] {
				continue;
			}

			let c = letter_set[i];

			let node:&StrTree;
			match self.get_child(c) {
				None => continue,
				Some(child) => node = child
			};

			if node.is_word {
				let mut word = current_word.clone();
				word.push(c);
				ret.push(word);
			}

			let mut subset = Vec::<char>::new();
			for ii in 0..letter_set.len() {
				if ii != i {
					subset.push(letter_set[ii]);
				}
			}
			let mut new_current_word = current_word.clone();
			new_current_word.push(c);
			ret.extend(node.get_anagrams_internal(subset, new_current_word.clone()));
		}

		return ret;
	}
}
