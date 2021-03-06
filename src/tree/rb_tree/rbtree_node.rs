use super::SubNode;
use crate::tree::rb_tree::rbtree_node::Color::Red;
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
        /*eprintln!(
            "({:?}).insert_node_rb(key: {:?}) root: {}",
            self.key, node.key, is_root
        );*/
        if node.key == self.key {
            //eprintln!("insert_node_rb() update");
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
            /* eprintln!(
                "({:?}).insert_node_rb() root: {} insert into subnode returned insert_state {:?}",
                self.key, is_root, ins_state
            );
            */
            match ins_state {
                InsertState::Clean => (res, ins_state),
                InsertState::ChgdColor => {
                    if is_root {
                        (res, InsertState::Clean)
                    } else {
                        // subnode has changed color to red
                        if self.color == Color::Red {
                            // tell my parent that me and my sibling are red
                            (res, InsertState::Conflict)
                        } else {
                            (res, InsertState::Clean)
                        }
                    }
                }
                InsertState::Conflict => {
                    // child reports that it and its child are red
                    if let Some(true) = uncle_link.as_mut().map(|node| node.color == Color::Red) {
                        // uncle is red
                        if !is_root {
                            self.color = Color::Red;
                        }
                        if let Some(uncle) = uncle_link.as_mut() {
                            uncle.color = Color::Black
                        }
                        if let Some(child) = child_link.as_mut() {
                            child.color = Color::Black
                        }

                        (
                            res,
                            if is_root {
                                InsertState::Clean
                            } else {
                                InsertState::ChgdColor
                            },
                        )
                    } else {
                        match branch {
                            Branch::Smaller => (res, InsertState::RightRotate),
                            Branch::Larger => (res, InsertState::LeftRotate),
                        }
                    }
                }
                InsertState::LeftRotate => match self.rotate_child(RotDir::Left, branch) {
                    Ok(_) => (res, InsertState::Clean),
                    Err(err) => panic!("{}", err),
                },
                InsertState::RightRotate => match self.rotate_child(RotDir::Right, branch) {
                    Ok(_) => (res, InsertState::Clean),
                    Err(err) => panic!("{}", err),
                },
            }
        } else {
            assert_eq!(node.color, Color::Red);
            *child_link = Some(node);
            (
                None,
                if self.color == Color::Black {
                    InsertState::Clean
                } else {
                    InsertState::Conflict
                },
            )
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

    /// rotate left - larger side moves up
    /// rotate anti clockwise (in this notation)
    ///    (self)              (self)
    ///     ??????<(?)              ??????<(?)
    ///     ??????>(c)              ??????>(l)
    ///         ??????<(l)            ??????<(l,l)
    ///         ???   ??????<(l,l)      ??????>(c)
    ///         ???   ??????>(l,s)          ??????<(l,s)
    ///         ??????>(s)                ??????>(s)
    ///             ??????<(s,l)              ??????<(s,l)
    ///             ??????>(s,s)              ??????>(s,s)

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
    ///     ??????<(l)            ??????<(l,l)
    ///     ???   ??????<(l,l)      ??????>(self)
    ///     ???   ??????>(l,s)          ??????<(l,s)
    ///     ??????>(s)                ??????>(s)
    ///         ??????<(s,l)              ??????<(s,l)
    ///         ??????>(s,s)              ??????>(s,s)

    pub fn left_rotate(
        mut self: Box<Self>,
    ) -> std::result::Result<Box<Self>, (Box<Self>, &'static str)> {
        if self.larger.is_some() {
            let mut larger = self.larger.take().expect("unexpected empty link");
            self.color = Color::Red;
            let mut new_root =
                if let Some(true) = larger.smaller.as_ref().map(|node| node.color == Color::Red) {
                    //  do a modified right rotate on larger.larger, larger, larger.smaller
                    let mut smaller_gc = larger.smaller.take().expect("unexpected empty link");
                    larger.smaller = smaller_gc.larger.take();
                    smaller_gc.larger = Some(larger);
                    smaller_gc
                } else {
                    larger
                };
            // now left rotate larger smaller_gc self

            self.larger = new_root.smaller.take();
            new_root.smaller = Some(self);
            new_root.color = Color::Black;
            Ok(new_root)
        } else {
            Err((self, "cannot left rotate - larger subnode is nil"))
        }
    }

    /// rotate right - smaller side moves up
    /// rotate clockwise (in this notation)
    ///    (self)           (s)
    ///     ??????<(l)           ??????<(self)
    ///     ???   ??????<(l,l)     ???   ??????<(l)
    ///     ???   ??????>(l,s)     ???   ???   ??????<(l,l)
    ///     ??????>(s)           ???   ???   ??????>(l,s)     
    ///         ??????<(s,l)     ???   ??????>(s,l)     
    ///         ??????>(s,s)     ??????>(s,s)          

    pub fn right_rotate(
        mut self: Box<Self>,
    ) -> std::result::Result<Box<Self>, (Box<Self>, &'static str)> {
        if self.smaller.is_some() {
            let mut smaller = self.smaller.take().expect("unexpected empty link");
            self.color = Color::Red;
            let mut new_root =
                if let Some(true) = smaller.larger.as_ref().map(|node| node.color == Color::Red) {
                    //  do a modified left rotate on smaller.larger, larger, larger.smaller
                    let mut larger_gc = smaller.larger.take().expect("unexpected empty link");
                    smaller.larger = larger_gc.smaller.take();
                    larger_gc.smaller = Some(smaller);
                    larger_gc
                } else {
                    smaller
                };
            // now left rotate larger smaller_gc self

            self.smaller = new_root.larger.take();
            new_root.larger = Some(self);
            new_root.color = Color::Black;
            Ok(new_root)
        } else {
            Err((self, "cannot right rotate - smaller subnode is nil"))
        }
    }
}

impl<K: PartialOrd + Debug, V: Debug> RBTreeNode<K, V> {
    /// format as:
    /// (kkk,vvv)
    ///   ??????<(kkk,vvv)
    ///   ???   ??????<(kkk,vvv)
    ///   ???   ??????>nil
    ///   ??????>(kkk,vvv)
    ///       ??????<(kkk,vvv)
    ///       ??????>(kkk,vvv)

    fn to_str_buffer(&self, buffer: &mut String, lead: &str, root: bool, smaller: bool) {
        const J_SMALLER: &str = " ??????>";
        const J_LARGER: &str = " ??????<";
        const L_SMALLER: &str = "    ";
        const L_LARGER: &str = " ???  ";

        // buffer.push_str(&format!("lead: '{}'\n", lead));

        let node_str = format!(
            "{}({:?},{:?})",
            if self.color == Color::Red { "R" } else { "B" },
            self.key,
            self.value
        );
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

    pub fn check_rules(
        &self,
        is_root: bool,
        parent_red: bool,
    ) -> std::result::Result<usize, String> {
        if is_root {
            // this is root
            if let Color::Red = self.color {
                return Err("RB violation: root is red".to_string());
            }
        }

        if (self.color == Red) && parent_red {
            return Err(format!(
                "RB violation: two successive red nodes @{:?} and parent",
                self.key
            ));
        }

        let black_count_sm = if let Some(node) = &self.smaller {
            node.check_rules(false, self.color == Color::Red)?
        } else {
            0
        };

        let black_count_lg = if let Some(node) = &self.larger {
            node.check_rules(false, self.color == Color::Red)?
        } else {
            0
        };

        if black_count_sm != black_count_lg {
            return Err(format!(
                "RB violation: mismatching black counts @{:?} {}!={}",
                self.key, black_count_sm, black_count_lg
            ));
        }

        Ok(black_count_sm + if self.color == Color::Black { 1 } else { 0 })
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
