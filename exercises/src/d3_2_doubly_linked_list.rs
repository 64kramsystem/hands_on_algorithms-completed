#![allow(unused_variables)]

use std::{cell::RefCell, rc::Rc, rc::Weak};

type StrongNodeRef<T> = Rc<RefCell<Node<T>>>;
type WeakNodeRef<T> = Weak<RefCell<Node<T>>>;

struct LinkedList<T: Copy>(Option<(StrongNodeRef<T>, WeakNodeRef<T>)>);

struct Node<T: Copy> {
    value: T,
    next: Option<StrongNodeRef<T>>,
    previous: Option<WeakNodeRef<T>>,
}

impl<T: Copy> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList(None)
    }

    fn push_front(&mut self, value: T) {
        if let Some((ref mut current_front, ref mut current_back)) = self.0 {
            let node = Rc::new(RefCell::new(Node {
                value,
                next: Some(current_front.clone()),
                previous: None,
            }));

            current_front.borrow_mut().previous = Some(Rc::downgrade(&node));

            let new_front = node;

            self.0 = Some((new_front, current_back.clone()));
        } else {
            let node = Rc::new(RefCell::new(Node {
                value,
                next: None,
                previous: None,
            }));

            let front = node;
            let back = Rc::downgrade(&front);

            self.0 = Some((front, back));
        }
    }

    pub fn values(&self) -> Vec<T> {
        let mut values = vec![];

        if let Some((ref front, _)) = self.0 {
            let mut current_opt = Some(front.clone());

            // See https://stackoverflow.com/questions/36597987/cyclic-reference-of-refcell-borrows-in-traversal
            //
            while let Some(current) = current_opt {
                let current_node = current.borrow();

                let current_value = current_node.value;
                values.push(current_value);

                current_opt = current_node.next.clone();
            }
        }

        values
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_front() {
        let values = vec![1273, 18273, 8273, 827, 92900];

        let mut list = LinkedList::new();

        for value in values.iter().rev() {
            list.push_front(*value);
        }

        assert_eq!(list.values()[..], values[..]);
    }
}
