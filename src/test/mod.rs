use crate::str_tree;
use crate::str_tree::StrTree;
use crate::str_tree::Dictionnary;

fn load_tree(path: &str) -> StrTree {
	match str_tree::build_dict_from_file(path) {
		Err(_) => panic!("Unable to open file {0:?}", path),
		Ok(dict) => dict
	}
}

fn get_anagrams(letters: &str) -> Vec<String> {
	let tree = load_tree("src/test/words.txt");
	return tree.get_anagrams(letters, None, None);
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
	let _ = load_tree("src/test/words.txt");
}

#[test]
#[should_panic]
fn load_fail() {
	let _ = load_tree("prout.prout");
}

#[test]
fn existing_words() {
	let tree = load_tree("src/test/words.txt");
	assert!(tree.is_word("arbre"));
	assert!(tree.is_word("bar"));
	assert!(tree.is_word("barre"));
	assert!(tree.is_word("mazout"));
	assert!(tree.is_word("cenestpasunmotduscrabble"));
	assert!(!tree.is_word("erreur"));
}

#[test]
fn add_word() {
	let mut tree = load_tree("src/test/words.txt");
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
	let tree = load_tree("src/test/words.txt");
	let mut correct_answer = vec![
		"arbre".to_string(),
		"barre".to_string()
	];

	assert!(unordered_equal(
		&tree.get_anagrams("arbre", Some(vec![]), None), 
		&Vec::<String>::new()));

	assert!(unordered_equal(
		&tree.get_anagrams("arbre", Some(vec![3]), None), 
		&vec!["bar".to_string()]));

	assert!(unordered_equal(
		&tree.get_anagrams("arbre", Some(vec![5]), None), 
		&correct_answer));

	correct_answer.push("bar".to_string());
	assert!(unordered_equal(
		&tree.get_anagrams("arbre", Some(vec![3, 5]), None),
		&correct_answer));
}

#[test]
fn letters_constraints() {
	let tree = load_tree("src/test/words.txt");

	let empty = &Vec::<String>::new();
	let mut correct_answer = vec![
		"arbre".to_string()
	];

	assert!(unordered_equal(
		&tree.get_anagrams("arbe", None, Some(vec![(2, 'z')])), 
		empty));

	assert!(unordered_equal(
		&tree.get_anagrams("arbe", None, Some(vec![(1, 'r')])), 
		&correct_answer));

	correct_answer.push("barre".to_string());
	assert!(unordered_equal(
		&tree.get_anagrams("arbe", None, Some(vec![(3, 'r')])), 
		&correct_answer));

	correct_answer.push("bar".to_string());
	assert!(unordered_equal(
		&tree.get_anagrams("arbr", None, Some(vec![(4, 'e')])), 
		&correct_answer));
}
