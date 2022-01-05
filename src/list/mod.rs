pub mod linked_list {
    pub struct LinkedList<T: PartialEq> {
        list: Link<T>,
    }

    impl<T> LinkedList<T>
    where
        T: PartialEq,
    {
        pub fn new() -> LinkedList<T> {
            LinkedList { list: Link::Nil }
        }

        pub fn push(&mut self, value: T) {
            if let Link::Node(_node) = &self.list {
                // TODO: try a direct swap
                let mut tmp = Link::Node(Box::new(Entry {
                    value: Some(value),
                    next: Link::Nil,
                }));
                std::mem::swap(&mut tmp, &mut self.list);
                if let Link::Node(node) = &mut self.list {
                    node.next = tmp;
                } else {
                    panic!("Cannot find the root I just wrote")
                }
            } else {
                self.list = Link::Node(Box::new(Entry {
                    value: Some(value),
                    next: Link::Nil,
                }))
            }
        }

        pub fn push1(&mut self, value: T) {
            let mut tmp = Link::Node(Box::new(Entry {
                value: Some(value),
                next: Link::Nil,
            }));
            std::mem::swap(&mut tmp, &mut self.list);
            if let Link::Node(next_node) = tmp {
                if let Link::Node(first_node) = &mut self.list {
                    first_node.next = Link::Node(next_node);
                } else {
                    panic!("cannot find the node I just added");
                }
            }
        }

        pub fn pop(&mut self) -> Option<T> {
            if let Link::Node(_node) = &self.list {
                let mut tmp = Link::Nil;
                std::mem::swap(&mut tmp, &mut self.list);
                if let Link::Node(node) = tmp {
                    self.list = node.next;
                    node.value
                } else {
                    panic!("Cannot find the node I just swapped out");
                }
            } else {
                None
            }
        }

        pub fn contains(&mut self, value: &T) -> bool {
            let mut curr = &self.list;
            loop {
                if let Link::Node(node) = curr {
                    if node.value.as_ref().expect("empty value encountered") == value {
                        return true;
                    } else {
                        curr = &node.next;
                    }
                } else {
                    return false;
                }
            }
        }
    }

    struct Entry<T: PartialEq> {
        // make it an option just so we can remove it
        value: Option<T>,
        next: Link<T>,
    }

    enum Link<T: PartialEq> {
        Node(Box<Entry<T>>),
        Nil,
    }
}

#[cfg(test)]
mod test {
    use super::linked_list::LinkedList;
    use crate::util::make_list_string;

    const SAMPLES: usize = 10;

    #[test]
    fn test_push_contains() {
        let values = make_list_string(SAMPLES);
        let mut list = LinkedList::new();
        for val in &values {
            list.push(val.clone());
        }
        for val in values {
            assert!(list.contains(&val));
        }
    }

    #[test]
    fn test_push_contains1() {
        let values = make_list_string(SAMPLES);
        let mut list = LinkedList::new();
        for val in &values {
            list.push1(val.clone());
        }
        for val in values {
            assert!(list.contains(&val));
        }
    }

    #[test]
    fn test_push_pop() {
        let values = make_list_string(SAMPLES);
        let mut list = LinkedList::new();
        for val in &values {
            list.push(val.clone());
        }
        for val in values.iter().rev() {
            assert_eq!(list.pop().unwrap(), *val);
            assert!(!list.contains(val));
        }
    }
}
