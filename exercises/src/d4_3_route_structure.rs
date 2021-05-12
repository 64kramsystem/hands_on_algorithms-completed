use std::{fmt, rc::Rc};

// use std::{collections::HashMap, hash::Hash};

trait Weight {
    fn weight(&self) -> i32;
}

impl Weight for i32 {
    fn weight(&self) -> i32 {
        *self
    }
}

pub struct Route<ID> {
    position: ID,
    // Since we don't modify a route, we don't need Rcc.
    path: Option<Rc<Route<ID>>>,
    // Optional.
    length: u32,
}

impl<ID: Eq> Route<ID> {
    pub fn start_rc(position: ID) -> Rc<Self> {
        Rc::new(Self {
            position,
            path: None,
            length: 0,
        })
    }

    pub fn contains(&self, node: &ID) -> bool {
        if self.position == *node {
            true
        } else if let Some(path) = &self.path {
            path.contains(node)
        } else {
            false
        }
    }
}

impl<ID: fmt::Debug> fmt::Display for Route<ID> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(path) = &self.path {
            write!(f, "{}-{}-", path, self.length)?;
        }
        write!(f, "{:?}", self.position)
    }
}

// #[derive(Debug)]
// pub struct Graph<T, E, ID: Hash + Eq> {
//     pub data: HashMap<ID, (T, Vec<ID>)>,
//     pub edges: HashMap<ID, (E, ID, ID)>,
// }
//
// impl<T, E, ID: Clone + Hash + Eq> Graph<T, E, ID> {
//     pub fn new() -> Self {
//         Graph {
//             data: HashMap::new(),
//             edges: HashMap::new(),
//         }
//     }
//
//     pub fn add_node(&mut self, id: ID, datum: T) {
//         // Edges are inserted separately, since they require nodes.
//         //
//         self.data.insert(id, (datum, vec![]));
//     }
//
//     // For simplicity, assume that the node exists.
//     //
//     pub fn add_edge(&mut self, edge_id: ID, from_id: ID, to_id: ID, datum: E) {
//         self.edges
//             .insert(edge_id.clone(), (datum, from_id.clone(), to_id.clone()));
//
//         let from = self.data.get_mut(&from_id).unwrap();
//         from.1.push(edge_id.clone());
//
//         let to = self.data.get_mut(&to_id).unwrap();
//         to.1.push(edge_id);
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     // use indoc::indoc;
//
//     #[test]
//     fn test_pizza() {
//         let mut graph = Graph::new();
//
//         for x in vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'] {
//             graph.add_node(x, ());
//         }
//
//         graph.add_edge('a', 'H', 'D', 6);
//         graph.add_edge('b', 'D', 'C', 18);
//         graph.add_edge('c', 'C', 'B', 10);
//         graph.add_edge('d', 'H', 'A', 7);
//         graph.add_edge('e', 'A', 'C', 4);
//         graph.add_edge('f', 'H', 'G', 5);
//         graph.add_edge('g', 'G', 'A', 8);
//         graph.add_edge('h', 'A', 'F', 3);
//         graph.add_edge('i', 'F', 'E', 15);
//         graph.add_edge('j', 'C', 'E', 12);
//
//         assert_eq!(graph.data[&'A'], ((), vec!['d', 'e', 'g', 'h']));
//         assert_eq!(graph.data[&'A'], ((), vec!['d', 'e', 'g', 'h']));
//         assert_eq!(graph.data[&'B'], ((), vec!['c']));
//         assert_eq!(graph.data[&'C'], ((), vec!['b', 'c', 'e', 'j']));
//         assert_eq!(graph.data[&'D'], ((), vec!['a', 'b']));
//         assert_eq!(graph.data[&'E'], ((), vec!['i', 'j']));
//         assert_eq!(graph.data[&'F'], ((), vec!['h', 'i']));
//         assert_eq!(graph.data[&'G'], ((), vec!['f', 'g']));
//         assert_eq!(graph.data[&'H'], ((), vec!['a', 'd', 'f']));
//
//         assert_eq!(graph.edges[&'a'], (6, 'H', 'D'));
//         assert_eq!(graph.edges[&'b'], (18, 'D', 'C'));
//         assert_eq!(graph.edges[&'c'], (10, 'C', 'B'));
//         assert_eq!(graph.edges[&'d'], (7, 'H', 'A'));
//         assert_eq!(graph.edges[&'e'], (4, 'A', 'C'));
//         assert_eq!(graph.edges[&'f'], (5, 'H', 'G'));
//         assert_eq!(graph.edges[&'g'], (8, 'G', 'A'));
//         assert_eq!(graph.edges[&'h'], (3, 'A', 'F'));
//         assert_eq!(graph.edges[&'i'], (15, 'F', 'E'));
//         assert_eq!(graph.edges[&'j'], (12, 'C', 'E'));
//     }
// }
