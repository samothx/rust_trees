use super::SubNode;

use std::fmt::Debug;

pub struct BTreeNode<K: PartialOrd, V> {
    pub key: K,
    pub value: V,
    pub smaller: SubNode<K, V>,
    pub larger: SubNode<K, V>,
}

impl<K: PartialOrd, V> BTreeNode<K, V> {
    pub fn new(key: K, value: V) -> BTreeNode<K, V> {
        BTreeNode {
            key,
            value,
            smaller: None,
            larger: None,
        }
    }

    pub fn insert_node_rec(&mut self, node: Box<BTreeNode<K, V>>) -> Option<V> {
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

    pub fn traverse_asc(&self, func: &mut dyn FnMut(&K, &V)) {
        if let Some(smaller) = &self.smaller {
            smaller.traverse_asc(func);
        }
        func(&self.key, &self.value);
        if let Some(larger) = &self.larger {
            larger.traverse_asc(func);
        }
    }

    pub fn right_rotate(mut self: Box<Self>) -> std::result::Result<Box<Self>, &'static str> {
        if self.smaller.is_some() {
            let mut smaller = self.smaller.take().expect("unexpected empty node 1");
            self.smaller = smaller.larger.take();
            smaller.larger = Some(self);
            Ok(smaller)
        } else {
            Err("cannot left rotate - smaller subnode is nil")
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        // remove only subnodes - this node has beech checked upstream
        #[cfg(test)]
        if self.key == *key {
            panic!("BTreeNode.remove() cannot semove self");
        }

        let child_link = if *key < self.key {
            &mut self.smaller
        } else {
            &mut self.larger
        };

        if let Some(true) = child_link.as_ref().map(|root| root.key == *key) {
            // delete the root
            let mut child = child_link.take().expect("unexpected empty link");
            let (res, new_child) = if child.smaller.is_some() {
                if child.larger.is_some() {
                    // root has two siblings - swap root with next larger, delete next larger
                    let (key, value) = child.remove_next_larger();
                    child.key = key;
                    let res = std::mem::replace(&mut child.value, value);
                    (Some(res), Some(child))
                } else {
                    // root becomes root.smaller
                    (Some(child.value), child.smaller.take())
                }
            } else if child.larger.is_some() {
                // root becomes root.larger
                (Some(child.value), child.larger.take())
            } else {
                // the tree is empty
                (Some(child.value), None)
            };
            if new_child.is_some() {
                *child_link = new_child;
            }
            res
        } else if let Some(child) = child_link {
            child.remove(key)
        } else {
            None
        }
    }

    pub fn remove_next_larger(&mut self) -> (K, V) {
        match self.larger.as_ref().map(|node| node.smaller.is_some()) {
            Some(true) => {
                // larger has smaller siblings - find the smallest one
                let mut curr = self.larger.as_mut().expect("unexpected empty link");
                while let Some(true) = curr.smaller.as_ref().map(|node| node.smaller.is_some()) {
                    curr = curr.smaller.as_mut().expect("unexpected empty link");
                }
                // current is the parent of the smallest node
                let smallest = curr.smaller.take().expect("unexpected empty node");
                if smallest.larger.is_some() {
                    curr.smaller = smallest.larger;
                }
                (smallest.key, smallest.value)
            }
            Some(false) => {
                // self.larger has no smaller siblings - larger is the one
                let larger = self.larger.take().expect("unexpected empty link");
                if larger.larger.is_some() {
                    self.larger = larger.larger;
                }
                (larger.key, larger.value)
            }
            None => panic!("remove_next_larger - no larger subnode exists"),
        }
    }
}

impl<K: PartialOrd + Debug, V: Debug> BTreeNode<K, V> {
    /// format as:
    /// (kkk,vvv)
    ///   ├─<(kkk,vvv)
    ///   │   ├─<(kkk,vvv)
    ///   │   └─>nil
    ///   └─>(kkk,vvv)
    ///       ├─<(kkk,vvv)
    ///       └─>(kkk,vvv)

    fn to_str_buffer(&self, buffer: &mut String, lead: &str, root: bool, smaller: bool) {
        const J_SMALLER: &str = " └─>";
        const J_LARGER: &str = " ├─<";
        const L_SMALLER: &str = "    ";
        const L_LARGER: &str = " │  ";

        // buffer.push_str(&format!("lead: '{}'\n", lead));
        if root {
            buffer.push_str(&format!("({:?},{:?})\n", self.key, self.value));
        } else {
            let junction = if smaller { J_SMALLER } else { J_LARGER };
            buffer.push_str(&format!(
                "{}{}({:?},{:?})\n",
                lead, junction, self.key, self.value
            ));
        }

        if self.smaller.is_some() || self.larger.is_some() {
            let sub_lead = if root {
                lead.to_string()
            } else if smaller {
                lead.to_string() + L_SMALLER
            } else {
                lead.to_string() + L_LARGER
            };

            if let Some(subnode) = &self.larger {
                subnode.to_str_buffer(buffer, &sub_lead, false, false);
            } else {
                buffer.push_str(&format!("{}{}nil\n", sub_lead, J_LARGER));
            }

            if let Some(subnode) = &self.smaller {
                subnode.to_str_buffer(buffer, &sub_lead, false, true);
            } else {
                buffer.push_str(&format!("{}{}nil\n", sub_lead, J_SMALLER));
            }
        }
    }
}

impl<K: PartialOrd + Debug, V: Debug> ToString for BTreeNode<K, V> {
    fn to_string(&self) -> String {
        let mut buffer = String::new();
        self.to_str_buffer(&mut buffer, "", true, false);
        buffer
    }
}
