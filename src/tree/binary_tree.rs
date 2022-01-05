pub struct BTree<K: PartialOrd, V: Clone> {
    root: SubNode<K, V>,
    // iter_pos: Option<&'a TreeNode<K, V>>,
}

impl<'a, K, V> BTree<K, V>
where
    K: PartialOrd,
    V: 'a + Clone,
{
    pub fn new() -> BTree<K, V> {
        BTree {
            root: SubNode::Nil,
            // iter_pos: None,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let SubNode::Node(root) = &mut self.root {
            root.insert(key, value)
        } else {
            self.root = SubNode::Node(Box::new(TreeNode::new(key, value)));
            None
        }
    }

    pub fn find(&'a self, key: K) -> Option<&'a V> {
        if let SubNode::Node(root) = &self.root {
            root.find(key)
        } else {
            None
        }
    }

    pub fn find_mut(&'a mut self, key: K) -> Option<&'a mut V> {
        if let SubNode::Node(root) = &mut self.root {
            root.find_mut(key)
        } else {
            None
        }
    }
}

#[derive(Debug)]
enum SubNode<K: PartialOrd, V: Clone> {
    Node(Box<TreeNode<K, V>>),
    Nil,
}

#[derive(Debug)]
struct TreeNode<K: PartialOrd, V: Clone> {
    key: K,
    value: V,
    left: SubNode<K, V>,
    right: SubNode<K, V>,
}

impl<'a, K, V> TreeNode<K, V>
where
    K: PartialOrd,
    V: 'a + Clone,
{
    fn new(key: K, value: V) -> TreeNode<K, V> {
        TreeNode {
            key,
            value,
            left: SubNode::Nil,
            right: SubNode::Nil,
        }
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        if key < self.key {
            if let SubNode::Node(left) = &mut self.left {
                left.insert(key, value)
            } else {
                self.left = SubNode::Node(Box::new(TreeNode::new(key, value)));
                None
            }
        } else if key > self.key {
            if let SubNode::Node(right) = &mut self.right {
                right.insert(key, value)
            } else {
                self.right = SubNode::Node(Box::new(TreeNode::new(key, value)));
                None
            }
        } else {
            let tmp = (self.value).clone();
            self.value = value;
            Some(tmp)
        }
    }

    fn find(&'a self, key: K) -> Option<&'a V> {
        if key < self.key {
            if let SubNode::Node(left) = &self.left {
                left.find(key)
            } else {
                None
            }
        } else if key > self.key {
            if let SubNode::Node(right) = &self.right {
                right.find(key)
            } else {
                None
            }
        } else {
            Some(&self.value)
        }
    }

    fn find_mut(&'a mut self, key: K) -> Option<&'a mut V> {
        if key < self.key {
            if let SubNode::Node(left) = &mut self.left {
                left.find_mut(key)
            } else {
                None
            }
        } else if key > self.key {
            if let SubNode::Node(right) = &mut self.right {
                right.find_mut(key)
            } else {
                None
            }
        } else {
            Some(&mut self.value)
        }
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
        assert_eq!(tree.find(10), Some(&10.to_string()));
        assert_eq!(tree.find(11), None);
    }

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
            assert_eq!(tree.find(value), Some(&value.to_string()));
        }

        assert_eq!(tree.find(11), None)
    }

    fn test_mut() {
        let values = [10u32, 20, 5, 15, 25, 3, 8];
        let mut tree: BTree<u32, String> = BTree::new();
        for value in values {
            assert_eq!(tree.insert(value, value.to_string()), None);
        }

        for value in values {
            if let Some(found) = tree.find_mut(value) {
                *found = (value * 2).to_string();
            } else {
                panic!("key not found: {}", value)
            }
        }

        for value in values {
            assert_eq!(tree.find(value), Some(&(value * 2).to_string()));
        }
    }
}
