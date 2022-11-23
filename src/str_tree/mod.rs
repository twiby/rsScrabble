
pub struct StrTree<'a> {
	data: Option<char>,
	nb_letters: u8,
	is_word: bool,
	parent: Option<&'a StrTree<'a>>,
	children: Vec<&'a StrTree<'a>>
}

impl StrTree<'_> {
	pub fn init() -> Self {
		return Self{
			data:None, 
			nb_letters: 0, 
			is_word: false, 
			parent: None, 
			children: Vec::new()};
	}
}
