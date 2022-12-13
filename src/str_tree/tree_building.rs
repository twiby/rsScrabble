use crate::str_tree::read_file::read_lines;
use crate::str_tree::read_file::cnt_lines;
use crate::str_tree::Dictionnary;

pub struct StrTree {
	data: Option<char>,
	nb_letters: u8,
	is_word: bool,
	children: Vec<StrTree>
}

impl Dictionnary for StrTree {
	fn build_dict_from_file(filename: &str) -> std::io::Result<StrTree> {
		let mut ret = StrTree::init();
		match ret.fill_with_file(filename) {
			Err(e) => return Err(e),
			Ok(_) => return Ok(ret)
		};
	}

	fn get_anagrams(&self, letter_set: &str) -> Vec<String> {
		let mut letter_set_vec:Vec<char> = letter_set.chars().collect();
		letter_set_vec.sort_unstable();
		return self.get_anagrams_internal(letter_set_vec, "".to_string());
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
}

impl StrTree {
	fn init() -> Self {
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

	#[allow(dead_code)]
	fn is_word(&self, word: &str) -> bool {
		match self.get_node(word) {
			None => return false,
			Some(node) => return node.is_word
		};
	}

	#[allow(dead_code)]
	fn get_node<'a: 'b, 'b>(&'a self, word: &str) -> Option<&'b StrTree> {
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
	fn fill_with_file(&mut self, filename: &str) -> std::io::Result<u32> {
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

	fn get_anagrams_internal(&self, letter_set: Vec<char>, current_word: String) -> Vec<String> {
		let mut ret = Vec::<String>::new();

		let mut new_current_word = current_word.clone();

		// Handle case where there's at least one joker in set
		if letter_set.first() == Some(&'0') {
			for child in &self.children {
				new_current_word.push(child.data.unwrap());

				if child.is_word { ret.push(new_current_word.clone()); }
				ret.extend(child.get_anagrams_internal(letter_set[1..].to_vec(), new_current_word.clone()));

				new_current_word.pop();
			}
		}

		// Now take every letter in the set, and see if you can build a word from it
		for i in 0..letter_set.len() {
			// This avoids repetition coming from identitical letters
			if i > 0 && letter_set[i-1]==letter_set[i] {
				continue;
			}

			let node = match self.get_child(letter_set[i]) {
				None => continue,
				Some(node) => node
			};

			new_current_word.push(letter_set[i]);

			if node.is_word { ret.push(new_current_word.clone()); }

			ret.extend(
				node.get_anagrams_internal(
					[letter_set[0..i].to_vec(), letter_set[i+1..].to_vec()].concat(), 
					new_current_word.clone()));

			new_current_word.pop();
		}

		return ret;
	}
}