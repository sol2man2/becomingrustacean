fn first() {
    type Link<T> = Option<Box<Node<T>>>;

    struct Node<T> {
        elem: T,
        next: Link<T>,
    }

    pub struct LinkedList<T> {
        head: Link<T>,
    }

    pub struct IterMut<'a, T: 'a>(Option<&'a mut Node<T>>);

    impl<T> LinkedList<T> {
        fn iter_mut(&mut self) -> IterMut<T> {
            IterMut(self.head.as_mut().map(|node| &mut **node))
        }
    }

    impl<'a, T> Iterator for IterMut<'a, T> {
        type Item = &'a mut T;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.take().map(|node| {
                self.0 = node.next.as_mut().map(|node| &mut **node);
                &mut node.elem
            })
        }
    }
}

fn second() {
    use std::mem;

    pub struct IterMut<'a, T: 'a>(&'a mut [T]);

    impl<'a, T> Iterator for IterMut<'a, T> {
        type Item = &'a mut T;

        fn next(&mut self) -> Option<Self::Item> {
            let slice = mem::replace(&mut self.0, &mut []);
            if slice.is_empty() {
                return None;
            }

            let (l, r) = slice.split_at_mut(1);
            self.0 = r;
            l.get_mut(0)
        }
    }

    impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
        fn next_back(&mut self) -> Option<Self::Item> {
            let slice = mem::replace(&mut self.0, &mut []);
            if slice.is_empty() {
                return None;
            }

            let new_len = slice.len() - 1;
            let (l, r) = slice.split_at_mut(new_len);
            self.0 = l;
            r.get_mut(0)
        }
    }
}

fn third() {
    use std::collections::VecDeque;

    type Link<T> = Option<Box<Node<T>>>;

    struct Node<T> {
        elem: T,
        left: Link<T>,
        right: Link<T>,
    }

    pub struct Tree<T> {
        root: Link<T>,
    }

    struct NodeIterMut<'a, T: 'a> {
        elem: Option<&'a mut T>,
        left: Option<&'a mut Node<T>>,
        right: Option<&'a mut Node<T>>,
    }

    enum State<'a, T: 'a> {
        Elem(&'a mut T),
        Node(&'a mut Node<T>),
    }

    pub struct IterMut<'a, T: 'a>(VecDeque<NodeIterMut<'a, T>>);

    impl<T> Tree<T> {
        pub fn iter_mut(&mut self) -> IterMut<T> {
            let mut deque = VecDeque::new();
            self.root
                .as_mut()
                .map(|root| deque.push_front(root.iter_mut()));
            IterMut(deque)
        }
    }

    impl<T> Node<T> {
        pub fn iter_mut(&mut self) -> NodeIterMut<T> {
            NodeIterMut {
                elem: Some(&mut self.elem),
                left: self.left.as_mut().map(|node| &mut **node),
                right: self.right.as_mut().map(|node| &mut **node),
            }
        }
    }

    impl<'a, T> Iterator for NodeIterMut<'a, T> {
        type Item = State<'a, T>;

        fn next(&mut self) -> Option<Self::Item> {
            match self.left.take() {
                Some(node) => Some(State::Node(node)),
                None => match self.elem.take() {
                    Some(elem) => Some(State::Elem(elem)),
                    None => match self.right.take() {
                        Some(node) => Some(State::Node(node)),
                        None => None,
                    },
                },
            }
        }
    }

    impl<'a, T> DoubleEndedIterator for NodeIterMut<'a, T> {
        fn next_back(&mut self) -> Option<Self::Item> {
            match self.right.take() {
                Some(node) => Some(State::Node(node)),
                None => match self.elem.take() {
                    Some(elem) => Some(State::Elem(elem)),
                    None => match self.left.take() {
                        Some(node) => Some(State::Node(node)),
                        None => None,
                    },
                },
            }
        }
    }

    impl<'a, T> Iterator for IterMut<'a, T> {
        type Item = &'a mut T;
        fn next(&mut self) -> Option<Self::Item> {
            loop {
                match self.0.front_mut().and_then(|node_it| node_it.next()) {
                    Some(State::Elem(elem)) => return Some(elem),
                    Some(State::Node(node)) => self.0.push_front(node.iter_mut()),
                    None => {
                        if let None = self.0.pop_front() {
                            return None;
                        }
                    }
                }
            }
        }
    }

    impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
        fn next_back(&mut self) -> Option<Self::Item> {
            loop {
                match self.0.back_mut().and_then(|node_it| node_it.next_back()) {
                    Some(State::Elem(elem)) => return Some(elem),
                    Some(State::Node(node)) => self.0.push_back(node.iter_mut()),
                    None => {
                        if let None = self.0.pop_back() {
                            return None;
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    println!("ch3-11!");
    first();
    second();
    third();
}
