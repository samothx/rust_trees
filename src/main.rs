// use chrono::Utc;
use rust_tree::tree::binary_tree::BTree;
// use rust_tree::{list::linked_list::LinkedList, util::make_list_string};

// const SAMPLES: usize = 10_000;

fn main() {
    let mut tree: BTree<u32, String> = BTree::new();
    tree.insert(10u32, "10".to_string());
    // tree.insert(5u32, "5".to_string());
    println!("{:?}", &tree);

    let mut values = [10u32, 20, 5, 15, 25, 3, 8];

    let mut tree: BTree<u32, String> = BTree::new();
    assert_eq!(tree.smallest(), None);
    for value in values {
        assert_eq!(tree.insert(value, value.to_string()), None);
    }

    println!("{:?}", &tree);

    values.sort();
    values.reverse();
    let mut key = values[0];
    for index in 1..values.len() {
        let expected = values[index];
        if let Some((lkey, lval)) = tree.smaller(&key) {
            eprintln!("looking for smaller than {:?}, got {:?}", key, lkey);
            assert_eq!(expected, *lkey);
            assert_eq!(expected.to_string(), *lval);
            key = *lkey;
        } else {
            eprintln!("looking for smaller than {:?}, got None", key);
            panic!("expected {}, found None @ key {}", expected, key);
        }
    }
    assert_eq!(tree.smaller(&key), None)

    /*    let values = make_list_usize(SAMPLES);

       let mut list = LinkedList::new();
       let start = Utc::now();
       for val in &values {
           list.push(val);
       }
       let duration = Utc::now() - start;
       eprintln!(
           "duration for push(usize:{}): {} seconds",
           SAMPLES,
           duration.num_nanoseconds().unwrap() as f64 / 1e9
       );

       let values = make_list_string(SAMPLES);

       let mut list = LinkedList::new();
       let start = Utc::now();
       for val in &values {
           list.push(val.clone());
       }
       let duration = Utc::now() - start;
       eprintln!(
           "duration for push(string:{}): {} seconds",
           SAMPLES,
           duration.num_nanoseconds().unwrap() as f64 / 1e9
       );

       let start = Utc::now();
       for val in &values {
           list.contains(val);
       }
       let duration = Utc::now() - start;
       eprintln!(
           "duration for contains(string:{}): {} seconds",
           SAMPLES,
           duration.num_nanoseconds().unwrap() as f64 / 1e9
       );

    */
}
