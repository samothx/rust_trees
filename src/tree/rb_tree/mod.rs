use std::fmt::{Debug, Formatter};
use tree_node::TreeNode;

mod tree_node;

type SubNode<K, V> = Option<Box<TreeNode<K, V>>>;

pub struct RBTree<K: PartialOrd, V> {
    root: SubNode<K, V>,
}

impl<K: PartialOrd, V> Default for RBTree<K, V> {
    fn default() -> Self {
        RBTree::new()
    }
}

impl<K: PartialOrd, V> RBTree<K, V> {
    pub fn new() -> RBTree<K, V> {
        RBTree { root: None }
    }

    // TODO: add size, contains, remove, iterator, try_insert, adapt to std collection api

    pub fn insert_rec(&mut self, key: K, value: V) -> Option<V> {
        let new_node = Box::new(TreeNode::new(key, value));
        if let Some(node) = &mut self.root {
            node.insert_node_rec(new_node)
        } else {
            self.root = Some(new_node);
            None
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(node) = &mut self.root {
            let mut curr = node;
            loop {
                if key < curr.key {
                    match &mut curr.smaller {
                        Some(node) => curr = node,
                        smaller @ None => {
                            *smaller = Some(Box::new(TreeNode::new(key, value)));
                            return None;
                        }
                    }
                } else if key > curr.key {
                    match &mut curr.larger {
                        Some(node) => curr = node,
                        larger @ None => {
                            *larger = Some(Box::new(TreeNode::new(key, value)));
                            return None;
                        }
                    }
                } else {
                    return Some(std::mem::replace(&mut curr.value, value));
                }
            }
        } else {
            self.root = Some(Box::new(TreeNode::new(key, value)));
            None
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
        if self.root.is_some() {
            // take  & unwrap root to circumnavigate ownership issues
            // this has to be undone later if root is not being removed
            let mut root = self.root.take().expect("unexpected empty root");
            if *key == root.key {
                // key found at root - remove root
                if root.smaller.is_some() {
                    // root.smaller will be the new root
                    if root.larger.is_some() {
                        // have to reinsert root.larger
                        let mut new_root = root.smaller.take().expect("unexpected empty subnode 1");
                        let _dummy = new_root.insert_node_rec(
                            root.larger.take().expect("unexpected empty subnode 2"),
                        );
                        self.root = Some(new_root);
                    } else {
                        self.root = root.smaller;
                    }
                } else if root.larger.is_some() {
                    // root.larger will be the new root
                    self.root = root.larger;
                }
                Some(root.value)
            } else if *key < root.key {
                if root.smaller.is_some() {
                    // same as above - take & unwrap the value that might or might not be removed
                    // this has to be undone later if root.smaller is not being removed
                    let mut smaller = root.smaller.take().expect("unexpected empty subnode 3");
                    if *key == smaller.key {
                        // remove root.smaller - it is already unchained - now chain up its children
                        if smaller.smaller.is_some() {
                            // smaller.smaller will be the new root.smaller
                            if smaller.larger.is_some() {
                                // have to reinsert smaller.larger first
                                let mut ins_root =
                                    smaller.smaller.take().expect("unexpected empty subnode 4");
                                let _dummy = ins_root.insert_node_rec(
                                    smaller.larger.take().expect("unexpected empty subnode 5"),
                                );
                                root.smaller = Some(ins_root);
                            } else {
                                root.smaller = smaller.smaller;
                            }
                        } else {
                            // smaller.larger will be the new root.smaller
                            root.smaller = smaller.larger;
                        }
                        self.root = Some(root);
                        Some(smaller.value)
                    } else {
                        let res = smaller.remove(key);
                        root.smaller = Some(smaller);
                        self.root = Some(root);
                        res
                    }
                } else {
                    self.root = Some(root);
                    None
                }
            } else {
                if root.larger.is_some() {
                    // same as above - take & unwrap the value that might or might not be removed
                    // this has to be undone later if root.larger is not being removed
                    let mut larger = root.larger.take().expect("unexpected empty subnode 6");
                    if *key == larger.key {
                        // remove smaller
                        if larger.smaller.is_some() {
                            if larger.larger.is_some() {
                                let mut ins_node =
                                    larger.smaller.take().expect("unexpected empty subnode 7");
                                let _dummy = ins_node.insert_node_rec(
                                    larger.larger.take().expect("unexpected empty subnode 8"),
                                );
                                root.larger = Some(ins_node);
                            } else {
                                root.larger = larger.smaller;
                            }
                        } else {
                            root.larger = larger.larger;
                        }
                        self.root = Some(root);
                        Some(larger.value)
                    } else {
                        let res = larger.remove(key);
                        root.larger = Some(larger);
                        self.root = Some(root);
                        res
                    }
                } else {
                    self.root = Some(root);
                    None
                }
            }
        } else {
            None
        }
    }

    fn smallest_node(&self) -> Option<&TreeNode<K, V>> {
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
        if let Some(node) = self.smallest_node() {
            Some((&node.key, &node.value))
        } else {
            None
        }
    }

    fn smaller_node(&self, key: &K) -> Option<&TreeNode<K, V>> {
        if let Some(root) = &self.root {
            let mut candidate: Option<&TreeNode<K, V>> = None;
            let mut curr = root;
            loop {
                // eprintln!("smaller_none({:?}), curr {:?}", key, curr.key);
                if curr.key < *key {
                    // search larger
                    if let Some(larger) = &curr.larger {
                        // eprintln!("smaller_node({:?}), go larger", key);
                        candidate = Some(&curr);
                        curr = larger;
                    } else {
                        return Some(&curr);
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
        if let Some(node) = self.smaller_node(key) {
            Some((&node.key, &node.value))
        } else {
            None
        }
    }

    fn largest_node(&self) -> Option<&TreeNode<K, V>> {
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
        if let Some(node) = self.largest_node() {
            Some((&node.key, &node.value))
        } else {
            None
        }
    }

    fn larger_node(&self, key: &K) -> Option<&TreeNode<K, V>> {
        if let Some(root) = &self.root {
            let mut candidate: Option<&TreeNode<K, V>> = None;
            let mut curr = root;
            loop {
                // eprintln!("smaller_none({:?}), curr {:?}", key, curr.key);
                if curr.key > *key {
                    // search smaller
                    if let Some(smaller) = &curr.smaller {
                        // eprintln!("smaller_node({:?}), go larger", key);
                        candidate = Some(&curr);
                        curr = smaller;
                    } else {
                        return Some(&curr);
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
        if let Some(node) = self.larger_node(key) {
            Some((&node.key, &node.value))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first_level() {
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
    fn test_next_level() {
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
    fn test_insert_rec() {
        let values = ["10", "20", "05", "15", "25", "03", "08"];

        let mut tree: RBTree<String, String> = RBTree::new();
        for value in values {
            assert_eq!(
                tree.insert_rec(value.to_string(), String::from("v1_") + value),
                None
            );
        }
        for value in values {
            assert!(tree.contains(&value.to_string()))
        }
    }

    #[test]
    fn test_remove() {
        eprintln!("test_remove");
        let values = [10u32, 20, 5, 15, 25, 3, 8, 4, 1, 9, 6, 13, 17, 22, 27];
        let mut tree: RBTree<u32, String> = RBTree::new();
        for value in values {
            assert_eq!(tree.insert(value, value.to_string()), None);
        }

        eprintln!("{:?}\n", &tree);

        assert_eq!(tree.remove(&10), Some(10.to_string()));
        assert!(!tree.contains(&10));

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
    fn test_mut() {
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
    fn test_smallest() {
        let values = [10u32, 20, 5, 15, 25, 3, 8, 22, 24];

        let mut tree: RBTree<u32, String> = RBTree::new();
        assert_eq!(tree.smallest(), None);
        for value in values {
            assert_eq!(tree.insert(value, value.to_string()), None);
        }
        assert_eq!(tree.smallest(), Some((&3, &3.to_string())))
    }

    #[test]
    fn test_largest() {
        let values = [10u32, 20, 5, 15, 25, 3, 8];

        let mut tree: RBTree<u32, String> = RBTree::new();
        assert_eq!(tree.smallest(), None);
        for value in values {
            assert_eq!(tree.insert(value, value.to_string()), None);
        }

        assert_eq!(tree.largest(), Some((&25, &25.to_string())))
    }

    #[test]
    fn test_smaller() {
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
    fn test_larger() {
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
    fn test_contains() {
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
    fn test_traverse() {
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
