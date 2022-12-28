use crate::board::SIDE;
use crate::str_tree::{read_lines, cnt_lines};
use crate::str_tree::Dictionnary;
use crate::str_tree::{ConstraintNbLetters, ConstraintLetters, ConstraintWords};

pub struct StrTree {
	data: Option<char>,
	is_word: bool,
	children: Vec<StrTree>
}

impl std::fmt::Debug for StrTree {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut string = match self.data {
			None => "Head of tree".to_string(),
			Some(c) => c.to_string()
		};
		string.push_str(" - ");
		if self.is_word {
			string.push_str("word - ");
		} else {
			string.push_str("not a word - ");
		}
		string.push_str(&self.children.len().to_string());
		string.push_str(" children");
		f.write_str(&string)
	}
}

impl Dictionnary for StrTree {
	fn build_dict_from_file(filename: &str) -> std::io::Result<StrTree> {
		let mut ret = StrTree::init();
		match ret.fill_with_file(filename) {
			Err(e) => return Err(e),
			Ok(_) => return Ok(ret)
		};
	}

	fn get_anagrams<CNbL, CL, CW>(
		&self, 
		letter_set: &str, 
		mut nb_letters: CNbL,
		mut letter_constraints: CL,
		mut word_constraints: CW) 
	-> Vec<String> 
	where CNbL: ConstraintNbLetters, CL: ConstraintLetters, CW: ConstraintWords {
		let mut letter_set_vec:Vec<char> = letter_set.chars().collect();
		letter_set_vec.sort_unstable();
		nb_letters.sort_and_fuse();
		letter_constraints.sort_and_fuse();
		word_constraints.sort_and_fuse();

		let mut max_nb_letters = 0;
		let mut valid_nb_letter = [false; SIDE];
		let mut obligatory_letters:[Option<char>; SIDE] = [None; SIDE];
		let mut words_to_fill: [Option<(&StrTree, String)>; SIDE] = Default::default();
		for i in 0..SIDE {
			if nb_letters.decrease() {
				valid_nb_letter[i] = true;
				max_nb_letters = i;
			}
			obligatory_letters[i] = letter_constraints.decrease();
			words_to_fill[i] = self.get_next_word_to_fill(word_constraints.decrease('_'));
		}

		return self.get_anagrams_internal(
			0,
			letter_set_vec, 
			"".to_string(), 
			max_nb_letters, 
			&valid_nb_letter, 
			&obligatory_letters,
			&words_to_fill);
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

	fn new_word_with_append(word: &str, c: char) -> String {
		let mut ret = word.to_string();
		ret.push(c);
		return ret;
	}

	fn get_next_word_to_fill<'a, 'b: 'a>(&'b self, wtf: Option<String>) -> Option<(&'a StrTree, String)> 
	{
		let binding = wtf?;
		let segments:Vec<&str> = binding.split('_').collect();
		let node = self.get_node(segments[0]).expect("Constraint word doesn't exist");
		Some((&node, segments[1].to_string()))
	}

	fn get_anagrams_internal(
		&self, 
		depth: usize,
		letter_set: Vec<char>, 
		current_word: String,
		max_nb_letters: usize,
		valid_nb_letter: &[bool; SIDE],
		obligatory_letters: &[Option<char>; SIDE],
		words_to_fill: &[Option<(&StrTree, String)>; SIDE])
	-> Vec<String> {
		let new_current_word = |c: char| Self::new_word_with_append(&current_word, c);

		let length = current_word.len();

		match self.data {
			None => (),
			Some(c) => {
				let ret = match words_to_fill[length-1] {
					None => false,
					Some((ref node, ref end)) => {
						if let Some(child) = node.get_child(c) {
							!child.is_word(&end)
						} else { true }
					}
				};
				if ret {
					return Vec::<String>::new();
				}
			}
		};

		// Case the next letter is a constraint: continue only on that branch if it exists
		if let Some(constraint) = obligatory_letters[length] {
			let node = match self.get_child(constraint) {
				None => return Vec::<String>::new(),
				Some(node) => node 
			};
			return node.get_anagrams_internal(
				depth,
				letter_set.clone(),
				new_current_word('_'),
				max_nb_letters,
				&valid_nb_letter,
				&obligatory_letters,
				&words_to_fill);
		}

		let mut ret = Vec::<String>::new();
		if self.is_word && valid_nb_letter[depth] { ret.push(current_word.clone()); }

		// Case there is no higher up number of letters possible: exit
		if depth >= max_nb_letters { return ret; }

		// Case where there's at least one joker in set
		if letter_set.first() == Some(&'0') {
			for child in &self.children {
				ret.extend(
					child.get_anagrams_internal(
						depth + 1,
						letter_set[1..].to_vec(), 
						new_current_word(child.data.unwrap().to_ascii_uppercase()),
						max_nb_letters,
						&valid_nb_letter,
						&obligatory_letters,
						&words_to_fill));
			}
		}

		// Now take every letter in the set, and see if you can build a word from it
		for i in 0..letter_set.len() {
			// This avoids repetition coming from identitical letters
			if i > 0 && letter_set[i-1]==letter_set[i] {
				continue;
			}

			match self.get_child(letter_set[i]) {
				None => continue,
				Some(node) => ret.extend(
					node.get_anagrams_internal(
						depth + 1,
						[letter_set[0..i].to_vec(), letter_set[i+1..].to_vec()].concat(), 
						new_current_word(node.data.unwrap()),
						max_nb_letters,
						&valid_nb_letter,
						&obligatory_letters,
						&words_to_fill))
			};
		}

		return ret;
	}
}
