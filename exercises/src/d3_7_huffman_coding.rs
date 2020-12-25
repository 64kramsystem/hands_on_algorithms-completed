use std::{collections::BTreeMap, fmt::Write};

// In real world, this would be best exported as HuffmanTree.
pub enum Node {
    Tree(Box<Node>, Box<Node>),
    Leaf(char),
}

struct Score {
    node: Node,
    score: u32,
}

impl Node {
    pub fn build_tree(input: &str) -> Node {
        // Fun stuff: if we use a HashMap, we're going to get a non-deterministic result!
        //
        let mut scores_map = BTreeMap::new();

        for token in input.chars() {
            let token_entry = scores_map.entry(token).or_insert(0);
            *token_entry += 1;
        }

        let mut scores_list = scores_map
            .into_iter()
            .map(|(token, score)| Score {
                node: Node::Leaf(token),
                score: score,
            })
            .collect::<Vec<_>>();

        for last in (1..scores_list.len()).rev() {
            let beforelast = last - 1;

            for i in 0..beforelast {
                // It's possible to save one comparison, by checking if the score is lowest than the last element,
                // then we move the last two elements, else, if the score is lowest than the beforelast, then
                // we move only that.
                //
                if scores_list[i].score < scores_list[beforelast].score {
                    scores_list.swap(i, beforelast);
                }
                if scores_list[beforelast].score < scores_list[last].score {
                    scores_list.swap(beforelast, last);
                }
            }

            let left_node = scores_list.pop().unwrap();
            let right_node = scores_list.pop().unwrap();

            let new_node = Node::Tree(Box::new(left_node.node), Box::new(right_node.node));

            scores_list.push(Score {
                node: new_node,
                score: left_node.score + right_node.score,
            });
        }

        scores_list.pop().unwrap().node
    }

    pub fn print<U: Write>(&self, depth: usize, direction: char, mut writer: U) -> U {
        let spacing = String::from(".").repeat(depth);

        match self {
            Node::Tree(left, right) => {
                writer = left.print(depth + 1, '╱', writer);
                writeln!(writer, "{}{}*", spacing, direction).unwrap();
                right.print(depth + 1, '╲', writer)
            }
            Node::Leaf(token) => {
                writeln!(writer, "{}{}{}", spacing, direction, token).unwrap();
                writer
            }
        }
    }

    pub fn encode(&self, input: &str) -> String {
        let mut output = Vec::new();

        for token in input.chars() {
            let encoded_token = self.encode_token(token, 0).unwrap();
            output.extend(encoded_token);
        }

        output.into_iter().collect()
    }

    pub(crate) fn encode_token(&self, token: char, depth: usize) -> Option<Vec<&str>> {
        match self {
            Node::Tree(left, right) => {
                if let Some(mut buffer) = left.encode_token(token, depth + 1) {
                    buffer[depth] = "0";
                    Some(buffer)
                } else if let Some(mut buffer) = right.encode_token(token, depth + 1) {
                    buffer[depth] = "1";
                    Some(buffer)
                } else {
                    None
                }
            }
            Node::Leaf(node_token) => {
                if token == *node_token {
                    Some(vec![""; depth])
                } else {
                    None
                }
            }
        }
    }

    pub fn decode(&self, input: &str) -> String {
        let mut output = String::new();

        let mut current_node = self;

        for direction_bit in input.chars() {
            current_node = match current_node {
                Node::Tree(left, right) => match direction_bit {
                    '0' => left.as_ref(),
                    '1' => right.as_ref(),
                    _ => panic!("Unexpected bit: {}", direction_bit),
                },
                Node::Leaf(_) => {
                    panic!()
                }
            };

            if let Node::Leaf(token) = current_node {
                output.push(*token);
                current_node = self;
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_build_tree() {
        let tree = Node::build_tree("at an apple app");

        let actual_representation = tree.print(0, '<', String::from(""));

        let expected_representation = indoc! {"
            ..╱a
            .╱*
            ...╱ 
            ..╲*
            .....╱n
            ....╱*
            .....╲l
            ...╲*
            .....╱t
            ....╲*
            .....╲e
            <*
            .╲p
        "};

        assert_eq!(actual_representation, expected_representation);
    }

    #[test]
    fn test_encode_token() {
        let tree = Node::build_tree("at an apple app");

        let actual_representation = tree.encode_token('n', 0).unwrap().join("");

        let expected_representation = "01100";

        assert_eq!(actual_representation, expected_representation);
    }

    #[test]
    fn test_encode_sequence() {
        let input = "at an apple app";

        let tree = Node::build_tree(input);

        let actual_output = tree.encode(input);

        let expected_output = "00011100100001100010001101101011110100011";

        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_decode_sequence() {
        let decoded_sequence = "at an apple app";
        let encoded_sequence = "00011100100001100010001101101011110100011";

        let tree = Node::build_tree(decoded_sequence.clone());

        let actual_decoded_sequence = tree.decode(encoded_sequence);

        assert_eq!(actual_decoded_sequence, decoded_sequence);
    }
}
