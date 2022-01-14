pub mod linked_list {
    type Link<T> = Option<Box<Entry<T>>>;

    struct Entry<T> {
        value: T,
        next: Link<T>,
    }

    pub struct LinkedList<T> {
        list: Link<T>,
    }

    impl<T> Default for LinkedList<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T> LinkedList<T> {
        pub fn new() -> Self {
            LinkedList { list: None }
        }

        pub fn push(&mut self, value: T) {
            let new_node = Some(Box::new(Entry {
                value,
                next: self.list.take(),
            }));
            self.list = new_node;
        }

        pub fn pop(&mut self) -> Option<T> {
            self.list.take().map(|node| {
                self.list = node.next;
                node.value
            })
        }

        pub fn peek(&self) -> Option<&T> {
            self.list.as_ref().map(|node| &node.value)
        }

        pub fn peek_mut(&mut self) -> Option<&mut T> {
            self.list.as_mut().map(|node| &mut node.value)
        }
    }

    impl<T: PartialEq> LinkedList<T> {
        pub fn contains(&mut self, value: &T) -> bool {
            let mut curr = &self.list;
            while let Some(node) = curr {
                if node.value == *value {
                    return true;
                } else {
                    curr = &node.next;
                }
            }
            false
        }
    }

    impl<T> Drop for LinkedList<T> {
        fn drop(&mut self) {
            let mut curr = self.list.take();
            while let Some(node) = &mut curr {
                curr = node.next.take();
            }
        }
    }

    pub struct IntoIter<T>(LinkedList<T>);

    impl<T> IntoIterator for LinkedList<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;

        fn into_iter(self) -> IntoIter<T> {
            IntoIter(self)
        }
    }

    impl<T> Iterator for IntoIter<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.pop()
        }
    }

    pub struct Iter<'a, T> {
        next: Option<&'a Entry<T>>,
    }

    impl<T> LinkedList<T> {
        pub fn iter(&self) -> Iter<T> {
            Iter {
                next: self.list.as_deref(),
            }
        }
    }

    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            self.next.map(|node| {
                self.next = node.next.as_deref();
                &node.value
            })
        }
    }

    pub struct IterMut<'a, T> {
        next: Option<&'a mut Entry<T>>,
    }

    impl<T> LinkedList<T> {
        pub fn iter_mut(&mut self) -> IterMut<T> {
            IterMut {
                next: self.list.as_deref_mut(),
            }
        }
    }

    impl<'a, T> Iterator for IterMut<'a, T> {
        type Item = &'a mut T;

        fn next(&mut self) -> Option<Self::Item> {
            self.next.take().map(|node| {
                self.next = node.next.as_deref_mut();
                &mut node.value
            })
        }
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

    #[test]
    fn test_peek() {
        let values = make_list_string(SAMPLES);
        let mut list = LinkedList::new();
        assert_eq!(list.peek(), None);
        for val in &values {
            list.push(val.clone());
            assert_eq!(list.peek(), Some(val));
        }
    }

    #[test]
    fn test_peek_mut() {
        let values = make_list_string(SAMPLES);
        let mut list = LinkedList::new();
        assert_eq!(list.peek_mut(), None);
        for val in &values {
            list.push(val.clone());
            list.peek_mut().unwrap().push('-');
            assert_eq!(list.peek().unwrap(), &(val.clone() + "-"));
        }
    }

    #[test]
    fn test_into_iter() {
        let values = make_list_string(SAMPLES);
        let mut list = LinkedList::new();
        for val in &values {
            list.push(val.clone());
        }

        let vector: Vec<String> = list.into_iter().collect();
        for (left, right) in vector.iter().rev().zip(values.iter()) {
            assert_eq!(left, right);
        }
    }

    #[test]
    fn test_iter() {
        let values = make_list_string(SAMPLES);
        let mut list = LinkedList::new();
        for val in &values {
            list.push(val.clone());
        }

        for (left, right) in list.iter().zip(values.iter().rev()) {
            assert_eq!(left, right);
        }
    }

    #[test]
    fn test_iter_mut() {
        let mut values = make_list_string(SAMPLES);
        let mut list = LinkedList::new();
        for val in &values {
            list.push(val.clone());
        }

        for (left, right) in list
            .iter_mut()
            .map(|item| {
                *item = format!("-{}", item);
                item
            })
            .zip(
                values
                    .iter_mut()
                    .map(|val| {
                        *val = format!("-{}", val);
                        val
                    })
                    .rev(),
            )
        {
            // eprintln!("{:?} == {:?}", left, right);
            assert_eq!(left, right);
        }
    }
}
