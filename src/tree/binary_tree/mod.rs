use std::fmt::{Debug, Formatter};
use tree_node::TreeNode;

type SubNode<K, V> = Option<Box<TreeNode<K, V>>>;

mod tree_node;

pub struct BTree<K: PartialOrd, V> {
    root: SubNode<K, V>,
}

impl<K: PartialOrd, V> Default for BTree<K, V> {
    fn default() -> Self {
        BTree::new()
    }
}

impl<K: PartialOrd + Debug, V: Debug> BTree<K, V> {}
impl<K: PartialOrd, V> BTree<K, V> {
    pub fn new() -> BTree<K, V> {
        BTree { root: None }
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
        self.smallest_node().map(|node| (&node.key, &node.value))
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
        self.largest_node().map(|node| (&node.key, &node.value))
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first_level() {
        let mut tree: BTree<String, String> = BTree::new();
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

        let mut tree: BTree<String, String> = BTree::new();
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

        let mut tree: BTree<String, String> = BTree::new();
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
        let mut tree: BTree<u32, String> = BTree::new();
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
        let mut tree: BTree<u32, String> = BTree::new();
        for value in values {
            assert_eq!(tree.insert(value, value.to_string()), None);
        }
        assert_eq!(tree.remove(&20), Some(20.to_string()));
        assert!(!tree.contains(&20));
        assert_eq!(tree.remove(&5), Some(5.to_string()));
        assert!(!tree.contains(&5));

        let values = [10u32, 20, 5, 15, 25, 3, 8, 4, 1, 9, 6, 13, 17, 22, 27];

        let mut tree: BTree<u32, String> = BTree::new();
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
        let mut tree: BTree<u32, String> = BTree::new();
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

        let mut tree: BTree<u32, String> = BTree::new();
        assert_eq!(tree.smallest(), None);
        for value in values {
            assert_eq!(tree.insert(value, value.to_string()), None);
        }
        assert_eq!(tree.smallest(), Some((&3, &3.to_string())))
    }

    #[test]
    fn test_largest() {
        let values = [10u32, 20, 5, 15, 25, 3, 8];

        let mut tree: BTree<u32, String> = BTree::new();
        assert_eq!(tree.smallest(), None);
        for value in values {
            assert_eq!(tree.insert(value, value.to_string()), None);
        }

        assert_eq!(tree.largest(), Some((&25, &25.to_string())))
    }

    #[test]
    fn test_smaller() {
        let mut values = [10u32, 20, 5, 15, 25, 3, 8];

        let mut tree: BTree<u32, String> = BTree::new();
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

        let mut tree: BTree<u32, String> = BTree::new();
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

        let mut tree: BTree<u32, String> = BTree::new();
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

        let mut tree: BTree<u32, String> = BTree::new();
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

impl<K: PartialOrd + Debug, V: Debug> Debug for BTree<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(root) = &self.root {
            write!(f, "{}", root.to_string())
        } else {
            write!(f, "nil")
        }
    }
}
