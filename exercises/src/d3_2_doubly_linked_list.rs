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

            // Alternative: `*current_front = node;`
            //
            self.0 = Some((node, current_back.clone()));
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

    fn push_back(&mut self, value: T) {
        if let Some((ref mut current_front_rc, ref mut current_back_wk)) = self.0 {
            let node = Rc::new(RefCell::new(Node {
                value,
                next: None,
                previous: Some(current_back_wk.clone()),
            }));

            let current_back_rc = Weak::upgrade(current_back_wk).unwrap();
            current_back_rc.borrow_mut().next = Some(node.clone());

            *current_back_wk = Rc::downgrade(&node);
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

    fn pop_front(&mut self) -> Option<T> {
        if let Some((ref mut current_front_rc, ref mut current_back_wk)) = self.0 {
            let popped_value = current_front_rc.borrow().value;

            let new_ends = if let Some(ref mut child_rc) = current_front_rc.borrow_mut().next {
                child_rc.borrow_mut().previous = None;
                Some((Rc::clone(child_rc), Weak::clone(current_back_wk)))
            } else {
                None
            };

            self.0 = new_ends;

            Some(popped_value)
        } else {
            None
        }
    }

    fn pop_back(&mut self) -> Option<T> {
        if let Some((ref mut current_front_rc, ref mut current_back_wk)) = self.0 {
            let current_back_rc = Weak::upgrade(current_back_wk).unwrap();
            let popped_value = current_back_rc.borrow().value;

            let new_ends = if let Some(ref mut parent_wk) = current_back_rc.borrow_mut().previous {
                let parent_rc = Weak::upgrade(parent_wk).unwrap();
                parent_rc.borrow_mut().next = None;
                Some((Rc::clone(current_front_rc), Weak::clone(parent_wk)))
            } else {
                None
            };

            self.0 = new_ends;

            Some(popped_value)
        } else {
            None
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

    // LinkedList#values() starts from the front reference, so we may also test the back reference.
    // The tests are incomplete - values() goes only in one direction, so the `previous` reference is
    // not tested; since this is an exercise, thorough testing is not expected.

    #[test]
    fn test_push_front() {
        let values = vec![1273, 18273, 8273, 827, 92900];

        let mut list = LinkedList::new();

        for value in values.iter().rev() {
            list.push_front(*value);
        }

        assert_eq!(list.values()[..], values[..]);

        let (_, ref list_back) = list.0.unwrap();

        let list_back_value = Weak::upgrade(list_back).unwrap().borrow().value;

        assert_eq!(list_back_value, *values.last().unwrap());
    }

    #[test]
    fn test_push_back() {
        let values = vec![1273, 18273, 8273, 827, 92900];

        let mut list = LinkedList::new();

        for value in &values {
            list.push_back(*value);
        }

        assert_eq!(list.values()[..], values[..]);

        let (_, ref list_back) = list.0.unwrap();

        let list_back_value = Weak::upgrade(list_back).unwrap().borrow().value;

        assert_eq!(list_back_value, *values.last().unwrap());
    }

    #[test]
    fn test_pop_front() {
        let values = vec![1273, 18273, 8273, 827, 92900];

        let mut list = LinkedList::new();

        for value in &values {
            list.push_back(*value);
        }

        for value in &values {
            let popped_value = list.pop_front();

            assert_eq!(popped_value.unwrap(), *value);
        }

        assert!(list.pop_front().is_none());
    }

    #[test]
    fn test_pop_back() {
        let values = vec![1273, 18273, 8273, 827, 92900];

        let mut list = LinkedList::new();

        for value in &values {
            list.push_back(*value);
        }

        for value in values.iter().rev() {
            let popped_value = list.pop_back();

            assert_eq!(popped_value.unwrap(), *value);
        }

        assert!(list.pop_back().is_none());
    }
}
