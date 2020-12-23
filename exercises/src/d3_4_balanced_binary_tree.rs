use std::{
    cmp,
    fmt::{Binary, Display},
};

// The type bounds are implemented only where necessary, with the exception of PartialOrd, which is
// in the types, because a binary tree contains inherently orderable data.
//
pub struct BinaryTree<T: PartialOrd> {
    node: Option<Box<Node<T>>>,
}

struct Node<T: PartialOrd> {
    data: T,
    height: i8,
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

            node.compute_and_set_height();
        } else {
            self.node = Some(Box::new(Node {
                data,
                height: 1,
                left: BinaryTree::new(),
                right: BinaryTree::new(),
            }));
        }
    }

    pub fn height(&self) -> i8 {
        if let Some(node) = &self.node {
            node.height
        } else {
            0
        }
    }
}

impl<T: PartialOrd> BinaryTree<T> {
    // Using an owned vector for the result is purely for convenience.
    //
    // It's not clear if this lifetimes usage is correct. The idea is that the lifetimes of self and
    // the vector match.
    // However, it'd be more correct to specify that the lifetime of self may outlive the vector; this
    // is possibly expressed by:
    //
    //   sorted_values<'a: 'b, 'b>(&'b self, out: Vec<&'a T>) -> Vec<&'a T>
    //
    // however, I can't find what's the name of this syntax, and if it expresses what intended.
    //
    pub fn sorted_values<'a>(&'a self, out: Vec<&'a T>) -> Vec<&'a T> {
        if let Some(node) = &self.node {
            let mut out = node.left.sorted_values(out);
            out.push(&node.data);
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

            buffer.push_str(&format!(
                "{}:{}{}\n",
                node.height,
                &".".repeat(depth),
                node.data
            ));

            let buffer = node.right.print_lfirst(depth + 1, buffer);

            buffer
        } else {
            buffer
        }
    }
}

impl<T: PartialOrd> Node<T> {
    pub fn compute_and_set_height(&mut self) {
        self.height = 1 + cmp::max(self.left.height(), self.right.height());
    }
}

impl<T: PartialOrd> BinaryTree<T> {
    // The course algorithm redundantly sets B height; since its children are not changed, there is
    // no change in height.
    //
    pub fn rotate_left(&mut self) {
        // See https://en.wikipedia.org/wiki/Tree_rotation for a diagram of the rotation, along with
        // the letters.

        // Extract the root as node, temporarily leaving the tree (self) empty.
        //
        let mut p = self
            .node
            .take()
            .expect("The root node doesn't have children!");

        // Detach Q (right node)
        //
        let mut q = p.right.node.expect("No right node found while rotating!");

        // Attach B to P (detachment is in the next step)

        let b = q.left;
        p.right = b;

        p.compute_and_set_height();

        // Attach P to Q; this implicitly performs the detachment mentioned above.
        //
        q.left = BinaryTree { node: Some(p) };

        q.compute_and_set_height();

        // Set Q as the new root node.

        self.node.replace(q);
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
        let expected_values = [&1, &3, &4, &5, &6, &10, &54, &94];

        assert_eq!(actual_values, expected_values);
    }

    #[test]
    fn test_print() {
        let tree = test_tree();

        let actual_representation = tree.print_lfirst(0, String::new());
        let expected_representation = indoc! {"
            2:.1
            1:..3
            6:4
            5:.5
            4:..6
            3:...10
            1:.....54
            2:....94
        "};

        assert_eq!(actual_representation, expected_representation);
    }

    #[test]
    fn test_rotate_left() {
        let mut tree = test_tree();

        tree.rotate_left();

        let actual_representation = tree.print_lfirst(0, String::new());

        let expected_representation = indoc! {"
            2:..1
            1:...3
            3:.4
            5:5
            4:.6
            3:..10
            1:....54
            2:...94
        "};

        assert_eq!(actual_representation, expected_representation);
    }
}
