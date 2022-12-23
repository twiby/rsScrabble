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
		"arbre".to_string(),
		"bar".to_string(),
		"barre".to_string()
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
		"barre".to_string()
	];

	assert!(unordered_equal(
		&tree.get_anagrams("re", Some(vec![2]), Some(vec![(0, 'b'), (1, 'a'), (2, 'r')]), None),
		&correct_answer));
}

#[test]
fn letters_constraints() {
	let tree = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");

	let empty = &Vec::<String>::new();
	let mut correct_answer = vec![
		"arbre".to_string()
	];

	assert!(unordered_equal(
		&tree.get_anagrams("arbe", None, Some(vec![(2, 'z')]), None), 
		empty));

	assert!(unordered_equal(
		&tree.get_anagrams("rbre", None, Some(vec![(0, 'a')]), None), 
		&correct_answer));
	assert!(unordered_equal(
		&tree.get_anagrams("arbe", None, Some(vec![(1, 'r')]), None), 
		&correct_answer));

	correct_answer.push("barre".to_string());
	assert!(unordered_equal(
		&tree.get_anagrams("arbe", None, Some(vec![(3, 'r')]), None), 
		&correct_answer));

	correct_answer.push("bar".to_string());
	assert!(unordered_equal(
		&tree.get_anagrams("arbr", None, Some(vec![(4, 'e')]), None), 
		&correct_answer));
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
	let correct_answer = vec![
		"bar".to_string()
	];

	assert!(unordered_equal(
		&tree.get_anagrams(
			"rbre", 
			Some(vec![2]), 
			Some(vec![(1, 'a')]), 
			Some(vec![(2, crate::constraints::WordToFill::new("a".to_string(), "bre".to_string()).unwrap())])),
		&correct_answer));
}
