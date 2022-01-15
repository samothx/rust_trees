use super::SubNode;
use crate::tree::rb_tree::{Branch, InsertState};
use colored::*;
use std::fmt::Debug;

#[derive(PartialEq, Debug)]
pub enum Color {
    Red,
    Black,
}

#[derive(PartialEq, Debug)]
pub enum RotDir {
    Right,
    Left,
}

pub struct RBTreeNode<K: PartialOrd, V> {
    pub key: K,
    pub value: V,
    pub color: Color,
    pub smaller: SubNode<K, V>,
    pub larger: SubNode<K, V>,
}

impl<K: PartialOrd + Debug, V> RBTreeNode<K, V> {
    pub fn new(key: K, value: V) -> RBTreeNode<K, V> {
        RBTreeNode {
            key,
            value,
            color: Color::Red,
            smaller: None,
            larger: None,
        }
    }

    pub fn new_black(key: K, value: V) -> RBTreeNode<K, V> {
        RBTreeNode {
            key,
            value,
            color: Color::Black,
            smaller: None,
            larger: None,
        }
    }

    pub fn insert_node_rb(
        &mut self,
        node: Box<RBTreeNode<K, V>>,
        is_root: bool,
    ) -> (Option<V>, InsertState) {
        eprintln!(
            "({:?}).insert_node_rb(key: {:?}) root: {}",
            self.key, node.key, is_root
        );
        if node.key == self.key {
            eprintln!("insert_node_rb() update");
            return (
                Some(std::mem::replace(&mut self.value, node.value)),
                InsertState::Clean,
            );
        }

        let (child_link, uncle_link, branch) = if node.key < self.key {
            (&mut self.smaller, &mut self.larger, Branch::Smaller)
        } else {
            (&mut self.larger, &mut self.smaller, Branch::Larger)
        };

        if let Some(child_node) = child_link {
            let (res, ins_state) = child_node.insert_node_rb(node, false);
            eprintln!(
                "({:?}).insert_node_rb() root: {} insert into subnode returned insert_state {:?}",
                self.key, is_root, ins_state
            );
            match ins_state {
                InsertState::Clean => return (res, ins_state),
                InsertState::ChgdColor => {
                    if is_root {
                        return (res, InsertState::Clean);
                    } else {
                        // subnode has changed color to red
                        if self.color == Color::Red {
                            // tell my parent that me and my sibling are red
                            return (
                                res,
                                match branch {
                                    Branch::Smaller => InsertState::LeftConflict,
                                    Branch::Larger => InsertState::RightConflict,
                                },
                            );
                        } else {
                            return (res, InsertState::Clean);
                        }
                    }
                }
                InsertState::LeftConflict | InsertState::RightConflict => {
                    // child reports that it and its child are red
                    if let Some(true) = uncle_link.as_mut().map(|node| node.color == Color::Red) {
                        // uncle is red
                        if !is_root {
                            self.color = Color::Red;
                        }
                        uncle_link.as_mut().map(|uncle| uncle.color = Color::Black);
                        child_link.as_mut().map(|child| child.color = Color::Black);
                        return (
                            res,
                            if is_root {
                                InsertState::Clean
                            } else {
                                InsertState::ChgdColor
                            },
                        );
                    } else {
                        if ins_state == InsertState::RightConflict {
                            return (res, InsertState::LeftRotate);
                        } else {
                            return (res, InsertState::RightRotate);
                        }
                    }
                }
                InsertState::LeftRotate => match self.rotate_child(RotDir::Left, branch) {
                    Ok(_) => return (res, InsertState::Clean),
                    Err(err) => panic!("{}", err),
                },
                InsertState::RightRotate => match self.rotate_child(RotDir::Right, branch) {
                    Ok(_) => return (res, InsertState::Clean),
                    Err(err) => panic!("{}", err),
                },
            }
        } else {
            assert_eq!(node.color, Color::Red);
            *child_link = Some(node);
            return (
                None,
                if self.color == Color::Black {
                    InsertState::Clean
                } else {
                    match branch {
                        Branch::Smaller => InsertState::LeftConflict,
                        Branch::Larger => InsertState::RightConflict,
                    }
                },
            );
        }
    }

    pub fn insert_node_rec(&mut self, node: Box<RBTreeNode<K, V>>) -> Option<V> {
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

    /// rotate left - larger side moves up
    /// rotate anti clockwise (in this notation)
    ///    (self)              (self)
    ///     ├─<(?)              ├─<(?)
    ///     └─>(c)              └─>(l)
    ///         ├─<(l)            ├─<(l,l)
    ///         │   ├─<(l,l)      └─>(c)
    ///         │   └─>(l,s)          ├─<(l,s)
    ///         └─>(s)                └─>(s)
    ///             ├─<(s,l)              ├─<(s,l)
    ///             └─>(s,s)              └─>(s,s)

    pub fn rotate_child(
        &mut self,
        direction: RotDir,
        branch: Branch,
    ) -> std::result::Result<(), &'static str> {
        let child_link = match branch {
            Branch::Smaller => &mut self.smaller,
            Branch::Larger => &mut self.larger,
        };

        if child_link.is_some() {
            let node = child_link.take().unwrap();
            let res = match direction {
                RotDir::Left => node.left_rotate(),
                RotDir::Right => node.right_rotate(),
            };
            match res {
                Ok(new_child) => {
                    *child_link = Some(new_child);
                    Ok(())
                }
                Err((old_child, msg)) => {
                    *child_link = Some(old_child);
                    Err(msg)
                }
            }
        } else {
            Err("no child found to rotate")
        }
    }

    /// rotate left - larger side moves up
    /// rotate anti clockwise (in this notation)
    ///    (self)            (l)
    ///     ├─<(l)            ├─<(l,l)
    ///     │   ├─<(l,l)      └─>(self)
    ///     │   └─>(l,s)          ├─<(l,s)
    ///     └─>(s)                └─>(s)
    ///         ├─<(s,l)              ├─<(s,l)
    ///         └─>(s,s)              └─>(s,s)

    pub fn left_rotate(
        mut self: Box<Self>,
    ) -> std::result::Result<Box<Self>, (Box<Self>, &'static str)> {
        if self.larger.is_some() {
            let mut larger = self.larger.take().expect("unexpected empty node 1");
            larger.color = Color::Black;
            self.color = Color::Red;
            self.larger = larger.smaller.take();
            larger.smaller = Some(self);
            Ok(larger)
        } else {
            Err((self, "cannot left rotate - larger subnode is nil"))
        }
    }

    /// rotate right - smaller side moves up
    /// rotate clockwise (in this notation)
    ///    (self)           (s)
    ///     ├─<(l)           ├─<(self)
    ///     │   ├─<(l,l)     │   ├─<(l)
    ///     │   └─>(l,s)     │   │   ├─<(l,l)
    ///     └─>(s)           │   │   └─>(l,s)     
    ///         ├─<(s,l)     │   └─>(s,l)     
    ///         └─>(s,s)     └─>(s,s)          

    pub fn right_rotate(
        mut self: Box<Self>,
    ) -> std::result::Result<Box<Self>, (Box<Self>, &'static str)> {
        if self.smaller.is_some() {
            let mut smaller = self.smaller.take().expect("unexpected empty node 1");
            smaller.color = Color::Black;
            self.color = Color::Red;
            self.smaller = smaller.larger.take();
            smaller.larger = Some(self);
            Ok(smaller)
        } else {
            Err((self, "cannot right rotate - smaller subnode is nil"))
        }
    }
}

impl<K: PartialOrd + Debug, V: Debug> RBTreeNode<K, V> {
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

        let node_str = format!("({:?},{:?})", self.key, self.value);
        let node_str = match self.color {
            Color::Red => node_str.red().to_string(),
            Color::Black => node_str.blue().to_string(),
        };
        if root {
            buffer.push_str(&format!("{}\n", node_str));
        } else {
            let junction = if smaller { J_SMALLER } else { J_LARGER };
            let out = format!("{}{}{}\n", lead, junction, node_str);
            buffer.push_str(&out);
        }

        if self.smaller.is_some() || self.larger.is_some() {
            let sub_lead = if root {
                lead.to_string()
            } else if smaller {
                lead.to_string() + L_SMALLER
            } else {
                lead.to_string() + L_LARGER
            };

            let nil = "nil".blue().to_string();

            if let Some(subnode) = &self.larger {
                subnode.to_str_buffer(buffer, &sub_lead, false, false);
            } else {
                buffer.push_str(&format!("{}{}{}\n", sub_lead, J_LARGER, nil));
            }

            if let Some(subnode) = &self.smaller {
                subnode.to_str_buffer(buffer, &sub_lead, false, true);
            } else {
                buffer.push_str(&format!("{}{}{}\n", sub_lead, J_SMALLER, nil));
            }
        }
    }
}

impl<K: PartialOrd + Debug, V: Debug> ToString for RBTreeNode<K, V> {
    fn to_string(&self) -> String {
        let mut buffer = String::new();
        self.to_str_buffer(&mut buffer, "", true, false);
        buffer
    }
}

#[cfg(test)]
mod test {
    use crate::tree::rb_tree::rbtree_node::{Branch, Color, RBTreeNode, RotDir};

    #[test]
    fn test_rotate() {
        let mut tree = RBTreeNode {
            key: 10,
            value: 10.to_string(),
            color: Color::Black,
            smaller: Some(Box::new(RBTreeNode {
                key: 5,
                value: 5.to_string(),
                color: Color::Black,
                smaller: Some(Box::new(RBTreeNode {
                    key: 3,
                    value: 3.to_string(),
                    color: Color::Black,
                    smaller: Some(Box::new(RBTreeNode {
                        key: 2,
                        value: 1.to_string(),
                        color: Color::Black,
                        smaller: None,
                        larger: None,
                    })),
                    larger: Some(Box::new(RBTreeNode {
                        key: 4,
                        value: 4.to_string(),
                        color: Color::Black,
                        smaller: None,
                        larger: None,
                    })),
                })),
                larger: Some(Box::new(RBTreeNode {
                    key: 8,
                    value: 8.to_string(),
                    color: Color::Black,
                    smaller: Some(Box::new(RBTreeNode {
                        key: 7,
                        value: 7.to_string(),
                        color: Color::Black,
                        smaller: None,
                        larger: None,
                    })),
                    larger: Some(Box::new(RBTreeNode {
                        key: 9,
                        value: 9.to_string(),
                        color: Color::Black,
                        smaller: None,
                        larger: None,
                    })),
                })),
            })),
            larger: Some(Box::new(RBTreeNode {
                key: 20,
                value: 20.to_string(),
                color: Color::Black,
                smaller: Some(Box::new(RBTreeNode {
                    key: 15,
                    value: 15.to_string(),
                    color: Color::Black,
                    smaller: Some(Box::new(RBTreeNode {
                        key: 12,
                        value: 12.to_string(),
                        color: Color::Black,
                        smaller: None,
                        larger: None,
                    })),
                    larger: Some(Box::new(RBTreeNode {
                        key: 17,
                        value: 17.to_string(),
                        color: Color::Black,
                        smaller: None,
                        larger: None,
                    })),
                })),
                larger: Some(Box::new(RBTreeNode {
                    key: 25,
                    value: 25.to_string(),
                    color: Color::Black,
                    smaller: Some(Box::new(RBTreeNode {
                        key: 22,
                        value: 22.to_string(),
                        color: Color::Black,
                        smaller: None,
                        larger: None,
                    })),
                    larger: Some(Box::new(RBTreeNode {
                        key: 27,
                        value: 27.to_string(),
                        color: Color::Black,
                        smaller: None,
                        larger: None,
                    })),
                })),
            })),
        };

        eprintln!("{}", tree.to_string());

        assert_eq!(tree.rotate_child(RotDir::Left, Branch::Smaller), Ok(()));

        eprintln!("{}", tree.to_string());

        assert_eq!(tree.rotate_child(RotDir::Right, Branch::Smaller), Ok(()));

        eprintln!("{}", tree.to_string());
    }
}
