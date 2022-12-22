mod constraints;

pub trait ConstraintNbLetters: Clone {
	fn sort_and_fuse(&mut self);
	fn decrease(&mut self) -> bool;
	fn valid(&self) -> bool;
}
pub trait ConstraintLetters: Clone {
	fn sort_and_fuse(&mut self);
	fn decrease(&mut self) -> Option<char>;
}
