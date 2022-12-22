use crate::str_tree::{read_lines, cnt_lines};
use crate::str_tree::{Dictionnary, ConstraintNbLetters, ConstraintLetters};

pub struct StrTree {
	data: Option<char>,
	nb_letters: u8,
	is_word: bool,
	children: Vec<StrTree>
}

impl Dictionnary<StrTree> for StrTree {
	fn build_dict_from_file(filename: &str) -> std::io::Result<StrTree> {
		let mut ret = StrTree::init();
		match ret.fill_with_file(filename) {
			Err(e) => return Err(e),
			Ok(_) => return Ok(ret)
		};
	}

	fn get_anagrams<CNbL, CL>(
		&self, 
		letter_set: &str, 
		mut nb_letters: CNbL,
		mut letter_constraints: CL) 
	-> Vec<String> 
	where CNbL: ConstraintNbLetters, CL: ConstraintLetters {
		let mut letter_set_vec:Vec<char> = letter_set.chars().collect();
		letter_set_vec.sort_unstable();
		nb_letters.sort_and_fuse();
		letter_constraints.sort_and_fuse();
		return self.get_anagrams_internal(letter_set_vec, "".to_string(), nb_letters, letter_constraints);
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

	fn is_word(&self, word: &str) -> bool {
		match self.get_node(word) {
			None => return false,
			Some(node) => return node.is_word
		};
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

	fn get_anagrams_internal<CNbL, CL>(
		&self, 
		letter_set: Vec<char>, 
		current_word: String, 
		mut nb_letters: CNbL, 
		mut letter_constraints: CL)
	-> Vec<String> 
	where CNbL: ConstraintNbLetters, CL: ConstraintLetters {
		let mut new_current_word = current_word.clone();

		// Case the next letter is a constraint: continue only on that branch if it exists
		if let Some(constraint) = letter_constraints.decrease() {
			let node = match self.get_child(constraint) {
				None => return Vec::<String>::new(),
				Some(node) => node 
			};
			new_current_word.push(constraint);
			return node.get_anagrams_internal(
				letter_set.clone(),
				new_current_word.clone(),
				nb_letters.clone(),
				letter_constraints.clone());
		}

		let mut ret = Vec::<String>::new();
		if self.is_word && nb_letters.valid() { ret.push(new_current_word.clone()); }

		// Case there is no higher up number of letters possible: exit
		if !nb_letters.decrease() {
			return ret;
		}

		// Case where there's at least one joker in set
		if letter_set.first() == Some(&'0') {
			for child in &self.children {
				new_current_word.push(child.data.unwrap());
				ret.extend(
					child.get_anagrams_internal(
						letter_set[1..].to_vec(), 
						new_current_word.clone(),
						nb_letters.clone(),
						letter_constraints.clone()));
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
			ret.extend(
				node.get_anagrams_internal(
					[letter_set[0..i].to_vec(), letter_set[i+1..].to_vec()].concat(), 
					new_current_word.clone(),
					nb_letters.clone(),
					letter_constraints.clone()));
			new_current_word.pop();
		}

		return ret;
	}
}
