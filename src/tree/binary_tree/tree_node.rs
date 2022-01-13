use super::SubNode;
use std::fmt::Debug;

#[derive(Debug)]
pub struct TreeNode<K: PartialOrd, V> {
    pub key: K,
    pub value: V,
    pub smaller: SubNode<K, V>,
    pub larger: SubNode<K, V>,
}

impl<K: PartialOrd, V> TreeNode<K, V> {
    pub fn new(key: K, value: V) -> TreeNode<K, V> {
        TreeNode {
            key,
            value,
            smaller: None,
            larger: None,
        }
    }

    pub fn insert_node_rec(&mut self, node: Box<TreeNode<K, V>>) -> Option<V> {
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

    pub fn remove(&mut self, key: &K) -> Option<V> {
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
                    Some(subnode.value)
                } else {
                    let res = subnode.remove(key);
                    // restore the link decoupled earlier
                    self.smaller = Some(subnode);
                    res
                }
            } else {
                None
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
                    Some(subnode.value)
                } else {
                    let res = subnode.remove(key);
                    // restore the link decoupled earlier
                    self.larger = Some(subnode);
                    res
                }
            } else {
                None
            }
        }
    }
}

impl<K: PartialOrd + Debug, V: Debug> TreeNode<K, V> {
    /// print as:
    /// (kkk,vvv)
    ///   ├─ (kkk,vvv)
    ///   │   ├─ (kkk,vvv)
    ///   │   └─ nil
    ///   └─ (kkk,vvv)
    ///       ├─ (kkk,vvv)
    ///       └─ (kkk,vvv)

    pub fn to_string(&self, buffer: &mut String, lead: &str, root: bool, smaller: bool) {
        const J_SMALLER: &str = " ├─";
        const J_LARGER: &str = " └─";
        const L_SMALLER: &str = " │ ";
        const L_LARGER: &str = "   ";

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

            if let Some(subnode) = &self.smaller {
                subnode.to_string(buffer, &sub_lead, false, true);
            } else {
                buffer.push_str(&format!("{}{}nil\n", sub_lead, J_SMALLER));
            }

            if let Some(subnode) = &self.larger {
                subnode.to_string(buffer, &sub_lead, false, false);
            } else {
                buffer.push_str(&format!("{}{}nil\n", sub_lead, J_LARGER));
            }
        }
    }
}
