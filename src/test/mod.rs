use crate::str_tree;
use crate::str_tree::Dictionnary;

fn get_anagrams(letters: &str) -> Vec<String> {
	let tree = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");
	return tree.get_anagrams(letters, None, None, None);
}

fn found_in_vec<T>(el: &T, vec: &Vec<T>) -> bool 
where T: std::cmp::PartialEq {
	for el2 in vec.into_iter() {
		if *el == *el2 {
			return true;
		}
	}
	return false;
}

fn unordered_equal<T>(v1: &Vec<T>, v2: &Vec<T>) -> bool 
where T: std::cmp::PartialEq {
	for el in v1.into_iter() {
		if !found_in_vec(el, v2) {
			return false;
		}
	}
	for el in v2.into_iter() {
		if !found_in_vec(el, v1) {
			return false;
		}
	}
	return true;
}

#[test]
fn load_success() {
	let _ = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");
}

#[test]
#[should_panic]
fn load_fail() {
	let _ = str_tree::build_dict_from_file("prout.prout").expect("File not found");
}

#[test]
fn existing_words() {
	let tree = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");
	assert!(tree.is_word("arbre"));
	assert!(tree.is_word("bar"));
	assert!(tree.is_word("barre"));
	assert!(tree.is_word("mazout"));
	assert!(tree.is_word("cenestpasunmotduscrabble"));
	assert!(!tree.is_word("erreur"));
}

#[test]
fn add_word() {
	let mut tree = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");
	assert!(!tree.is_word("erreur"));
	tree.add_word("erreur");
	assert!(tree.is_word("erreur"));
}

#[test]
fn no_double_without_joker() {
	let anagrams = get_anagrams("arbre");

	for i1 in 0..anagrams.len() {
		for i2 in 0..anagrams.len() {
			if i1 == i2 { continue; }
			assert_ne!(anagrams[i1], anagrams[i2]);
		}
	}
}

#[test]
fn all_anagrams_without_joker() {
	let anagrams = get_anagrams("arbre");
	let correct_answer = vec![
		"arbre".to_string(),
		"bar".to_string(),
		"barre".to_string()
	];

	assert!(unordered_equal(&anagrams, &correct_answer));
}

#[test]
fn all_anagrams_with_joker() {
	let anagrams = get_anagrams("arbr0");
	let correct_answer = vec![
		"arbrE".to_string(),
		"bar".to_string(),
		"Bar".to_string(),
		"bAr".to_string(),
		"baR".to_string(),
		"barrE".to_string()
	];

	assert!(unordered_equal(&anagrams, &correct_answer));
}

#[test]
fn no_anagrams() {
	let empty = &Vec::<String>::new();
	assert_eq!(&get_anagrams(""), empty);
	assert_eq!(&get_anagrams("zzz"), empty);
	assert_eq!(&get_anagrams("00"), empty);
}

#[test]
fn nb_letters_constraints() {
	let tree = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");
	let mut correct_answer = vec![
		"arbre".to_string(),
		"barre".to_string()
	];

	assert!(unordered_equal(
		&tree.get_anagrams("arbre", Some(vec![]), None, None), 
		&Vec::<String>::new()));

	assert!(unordered_equal(
		&tree.get_anagrams("arbre", Some(vec![3]), None, None), 
		&vec!["bar".to_string()]));

	assert!(unordered_equal(
		&tree.get_anagrams("arbre", Some(vec![5]), None, None), 
		&correct_answer));

	correct_answer.push("bar".to_string());
	assert!(unordered_equal(
		&tree.get_anagrams("arbre", Some(vec![3, 5]), None, None),
		&correct_answer));
}

#[test]
fn no_letter_actually_used() {
	let tree = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");
	let empty = &Vec::<String>::new();

	assert!(unordered_equal(
		&tree.get_anagrams("", Some(vec![0]), None, None),
		empty));
	assert!(unordered_equal(
		&tree.get_anagrams("", Some(vec![0]), Some(vec![(0, 'b'), (1, 'a'), (2, 'r')]), None),
		empty));
}

#[test]
fn nb_letters_does_not_include_constraints() {
	let tree = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");
	let correct_answer = vec![
		"___re".to_string()
	];

	assert!(unordered_equal(
		&tree.get_anagrams("re", Some(vec![2]), Some(vec![(0, 'b'), (1, 'a'), (2, 'r')]), None),
		&correct_answer));
}

#[test]
fn letters_constraints() {
	let tree = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");

	assert!(unordered_equal(
		&tree.get_anagrams("arbe", None, Some(vec![(2, 'z')]), None), 
		&Vec::<String>::new()));

	assert!(unordered_equal(
		&tree.get_anagrams("rbre", None, Some(vec![(0, 'a')]), None), 
		&vec!["_rbre".to_string()]));
	assert!(unordered_equal(
		&tree.get_anagrams("arbe", None, Some(vec![(1, 'r')]), None), 
		&vec!["a_bre".to_string()]));

	assert!(unordered_equal(
		&tree.get_anagrams("arbe", None, Some(vec![(3, 'r')]), None), 
		&vec!["arb_e".to_string(), "bar_e".to_string()]));

	assert!(unordered_equal(
		&tree.get_anagrams("arbr", None, Some(vec![(4, 'e')]), None), 
		&vec!["arbr_".to_string(), "barr_".to_string(), "bar".to_string()]));
}

#[test]
fn words_constraint() {
	let tree = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");

	let mut correct_answer = vec![
		"bar".to_string()
	];
	let mut constraints = Some(vec![(2, crate::constraints::WordToFill::new("ba".to_string(),"re".to_string()).unwrap())]);
	assert!(unordered_equal(
		&tree.get_anagrams("arbre", Some(vec![2, 3]), None, constraints.clone()), 
		&correct_answer));

	correct_answer = vec![
		"barre".to_string(),
		"bar".to_string()
	];
	constraints = Some(vec![(2, crate::constraints::WordToFill::new("ba".to_string(),"re".to_string()).unwrap())]);
	assert!(unordered_equal(
		&tree.get_anagrams("arbre", None, None, constraints.clone()), 
		&correct_answer));

	correct_answer = vec![
		"arbre".to_string()
	];
	constraints = Some(vec![(2, crate::constraints::WordToFill::new("ar".to_string(),"re".to_string()).unwrap())]);
	assert!(unordered_equal(
		&tree.get_anagrams("arbre", None, None, constraints.clone()), 
		&correct_answer));
}

#[test]
fn all_constraints() {
	let tree = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");
	let mut correct_answer = vec![
		"b_r".to_string()
	];

	assert!(unordered_equal(
		&tree.get_anagrams(
			"rbre", 
			Some(vec![2]), 
			Some(vec![(1, 'a')]), 
			Some(vec![(2, crate::constraints::WordToFill::new("a".to_string(), "bre".to_string()).unwrap())])),
		&correct_answer));

	correct_answer.push("b_rre".to_string());

	assert!(unordered_equal(
		&tree.get_anagrams(
			"rbre", 
			None, 
			Some(vec![(1, 'a')]), 
			Some(Vec::<(u8, crate::constraints::WordToFill)>::new())),
		&correct_answer));

	assert!(unordered_equal(
		&tree.get_anagrams(
			"rbre", 
			None, 
			Some(vec![(1, 'a')]), 
			None),
		&correct_answer));
}

use crate::board;
use crate::board::BoardService;
use crate::board::DeserializingError;


use crate::constraints;
use crate::constraints::{WordToFill, PotentialWordConditions, PotentialWordConditionsBuilder};

#[test]
fn board_serialization() {
	let mut str_board = "".to_string();
	str_board.push_str("6__2___6___2__6");
	str_board.push_str("_5___3___3___5_");
	str_board.push_str("__5___2_2___5__");
	str_board.push_str("2__5___2___5__2");
	str_board.push_str("____5_____5____");
	str_board.push_str("_3___3___3___3_");
	str_board.push_str("__2___2_2___2__");
	str_board.push_str("6__2___a___2__6");
	str_board.push_str("__2___2r2___2__");
	str_board.push_str("_3___3_b_3___3_");
	str_board.push_str("____5__R__5____");
	str_board.push_str("2__5___e___5__2");
	str_board.push_str("__5___2_2___5__");
	str_board.push_str("_5___3___3___5_");
	str_board.push_str("6__2___6___2__");

	let b = board::deserialize(str_board.clone()).expect_err("Unlikely Success");
	assert_eq!(b, DeserializingError::WrongLength);
	str_board.push('!');
	let b2 = board::deserialize(str_board.clone()).expect_err("Unlikely success");
	assert_eq!(b2, DeserializingError::UnknownSymbol);
}

#[test]
fn get_conditions_vertical() {
	let mut str_board = "".to_string();
	str_board.push_str("6__2___6___2__6");
	str_board.push_str("_5___3___3___5_");
	str_board.push_str("__5___2_2___5__");
	str_board.push_str("2__5___2___5__2");
	str_board.push_str("____5_____5____");
	str_board.push_str("_3___3___3___3_");
	str_board.push_str("__2___2_2___2__");
	str_board.push_str("6__2___a___2__6");
	str_board.push_str("__2___2r2___2__");
	str_board.push_str("_3___3_b_3___3_");
	str_board.push_str("____5__R__5____");
	str_board.push_str("2__5___e___5__2");
	str_board.push_str("__5___2_2___5__");
	str_board.push_str("_5___3___3___5_");
	str_board.push_str("6__2___6___2__6");

	let board = board::deserialize(str_board).expect("Error when deserializing board message");
	let mut pw = constraints::PotentialWord::new();

	board.get_conditions(10, 0, &mut pw);
	assert_eq!(pw.get_constraint_nb_letters(), Some(vec![7,8,9,10,11,12,13,14]));
	assert_eq!(pw.get_constraint_letters(), Some(vec![(7, 'r')]));
	assert_eq!(pw.get_constraint_words(), Some(Vec::<(u8, WordToFill)>::new()));

	board.get_conditions(11, 0, &mut pw);
	assert_eq!(pw.get_constraint_nb_letters(), Some(vec![7,8,9,10,11,12,13,14]));
	assert_eq!(pw.get_constraint_letters(), Some(vec![(7, 'e')]));
	assert_eq!(pw.get_constraint_words(), Some(Vec::<(u8, WordToFill)>::new()));

	board.get_conditions(12, 0, &mut pw);
	assert_eq!(pw.get_constraint_nb_letters(), Some(vec![8,9,10,11,12,13,14,15]));
	assert_eq!(pw.get_constraint_letters(), Some(Vec::<(u8, char)>::new()));
	assert_eq!(pw.get_constraint_words(), Some(vec![(7, WordToFill::new("arbre".to_string(), "".to_string()).unwrap())]));

	board.get_conditions(6, 0, &mut pw);
	assert_eq!(pw.get_constraint_nb_letters(), Some(vec![8,9,10,11,12,13,14,15]));
	assert_eq!(pw.get_constraint_letters(), Some(Vec::<(u8, char)>::new()));
	assert_eq!(pw.get_constraint_words(), Some(vec![(7, WordToFill::new("".to_string(), "arbre".to_string()).unwrap())]));

	board.get_conditions(11, 7, &mut pw);
	assert_eq!(pw.get_constraint_nb_letters(), Some(vec![0,1,2,3,4,5,6,7]));
	assert_eq!(pw.get_constraint_letters(), Some(vec![(0, 'e')]));
	assert_eq!(pw.get_constraint_words(), Some(Vec::<(u8, WordToFill)>::new()));

	board.get_conditions(11, 8, &mut pw);
	assert_eq!(pw.get_constraint_nb_letters(), Some(Vec::<u8>::new()));
	assert_eq!(pw.get_constraint_letters(), Some(Vec::<(u8, char)>::new()));
	assert_eq!(pw.get_constraint_words(), Some(Vec::<(u8, WordToFill)>::new()));
}

#[test]
fn get_conditions_horizontal() {
	let mut str_board = "".to_string();
	str_board.push_str("6__2___6___2__6");
	str_board.push_str("_5___3___3___5_");
	str_board.push_str("__5___2_2___5__");
	str_board.push_str("2__5___2___5__2");
	str_board.push_str("____5_____5____");
	str_board.push_str("_3___3___3___3_");
	str_board.push_str("__2___2_2___2__");
	str_board.push_str("6__2___arbre__6");
	str_board.push_str("__2___2_2___2__");
	str_board.push_str("_3___3___3___3_");
	str_board.push_str("____5_____5____");
	str_board.push_str("2__5___2___5__2");
	str_board.push_str("__5___2_2___5__");
	str_board.push_str("_5___3___3___5_");
	str_board.push_str("6__2___6___2__6");

	let board = board::deserialize(str_board).expect("Error when deserializing board message");
	let mut pw = constraints::PotentialWord::new();

	board.get_conditions(10, 0, &mut pw);
	assert_eq!(pw.get_constraint_nb_letters(), Some(Vec::<u8>::new()));
	assert_eq!(pw.get_constraint_letters(), Some(Vec::<(u8, char)>::new()));
	assert_eq!(pw.get_constraint_words(), Some(Vec::<(u8, WordToFill)>::new()));

	board.get_conditions(7, 0, &mut pw);
	assert_eq!(pw.get_constraint_nb_letters(), Some(vec![7,8,9,10]));
	assert_eq!(pw.get_constraint_letters(), Some(vec![(7, 'a'),(8, 'r'),(9, 'b'),(10, 'r'),(11, 'e')]));
	assert_eq!(pw.get_constraint_words(), Some(Vec::<(u8, WordToFill)>::new()));

	board.get_conditions(8, 10, &mut pw);
	assert_eq!(pw.get_constraint_nb_letters(), Some(vec![1,2,3,4,5]));
	assert_eq!(pw.get_constraint_letters(), Some(Vec::<(u8, char)>::new()));
	assert_eq!(pw.get_constraint_words(), Some(vec![(0, WordToFill::new("r".to_string(), "".to_string()).unwrap()),(1, WordToFill::new("e".to_string(), "".to_string()).unwrap())]));
}

#[test]
fn get_score() {
	let mut str_board = "".to_string();
	str_board.push_str("6__2___6___2__6");
	str_board.push_str("_5___3___3___5_");
	str_board.push_str("__5___2_2___5__");
	str_board.push_str("2__5___2___5__2");
	str_board.push_str("____5_____5____");
	str_board.push_str("_3___3___3___3_");
	str_board.push_str("__2___2_2___2__");
	str_board.push_str("6__2___a___2__6");
	str_board.push_str("__2___2r2___2__");
	str_board.push_str("_3___3_bE3___3_");
	str_board.push_str("____5__R2_5____");
	str_board.push_str("2__5___e___5__2");
	str_board.push_str("__5___2_2___5__");
	str_board.push_str("_5___3___3___5_");
	str_board.push_str("6__2___6___2__6");

	let board = board::deserialize(str_board).expect("Error when deserializing board message");
	println!("{}", board.serialize());

	assert_eq!(7, board.get_score("te_se", 10, 5).unwrap());
	assert_eq!(6, board.get_score("tE_se", 10, 5).unwrap());
	assert_eq!(3, board.get_score("te_Se", 10, 5).unwrap());
	assert_eq!(14, board.get_score("te_ses", 10, 5).unwrap());
	assert_eq!(8, board.get_score("te_Ses", 10, 5).unwrap());
	assert_eq!(32, board.get_score("te_fes", 10, 5).unwrap());
}

use crate::board::WordError;

#[test]
fn get_score_errors() {
	let mut str_board = "".to_string();
	str_board.push_str("6__2___6___2__6");
	str_board.push_str("_5___3___3___5_");
	str_board.push_str("__5___2_2___5__");
	str_board.push_str("2__5___2___5__2");
	str_board.push_str("____5_____5____");
	str_board.push_str("_3___3___3___3_");
	str_board.push_str("__2___2_2___2__");
	str_board.push_str("6__2___a___2__6");
	str_board.push_str("__2___2r2___2__");
	str_board.push_str("_3___3_bE3___3_");
	str_board.push_str("____5__R2_5____");
	str_board.push_str("2__5___e___5__2");
	str_board.push_str("__5___2_2___5__");
	str_board.push_str("_5___3___3___5_");
	str_board.push_str("6__2___6___2__6");

	let board = board::deserialize(str_board).expect("Error when deserializing board message");
	println!("{}", board.serialize());

	assert_eq!(Err(WordError::TileOccupied), board.get_score("terse", 10, 5));
	assert_eq!(Err(WordError::UnexpectedUnderscore), board.get_score("tE_s_", 10, 5));
	assert_eq!(Err(WordError::UnknownChar), board.get_score("tE_s!", 10, 5));
}
