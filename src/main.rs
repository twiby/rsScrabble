mod str_tree;
use str_tree::StrTree;

fn main() {
        let mut _tree = StrTree::init();


        _tree.add_word("arbre");
        _tree.add_word("ame");

        println!("{}, {}", 'a', _tree.check_child('a'));
        println!("{}, {}", 'r', _tree.check_child('r'));
        println!("{}, {}", 'b', _tree.check_child('b'));
        println!("{}, {}", 'r', _tree.check_child('r'));
        println!("{}, {}", 'e', _tree.check_child('e'));

        let _node = _tree.get_child('a').unwrap();
        println!("\n");
        println!("{}, {}", 'r', _node.check_child('r'));
        println!("{}, {}", 'm', _node.check_child('m'));
        println!("{}, {}", 'b', _node.check_child('b'));

        println!("Hello, world!");
}
