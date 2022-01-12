use std::fmt::Debug;

type SubNode<K, V> = Option<Box<TreeNode<K, V>>>;

enum NodeOrd {
    Smaller,
    Larger,
}

#[derive(Debug)]
struct TreeNode<K: PartialOrd + Debug, V: Debug> {
    key: K,
    value: V,
    smaller: SubNode<K, V>,
    larger: SubNode<K, V>,
}

impl<K: PartialOrd + Debug, V: Debug> TreeNode<K, V> {
    fn new(key: K, value: V) -> TreeNode<K, V> {
        TreeNode {
            key,
            value,
            smaller: None,
            larger: None,
        }
    }

    fn insert_node_rec(&mut self, node: Box<TreeNode<K, V>>) -> Option<V> {
        if node.key < self.key {
            if let Some(smaller) = &mut self.smaller {
                smaller.insert_node_rec(node)
            } else {
                self.smaller = Some(node);
                None
            }
        } else if node.key > self.key {
            if let Some(larger) = &mut self.larger {
                larger.insert_node_rec(node)
            } else {
                self.larger = Some(node);
                None
            }
        } else {
            // don't even really need to support this case
            Some(std::mem::replace(&mut self.value, node.value))
        }
    }

    fn traverse_asc(&self, func: &mut dyn FnMut(&K, &V)) {
        if let Some(smaller) = &self.smaller {
            smaller.traverse_asc(func);
        }
        func(&self.key, &self.value);
        if let Some(larger) = &self.larger {
            larger.traverse_asc(func);
        }
    }

    fn traverse_top_down(&self, func: &mut dyn FnMut(&K, &V)) {
        func(&self.key, &self.value);
        if let Some(smaller) = &self.smaller {
            smaller.traverse_top_down(func);
        }
        if let Some(larger) = &self.larger {
            larger.traverse_top_down(func);
        }
    }

    fn traverse_print(&self, depth: u32) {
        let mut offset = "".to_string();
        for _ in 1..depth {
            offset.push_str("  ");
        }
        eprintln!(
            "{}Node {:?} value: {:?}, depth: {}",
            offset, self.key, self.value, depth
        );

        if let Some(smaller) = &self.smaller {
            eprintln!("{}Node {:?} smaller ->", offset, self.key);
            smaller.traverse_print(depth + 1);
        } else {
            eprintln!("{}Node {:?} smaller -> None", offset, self.key);
        }

        if let Some(larger) = &self.larger {
            eprintln!("{}Node {:?} larger -> ", offset, self.key);
            larger.traverse_print(depth + 1);
        } else {
            eprintln!("{}Node {:?} larger -> None", offset, self.key);
        }
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        // key == self.key is not expected - that case is to be handled upstream
        // this is only about removing self.smaller/self.larger from self

        if *key < self.key {
            // we are working on the smaller link
            if self.smaller.is_some() {
                // decouple smaller from self.smaller
                let mut subnode = self.smaller.take().unwrap();
                if *key == subnode.key {
                    // subnode is already removed above, so take care of its children
                    if subnode.smaller.is_some() {
                        if subnode.larger.is_some() {
                            let mut new_smaller =
                                subnode.smaller.take().expect("unexpected empty node 1");
                            let _ = new_smaller.insert_node_rec(
                                subnode.larger.take().expect("unexpected empty node 2"),
                            );
                            self.smaller = Some(new_smaller);
                        } else {
                            self.smaller = subnode.smaller;
                        }
                    } else if subnode.larger.is_some() {
                        self.smaller = subnode.larger;
                    }
                    return Some(subnode.value);
                } else {
                    let res = subnode.remove(key);
                    // restore the link decoupled earlier
                    self.smaller = Some(subnode);
                    return res;
                }
            } else {
                return None;
            }
        } else {
            // we are working on the larger link
            if self.larger.is_some() {
                // decouple smaller from self.smaller
                let mut subnode = self.larger.take().unwrap();
                if *key == subnode.key {
                    // subnode is already removed above, so take care of its children
                    if subnode.smaller.is_some() {
                        if subnode.larger.is_some() {
                            let mut new_larger =
                                subnode.smaller.take().expect("unexpected empty node 3");
                            let _ = new_larger.insert_node_rec(
                                subnode.larger.take().expect("unexpected empty node 4"),
                            );
                            self.larger = Some(new_larger)
                        } else {
                            self.larger = subnode.smaller;
                        }
                    } else if subnode.larger.is_some() {
                        self.larger = subnode.larger;
                    }
                    return Some(subnode.value);
                } else {
                    let res = subnode.remove(key);
                    // restore the link decoupled earlier
                    self.larger = Some(subnode);
                    return res;
                }
            } else {
                return None;
            }
        }
    }
}

#[derive(Debug)]
pub struct BTree<K: PartialOrd + Debug, V: Debug> {
    root: SubNode<K, V>,
}

impl<K: PartialOrd + Debug, V: Debug> Default for BTree<K, V> {
    fn default() -> Self {
        BTree::new()
    }
}

impl<K: PartialOrd + Debug, V: Debug> BTree<K, V> {
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

    pub fn traverse_top_down(&self, func: &mut dyn FnMut(&K, &V)) {
        if let Some(node) = &self.root {
            node.traverse_top_down(func);
        }
    }

    pub fn traverse_print(&self) {
        if let Some(node) = &self.root {
            node.traverse_print(1);
        } else {
            eprintln!("empty root");
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
        let values = [10u32, 20, 5, 15, 25, 3, 8];
        let mut tree: BTree<u32, String> = BTree::new();
        for value in values {
            assert_eq!(tree.insert(value, value.to_string()), None);
        }

        tree.traverse_print();

        assert_eq!(tree.remove(&10), Some(10.to_string()));
        assert!(!tree.contains(&10));
        tree.traverse_print();
        assert_eq!(tree.remove(&5), Some(5.to_string()));
        assert!(!tree.contains(&5));

        assert_eq!(tree.remove(&20), Some(20.to_string()));
        assert!(!tree.contains(&20));

        let values = [10u32, 20, 5];
        let mut tree: BTree<u32, String> = BTree::new();
        for value in values {
            assert_eq!(tree.insert(value, value.to_string()), None);
        }
        assert_eq!(tree.remove(&20), Some(20.to_string()));
        assert!(!tree.contains(&20));
        assert_eq!(tree.remove(&5), Some(5.to_string()));
        assert!(!tree.contains(&5));

        let values = [10u32, 20, 5, 15, 25, 3, 8];

        let mut tree: BTree<u32, String> = BTree::new();
        for value in values {
            assert_eq!(tree.insert(value, value.to_string()), None);
        }

        for value in values {
            eprintln!("remove {} from: {:?}", value, tree);
            assert_eq!(tree.remove(&value), Some(value.to_string()));
            eprintln!("tree after remove {}: {:?}", value, tree);
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
        tree.traverse_print();
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

        let values = [10u32, 5, 3, 8, 20, 15, 25];
        let mut iter = values.iter();
        // let func = ;
        tree.traverse_top_down(&mut move |key: &u32, value: &String| {
            let xpctd_key = iter.next().expect("unexpected end of values encountered");
            eprintln!("traverse_top_down({:?},{:?})", key, value);
            assert_eq!(key, xpctd_key);
            assert_eq!(value, &xpctd_key.to_string());
        });
    }
}
