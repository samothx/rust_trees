use colored::Colorize;
use rand::Rng;
use rust_tree::tree::rb_tree::RBTree;

fn main() {
    let mut tree = RBTree::new();
    let mut rng = rand::thread_rng();

    // eprintln!("testing random tree");
    const MAX: u32 = 40;
    for _ in 1..=MAX {
        loop {
            let val = rng.gen_range(1..MAX * 4);
            if !tree.contains(&val) {
                assert_eq!(tree.insert(val, val.to_string()), None);
                if let std::result::Result::Err(msg) = tree.check_rules() {
                    eprintln!(
                        "RB violation after insert of {}, msg: {}\n{:?}",
                        val, msg, tree
                    );
                    panic!("{}", msg)
                }
                // eprintln!("after insert {}\n{:?}", val, tree);
                break;
            }
        }
    }
    println!("{:?}", tree);
}
