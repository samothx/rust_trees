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
        let child_link = if *key < self.key {
            &mut self.smaller
        } else {
            &mut self.larger
        };

        match child_link.as_ref().map(|node| node.key == *key) {
            Some(true) => {
                // remove child
                let mut child = child_link.take().expect("unexpected empty link");
                if child.smaller.is_some() {
                    let mut new_child = child.smaller.take().expect("unexpected empty link");
                    if child.larger.is_some() {
                        new_child
                            .insert_node_rec(child.larger.take().expect("unexpected empty link"));
                    }
                    *child_link = Some(new_child);
                } else if child.larger.is_some() {
                    *child_link = child.larger;
                }
                Some(child.value)
            }
            Some(false) => {
                if let Some(node) = child_link {
                    node.remove(key)
                } else {
                    None
                }
            }
            None => None,
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
