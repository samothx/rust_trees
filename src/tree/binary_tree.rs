use std::fmt::Debug;

type SubNode<K, V> = Option<Box<TreeNode<K, V>>>;

pub struct BTree<'a, K: PartialOrd + Debug, V: Clone> {
    root: SubNode<K, V>,
    iter_pos: Option<&'a K>,
}

impl<'a, K, V> BTree<'a, K, V>
where
    K: PartialOrd + Debug,
    V: Clone,
{
    pub fn new() -> BTree<'a, K, V> {
        BTree {
            root: None,
            iter_pos: None,
        }
    }

    // TODO: add size, contains, remove, iterator, try_insert, adapt to std collection api

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(node) = &mut self.root {
            let mut curr = node;
            loop {
                if key < curr.key {
                    match &mut curr.smaller {
                        Some(node) => curr = node,
                        s @ None => {
                            *s = Some(Box::new(TreeNode::new(key, value)));
                            return None;
                        }
                    }
                } else if key > curr.key {
                    match &mut curr.bigger {
                        Some(node) => curr = node,
                        s @ None => {
                            *s = Some(Box::new(TreeNode::new(key, value)));
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
                    if let Some(subnode) = &curr.bigger {
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
                    if let Some(subnode) = &curr.bigger {
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
                    if let Some(subnode) = &mut curr.bigger {
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

    fn remove_node(&mut self, _key: &K) -> Option<TreeNode<K, V>> {
        unimplemented!()
    }

    // TODO: Find out how to return a value from a deleted entry
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(_node) = self.remove_node(key) {
            unimplemented!()
        } else {
            None
        }
    }

    fn smallest_node(&self) -> Option<&TreeNode<K, V>> {
        if let Some(root) = &self.root {
            root.smallest_node()
        } else {
            None
        }
    }

    pub fn smallest(&self) -> Option<(&K, &V)> {
        if let Some(node) = self.smallest_node() {
            Some((&node.key, &node.value))
        } else {
            None
        }
    }

    fn largest_node(&self) -> Option<&TreeNode<K, V>> {
        if let Some(root) = &self.root {
            root.largest_node()
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
            root.larger_node(key)
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

    fn smaller_node(&self, key: &K) -> Option<&TreeNode<K, V>> {
        if let Some(root) = &self.root {
            root.smaller_node(key)
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
}

/*
impl<'a, K, V> Iterator for BTree<'a, K, V>
where
    K: PartialOrd,
    V: Clone,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(_curr_node) = &self.iter_pos {
            unimplemented!()
        } else {
            if let Some(node) = self.smallest_node() {
                self.iter_pos = Some(&node);
                Some((&node.key, &node.value))
            } else {
                None
            }
        }
    }
}
*/

#[derive(Debug)]
struct TreeNode<K: PartialOrd + Debug, V: Clone> {
    key: K,
    value: V,
    smaller: SubNode<K, V>,
    bigger: SubNode<K, V>,
}

impl<K, V> TreeNode<K, V>
where
    K: PartialOrd + Debug,
    V: Clone,
{
    fn new(key: K, value: V) -> TreeNode<K, V> {
        TreeNode {
            key,
            value,
            smaller: None,
            bigger: None,
        }
    }

    fn smallest_node(&self) -> Option<&TreeNode<K, V>> {
        if let Some(smaller) = &self.smaller {
            smaller.smallest_node()
        } else {
            Some(&self)
        }
    }

    fn smaller_node(&self, key: &K) -> Option<&TreeNode<K, V>> {
        if self.key < *key {
            // search larger
            if let Some(bigger) = &self.bigger {
                let res = bigger.smaller_node(key);
                if res.is_none() {
                    Some(self)
                } else {
                    res
                }
            } else {
                Some(self)
            }
        } else {
            // search smaller
            if let Some(smaller) = &self.smaller {
                smaller.smaller_node(key)
            } else {
                None
            }
        }
    }

    fn largest_node(&self) -> Option<&TreeNode<K, V>> {
        if let Some(bigger) = &self.bigger {
            bigger.largest_node()
        } else {
            Some(&self)
        }
    }

    fn larger_node(&self, key: &K) -> Option<&TreeNode<K, V>> {
        if self.key > *key {
            // search smaller
            if let Some(smaller) = &self.smaller {
                let res = smaller.larger_node(key);
                if res.is_none() {
                    Some(self)
                } else {
                    res
                }
            } else {
                Some(self)
            }
        } else {
            // search larger
            if let Some(bigger) = &self.bigger {
                bigger.larger_node(key)
            } else {
                None
            }
        }
    }

    fn remove_node(&mut self, _key: &K) -> Option<(bool, V)> {
        /*        if *key < self.key {
                   unimplemented!()
               } else if *key > self.key {
                   unimplemented!()
               } else {
                   Some((true, self.value))
               }
        */
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first_level() {
        let mut tree: BTree<u32, String> = BTree::new();
        assert_eq!(tree.insert(10, 10.to_string()), None);
        assert_eq!(tree.insert(10, 10.to_string()), Some(10.to_string()));
        assert_eq!(tree.find(&10), Some(&10.to_string()));
        assert_eq!(tree.find(&11), None);
    }

    #[test]
    fn test_next_level() {
        let values = [10u32, 20, 5, 15, 25, 3, 8];

        let mut tree: BTree<u32, String> = BTree::new();
        for value in values {
            assert_eq!(tree.insert(value, value.to_string()), None);
        }

        for value in values {
            assert_eq!(
                tree.insert(value, value.to_string()),
                Some(value.to_string())
            );
        }

        for value in values {
            assert_eq!(tree.find(&value), Some(&value.to_string()));
        }

        assert_eq!(tree.find(&11), None)
    }

    #[test]
    fn test_remove() {
        let values = [10u32, 20, 5, 15, 25, 3, 8];

        let mut tree: BTree<u32, String> = BTree::new();
        for value in values {
            assert_eq!(tree.insert(value, value.to_string()), None);
        }

        for value in values {
            assert_eq!(tree.remove(&value), Some(value.to_string()));
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
        let values = [10u32, 20, 5, 15, 25, 3, 8];

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
    fn test_larger() {
        let mut values = [10u32, 20, 5, 15, 25, 3, 8];

        let mut tree: BTree<u32, String> = BTree::new();
        assert_eq!(tree.smallest(), None);
        for value in values {
            assert_eq!(tree.insert(value, value.to_string()), None);
        }

        values.sort();
        let mut key = values[0];
        for index in 1..values.len() {
            let expected = values[index];
            if let Some((lkey, lval)) = tree.larger(&key) {
                eprintln!("looking for larger than {:?}, got {:?}", key, lkey);
                assert_eq!(expected, *lkey);
                assert_eq!(expected.to_string(), *lval);
                key = *lkey;
            } else {
                eprintln!("looking for larger than {:?}, got None", key);
                panic!("expected {}, found None @ key {}", expected, key);
            }
        }
        assert_eq!(tree.larger(&key), None)
    }

    #[test]
    fn test_smaller() {
        let mut values = [10u32, 20, 5, 15, 25, 3, 8];

        let mut tree: BTree<u32, String> = BTree::new();
        assert_eq!(tree.smallest(), None);
        for value in values {
            assert_eq!(tree.insert(value, value.to_string()), None);
        }

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
}
