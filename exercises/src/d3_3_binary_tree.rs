use std::fmt::Display;

// The type bounds are implemented only where necessary, with the exception of PartialOrd, which is
// in the types, because a binary tree contains inherently orderable data.
//
pub struct BinaryTree<T: PartialOrd> {
    node: Option<Box<Node<T>>>,
}

struct Node<T: PartialOrd> {
    data: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<T: PartialOrd> BinaryTree<T> {
    pub fn new() -> BinaryTree<T> {
        BinaryTree { node: None }
    }

    pub fn add(&mut self, data: T) {
        if let Some(node) = &mut self.node {
            if data < node.data {
                node.left.add(data);
            } else {
                node.right.add(data);
            }
        } else {
            self.node = Some(Box::new(Node {
                data,
                left: BinaryTree::new(),
                right: BinaryTree::new(),
            }));
        }
    }
}

impl<T: PartialOrd + Copy> BinaryTree<T> {
    // Using an owned vector for the result is purely for convenience.
    //
    pub fn sorted_values(&self, out: Vec<T>) -> Vec<T> {
        if let Some(node) = &self.node {
            let mut out = node.left.sorted_values(out);
            out.push(node.data);
            let out = node.right.sorted_values(out);

            out
        } else {
            out
        }
    }
}

impl<T: PartialOrd + Display> BinaryTree<T> {
    pub fn print_lfirst(&self, depth: usize, buffer: String) -> String {
        if let Some(node) = &self.node {
            let mut buffer = node.left.print_lfirst(depth + 1, buffer);

            buffer.push_str(&format!("{}{}\n", &".".repeat(depth), node.data));

            let buffer = node.right.print_lfirst(depth + 1, buffer);

            buffer
        } else {
            buffer
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Binary;

    use super::*;
    use indoc::indoc;

    fn test_tree() -> BinaryTree<i32> {
        let mut tree = BinaryTree::new();

        tree.add(4);
        tree.add(5);
        tree.add(6);
        tree.add(10);
        tree.add(1);
        tree.add(94);
        tree.add(54);
        tree.add(3);

        tree
    }

    #[test]
    fn test_add() {
        let tree = test_tree();

        let actual_values = tree.sorted_values(vec![]);
        let expected_values = [1, 3, 4, 5, 6, 10, 54, 94];

        assert_eq!(actual_values, expected_values);
    }

    #[test]
    fn test_print() {
        let tree = test_tree();

        let actual_representation = tree.print_lfirst(0, String::new());
        let expected_representation = indoc! {"
            .1
            ..3
            4
            .5
            ..6
            ...10
            .....54
            ....94
        "};

        assert_eq!(actual_representation, expected_representation);
    }
}
