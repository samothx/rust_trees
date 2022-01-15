use crate::tree::rb_tree::rbtree_node::Color;
// use rand::Rng;
use rbtree_node::RBTreeNode;
use std::fmt::{Debug, Formatter};

mod rbtree_node;

type SubNode<K, V> = Option<Box<RBTreeNode<K, V>>>;

#[derive(PartialEq, Debug)]
pub enum Branch {
    Smaller,
    Larger,
}

#[derive(PartialEq, Debug)]
pub enum InsertState {
    Clean,
    Conflict,
    LeftRotate,
    RightRotate,
    ChgdColor,
}

pub struct RBTree<K: PartialOrd, V> {
    root: SubNode<K, V>,
}

impl<K: PartialOrd + Debug, V: Debug> Default for RBTree<K, V> {
    fn default() -> Self {
        RBTree::new()
    }
}

// impl<K: PartialOrd + Debug, V: Debug> RBTree<K, V> {

impl<K: PartialOrd + Debug, V: Debug> RBTree<K, V> {
    pub fn new() -> RBTree<K, V> {
        RBTree { root: None }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let (res, insert_state) = if let Some(root) = &mut self.root {
            root.insert_node_rb(Box::new(RBTreeNode::new(key, value)), true)
        } else {
            self.root = Some(Box::new(RBTreeNode::new_black(key, value)));
            return None;
        };

        // eprintln!("insert into root returned insert_state {:?}", insert_state);

        match insert_state {
            InsertState::Conflict => {
                panic!("Unexpected conflict in root")
            }
            InsertState::Clean => res,
            InsertState::ChgdColor => panic!("Unexpected insert state in root: {:?}", insert_state),
            InsertState::LeftRotate => {
                let root = self.root.take().expect("unexpected empty root node");

                match root.left_rotate() {
                    Ok(mut new_child) => {
                        new_child.color = Color::Black;
                        self.root = Some(new_child);
                        res
                    }
                    Err((_old_child, err)) => {
                        // eprintln!("Failed to rotate: {}\n{:?}", err, self);
                        panic!("failed to left-rotate: {}", err);
                    }
                }
            }
            InsertState::RightRotate => {
                let root = self.root.take().expect("unexpected empty root node");
                match root.right_rotate() {
                    Ok(mut new_child) => {
                        new_child.color = Color::Black;
                        self.root = Some(new_child);
                        res
                    }
                    Err((_old_child, err)) => {
                        // eprintln!("Failed to rotate: {}\n{:?}", err, self);
                        panic!("failed to right-rotate: {}", err);
                    }
                }
            }
        }
    }

    pub fn traverse_asc(&self, func: &mut dyn FnMut(&K, &V)) {
        if let Some(node) = &self.root {
            node.traverse_asc(func);
        }
    }

    pub fn contains(&self, key: &K) -> bool {
        if let Some(node) = &self.root {
            let mut curr = node;
            // loop {
            loop {
                if *key < curr.key {
                    if let Some(subnode) = &curr.smaller {
                        curr = subnode;
                    } else {
                        return false;
                    }
                } else if *key > curr.key {
                    if let Some(subnode) = &curr.larger {
                        curr = subnode;
                    } else {
                        return false;
                    }
                } else {
                    return true;
                }
            }
        } else {
            false
        }
    }

    pub fn find(&self, key: &K) -> Option<&V> {
        if let Some(node) = &self.root {
            let mut curr = node;
            loop {
                if *key < curr.key {
                    if let Some(subnode) = &curr.smaller {
                        curr = subnode;
                    } else {
                        return None;
                    }
                } else if *key > curr.key {
                    if let Some(subnode) = &curr.larger {
                        curr = subnode;
                    } else {
                        return None;
                    }
                } else {
                    return Some(&curr.value);
                }
            }
        } else {
            None
        }
    }

    pub fn find_mut(&mut self, key: &K) -> Option<&mut V> {
        if let Some(node) = &mut self.root {
            let mut curr = node;
            loop {
                if *key < curr.key {
                    if let Some(subnode) = &mut curr.smaller {
                        curr = subnode;
                    } else {
                        return None;
                    }
                } else if *key > curr.key {
                    if let Some(subnode) = &mut curr.larger {
                        curr = subnode;
                    } else {
                        return None;
                    }
                } else {
                    return Some(&mut curr.value);
                }
            }
        } else {
            None
        }
    }

    // TODO: Find out how to return a value from a deleted entry
    // TODO: try recursive version first

    /// #remove
    /// handles removes on layer 1 & 2 of the tree then uses recursive TreeNode.remove()
    pub fn remove(&mut self, key: &K) -> Option<V> {
        match self.root.as_ref().map(|root| root.key == *key) {
            Some(true) => {
                // delete the root
                let mut root = *self.root.take().expect("unexpected empty node");
                if root.smaller.is_some() {
                    if root.larger.is_some() {
                        let mut new_root = root.smaller.take().expect("unexpected empty node");
                        new_root
                            .insert_node_rec(root.larger.take().expect("unexpected empty node"));
                        self.root = Some(new_root);
                    } else {
                        self.root = root.smaller;
                    }
                } else if root.larger.is_some() {
                    self.root = root.larger;
                }
                Some(root.value)
            }
            Some(false) => {
                // let the root/siblings node delete matching siblings recursively
                if let Some(root) = &mut self.root {
                    root.remove(key)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn smallest_node(&self) -> Option<&RBTreeNode<K, V>> {
        if let Some(root) = &self.root {
            let mut curr = root;
            while let Some(subnode) = &curr.smaller {
                curr = subnode;
            }
            Some(curr)
        } else {
            None
        }
    }

    // TODO: add a mut version returning Option<(&K, &mut V)> ?
    pub fn smallest(&self) -> Option<(&K, &V)> {
        self.smallest_node().map(|node| (&node.key, &node.value))
    }

    fn smaller_node(&self, key: &K) -> Option<&RBTreeNode<K, V>> {
        if let Some(root) = &self.root {
            let mut candidate: Option<&RBTreeNode<K, V>> = None;
            let mut curr = root;
            loop {
                // eprintln!("smaller_none({:?}), curr {:?}", key, curr.key);
                if curr.key < *key {
                    // search larger
                    if let Some(larger) = &curr.larger {
                        // eprintln!("smaller_node({:?}), go larger", key);
                        candidate = Some(curr);
                        curr = larger;
                    } else {
                        return Some(curr);
                    }
                } else if curr.key >= *key {
                    // search smaller
                    // eprintln!("smaller_node({:?}), go smaller", key);
                    if let Some(smaller) = &curr.smaller {
                        curr = smaller;
                    } else {
                        return candidate;
                    }
                }
            }
        } else {
            None
        }
    }

    pub fn smaller(&self, key: &K) -> Option<(&K, &V)> {
        self.smaller_node(key).map(|node| (&node.key, &node.value))
    }

    fn largest_node(&self) -> Option<&RBTreeNode<K, V>> {
        if let Some(root) = &self.root {
            let mut curr = root;
            while let Some(subnode) = &curr.larger {
                curr = subnode;
            }
            Some(curr)
        } else {
            None
        }
    }

    pub fn largest(&self) -> Option<(&K, &V)> {
        self.largest_node().map(|node| (&node.key, &node.value))
    }

    fn larger_node(&self, key: &K) -> Option<&RBTreeNode<K, V>> {
        if let Some(root) = &self.root {
            let mut candidate: Option<&RBTreeNode<K, V>> = None;
            let mut curr = root;
            loop {
                // eprintln!("smaller_none({:?}), curr {:?}", key, curr.key);
                if curr.key > *key {
                    // search smaller
                    if let Some(smaller) = &curr.smaller {
                        // eprintln!("smaller_node({:?}), go larger", key);
                        candidate = Some(curr);
                        curr = smaller;
                    } else {
                        return Some(curr);
                    }
                } else if curr.key <= *key {
                    // search larger
                    // eprintln!("smaller_node({:?}), go smaller", key);
                    if let Some(larger) = &curr.larger {
                        curr = larger;
                    } else {
                        return candidate;
                    }
                }
            }
        } else {
            None
        }
    }

    pub fn larger(&self, key: &K) -> Option<(&K, &V)> {
        self.larger_node(key).map(|node| (&node.key, &node.value))
    }

    pub fn check_rules(&self) -> std::result::Result<usize, String> {
        if let Some(root) = &self.root {
            root.check_rules(true, false)
        } else {
            Ok(0)
        }
    }
}

impl<K: PartialOrd + Debug, V: Debug> Debug for RBTree<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(root) = &self.root {
            write!(f, "{}", root.to_string())
        } else {
            write!(f, "nil")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;

    #[test]
    fn rb_test_first_level() {
        let mut tree: RBTree<String, String> = RBTree::new();
        assert_eq!(tree.insert(10.to_string(), "v0_10".to_string()), None);
        assert_eq!(
            tree.insert(10.to_string(), "v1_10".to_string()),
            Some("v0_10".to_string())
        );
        assert_eq!(tree.find(&"10".to_string()), Some(&"v1_10".to_string()));
        assert_eq!(tree.find(&"11".to_string()), None);
    }

    #[test]
    fn rb_test_next_level() {
        let values = ["10", "20", "05", "15", "25", "03", "08"];

        let mut tree: RBTree<String, String> = RBTree::new();
        for value in values {
            assert_eq!(
                tree.insert(value.to_string(), String::from("v1_") + value),
                None
            );
        }

        for value in values {
            assert_eq!(
                tree.insert(value.to_string(), String::from("v2_") + value),
                Some(String::from("v1_") + value)
            );
        }

        for value in values {
            assert_eq!(
                tree.find(&value.to_string()),
                Some(&(String::from("v2_") + value))
            );
        }

        assert_eq!(tree.find(&11.to_string()), None)
    }

    #[test]
    fn rb_test_remove() {
        eprintln!("test_remove");
        let values = [10u32, 20, 5, 15, 25, 3, 8, 4, 1, 9, 6, 13, 17, 22, 27];
        let mut tree: RBTree<u32, String> = RBTree::new();
        for value in values {
            assert_eq!(tree.insert(value, value.to_string()), None);
        }

        eprintln!("{:?}\n", &tree);

        assert_eq!(tree.remove(&10), Some(10.to_string()));
        assert!(!tree.contains(&10));
        if let Err(msg) = tree.check_rules() {
            panic!("invalid tree after remove {}: {}", 10, msg);
        }

        eprintln!("after remove 10\n{:?}\n", &tree);

        assert_eq!(tree.remove(&5), Some(5.to_string()));
        assert!(!tree.contains(&5));

        eprintln!("after remove 5\n{:?}\n", &tree);

        assert_eq!(tree.remove(&20), Some(20.to_string()));
        assert!(!tree.contains(&20));

        eprintln!("after remove 20\n{:?}\n", &tree);

        let values = [10u32, 20, 5];
        let mut tree: RBTree<u32, String> = RBTree::new();
        for value in values {
            assert_eq!(tree.insert(value, value.to_string()), None);
        }
        assert_eq!(tree.remove(&20), Some(20.to_string()));
        assert!(!tree.contains(&20));
        assert_eq!(tree.remove(&5), Some(5.to_string()));
        assert!(!tree.contains(&5));

        let values = [10u32, 20, 5, 15, 25, 3, 8, 4, 1, 9, 6, 13, 17, 22, 27];

        let mut tree: RBTree<u32, String> = RBTree::new();
        for value in values {
            assert_eq!(tree.insert(value, value.to_string()), None);
        }

        for value in values {
            eprintln!("remove {} from:\n{:?}\n", value, &tree);
            assert_eq!(tree.remove(&value), Some(value.to_string()));
            eprintln!("tree after remove {}:\n{:?}\n", value, &tree);
            assert_eq!(tree.find(&value), None);
            assert_eq!(tree.remove(&value), None);
        }
    }

    #[test]
    fn rb_test_find_mut() {
        let values = [10u32, 20, 5, 15, 25, 3, 8];
        let mut tree: RBTree<u32, String> = RBTree::new();
        for value in values {
            assert_eq!(tree.insert(value, value.to_string()), None);
        }
        for value in values {
            if let Some(found) = tree.find_mut(&value) {
                *found = (value * 2).to_string();
            } else {
                panic!("key not found: {}", value)
            }
        }

        for value in values {
            assert_eq!(tree.find(&value), Some(&(value * 2).to_string()));
        }
    }

    #[test]
    fn rb_test_smallest() {
        let values = [10u32, 20, 5, 15, 25, 3, 8, 22, 24];

        let mut tree: RBTree<u32, String> = RBTree::new();
        assert_eq!(tree.smallest(), None);
        for value in values {
            assert_eq!(tree.insert(value, value.to_string()), None);
        }
        assert_eq!(tree.smallest(), Some((&3, &3.to_string())))
    }

    #[test]
    fn rb_test_largest() {
        let values = [10u32, 20, 5, 15, 25, 3, 8];

        let mut tree: RBTree<u32, String> = RBTree::new();
        assert_eq!(tree.smallest(), None);
        for value in values {
            assert_eq!(tree.insert(value, value.to_string()), None);
        }

        assert_eq!(tree.largest(), Some((&25, &25.to_string())))
    }

    #[test]
    fn rb_test_smaller() {
        let mut values = [10u32, 20, 5, 15, 25, 3, 8];

        let mut tree: RBTree<u32, String> = RBTree::new();
        for value in values {
            assert_eq!(tree.insert(value, value.to_string()), None);
        }

        values.sort();
        values.reverse();
        let mut iter = values.iter();
        let mut key = iter.next().expect("No test cases found");
        for val in iter {
            if let Some((lkey, lval)) = tree.smaller(&key) {
                eprintln!("looking for smaller than {:?}, got {:?}", key, lkey);
                assert_eq!(val, lkey);
                assert_eq!(val.to_string(), *lval);
                key = lkey;
            } else {
                eprintln!("looking for smaller than {:?}, got None", key);
                panic!("expected {}, found None @ key {}", val, key);
            }
        }

        assert_eq!(tree.smaller(&key), None);

        for val in values {
            if let Some((lkey, lval)) = tree.smaller(&(val + 1)) {
                eprintln!("looking for smaller than {:?}, got {:?}", val + 1, lkey);
                assert_eq!(val, *lkey);
                assert_eq!(val.to_string(), *lval);
            } else {
                eprintln!("looking for smaller than {:?}, got None", key);
                panic!("expected {}, found None @ key {}", val, key);
            }
        }
    }

    #[test]
    fn rb_test_larger() {
        let mut values = [10u32, 20, 5, 15, 25, 3, 8];

        let mut tree: RBTree<u32, String> = RBTree::new();
        for value in values {
            assert_eq!(tree.insert(value, value.to_string()), None);
        }

        values.sort();
        let mut iter = values.iter();
        let mut key = iter.next().expect("empty test array");
        for val in iter {
            if let Some((lkey, lval)) = tree.larger(&key) {
                eprintln!("looking for smaller than {:?}, got {:?}", key, lkey);
                assert_eq!(val, lkey);
                assert_eq!(val.to_string(), *lval);
                key = lkey;
            } else {
                eprintln!("looking for smaller than {:?}, got None", key);
                panic!("expected {}, found None @ key {}", val, key);
            }
        }

        assert_eq!(tree.larger(&key), None);

        for val in values {
            if let Some((lkey, lval)) = tree.larger(&(val - 1)) {
                eprintln!("looking for smaller than {:?}, got {:?}", val - 1, lkey);
                assert_eq!(val, *lkey);
                assert_eq!(val.to_string(), *lval);
            } else {
                eprintln!("looking for smaller than {:?}, got None", key);
                panic!("expected {}, found None @ key {}", val, key);
            }
        }
    }

    #[test]
    fn rb_test_contains() {
        let values = [10u32, 20, 5, 15, 25, 3, 8];

        let mut tree: RBTree<u32, String> = RBTree::new();
        assert_eq!(tree.smallest(), None);
        for value in values {
            assert_eq!(tree.insert(value, value.to_string()), None);
        }
        for val in values {
            assert_eq!(tree.contains(&val), true);
        }
        assert_eq!(tree.contains(&100), false);
    }

    #[test]
    fn rb_test_traverse() {
        let mut values = [10u32, 20, 5, 15, 25, 3, 8];

        let mut tree: RBTree<u32, String> = RBTree::new();
        for value in values {
            assert_eq!(tree.insert(value, value.to_string()), None);
        }

        values.sort();
        let mut iter = values.iter();
        // let func = ;
        tree.traverse_asc(&mut move |key: &u32, value: &String| {
            let xpctd_key = iter.next().expect("unexpected end of values encountered");
            eprintln!("traverse_asc({:?},{:?})", key, value);
            assert_eq!(key, xpctd_key);
            assert_eq!(value, &xpctd_key.to_string());
        });
    }

    #[test]
    fn rb_test_insert() {
        let mut tree = RBTree::new();
        for val in 1..=100 {
            assert_eq!(tree.insert(val, val.to_string()), None);
            if let std::result::Result::Err(msg) = tree.check_rules() {
                eprintln!(
                    "RB violation after insert of {}, msg: {}\n{:?}",
                    val, msg, tree
                );
                panic!("{}", msg)
            }
            // eprintln!("after insert ascending {}\n{:?}", val, tree);
        }
        // eprintln!("after insert ascending\n{:?}", tree);

        let mut count = 0;
        let mut cref = &mut count;
        tree.traverse_asc(&mut move |_key: &u32, _value: &String| {
            *cref += 1;
        });
        assert_eq!(count, 100);

        let mut tree = RBTree::new();
        for val in (1..=100).rev() {
            assert_eq!(tree.insert(val, val.to_string()), None);
            if let std::result::Result::Err(msg) = tree.check_rules() {
                eprintln!(
                    "RB violation after insert of {}, msg: {}\n{:?}",
                    val, msg, tree
                );
                panic!("{}", msg)
            }

            // eprintln!("after insert descending {}\n{:?}", val, tree);
        }
        // eprintln!("after insert descending\n{:?}", tree);

        let mut count = 0;
        let mut cref = &mut count;

        tree.traverse_asc(&mut move |_key: &u32, _value: &String| {
            *cref += 1;
        });

        assert_eq!(count, 100, "invalid node count");

        let mut tree = RBTree::new();
        let mut rng = rand::thread_rng();

        // eprintln!("testing random tree");
        const MAX: u32 = 10000;
        let mut entries = Vec::new();
        for _ in 1..=MAX {
            loop {
                let val = rng.gen_range(1..MAX * 4);
                if !tree.contains(&val) {
                    assert_eq!(tree.insert(val, val.to_string()), None);
                    entries.push(val);
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

        let mut count = 0;
        let mut cref = &mut count;
        tree.traverse_asc(&mut move |_key: &u32, _value: &String| {
            *cref += 1;
        });
        assert_eq!(
            count, MAX,
            "invalid node count {}, should be {}",
            count, MAX
        );

        // eprintln!("after random insert\n{:?}", tree);
        for val in entries {
            assert_eq!(tree.find(&val), Some(&val.to_string()));
        }
    }
}
