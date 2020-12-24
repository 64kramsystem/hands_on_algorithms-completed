use std::{
    cell::RefCell,
    fmt::{Display, Write},
    rc::Rc,
};

type Rcc<T> = Rc<RefCell<T>>;

pub struct Node<T: PartialOrd> {
    right: Option<Rcc<Node<T>>>,
    down: Option<Rcc<Node<T>>>,
    data: Rc<T>,
}

fn rcc<T: PartialOrd>(node: Node<T>) -> Rcc<Node<T>> {
    Rc::new(RefCell::new(node))
}

impl<T: PartialOrd> Node<T> {
    pub fn new(data: Rc<T>, right: Option<Rcc<Node<T>>>, down: Option<Rcc<Node<T>>>) -> Self {
        Node { right, down, data }
    }

    pub fn insert(&mut self, data: T) -> Option<Rcc<Node<T>>> {
        // If there is a child on the right, and the data is greater than it, recursively insert on
        // the right.
        //
        if let Some(right) = &self.right {
            let mut right = right.borrow_mut();

            if data > *right.data {
                return right.insert(data);
            }
        }

        if let Some(down) = &self.down {
            let inserted_node = down.borrow_mut().insert(data);

            if let Some(inserted_node) = inserted_node {
                if rand::random() {
                    let data = &inserted_node.borrow().data;
                    let down = Some(Rc::clone(&inserted_node));

                    return self.insert_to_right(Rc::clone(data), down);
                }
            }

            return None;
        }

        self.insert_to_right(Rc::new(data), None)
    }

    // Returns the new node.
    //
    fn insert_to_right(&mut self, data: Rc<T>, down: Option<Rcc<Node<T>>>) -> Option<Rcc<Node<T>>> {
        let extracted_right = self.right.take();
        let new_node = rcc(Node::new(data, extracted_right, down));
        self.right = Some(Rc::clone(&new_node));
        Some(new_node)
    }
}

impl<T: PartialOrd + Display> Node<T> {
    pub fn print<U: Write>(&self, mut writer: U) -> U {
        write!(writer, " {}", self.data).unwrap();

        if let Some(right) = &self.right {
            right.borrow().print(writer)
        } else {
            writer
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_node() -> Node<i32> {
        let mut list = Node::new(Rc::new(4), None, None);

        list.insert(4);
        list.insert(6);
        list.insert(77);
        list.insert(84);
        list.insert(23);
        list.insert(1);

        list
    }

    #[test]
    fn test_insert() {
        let node = test_node();

        let actual_representation = node.print(String::from(""));
        let expected_representation = " 4 1 4 6 23 77 84";

        assert_eq!(actual_representation, expected_representation);
    }
}
