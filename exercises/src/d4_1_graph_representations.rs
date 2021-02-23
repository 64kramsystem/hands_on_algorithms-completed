use std::{
    cell::RefCell,
    collections::HashMap,
    hash::Hash,
    rc::{Rc, Weak},
};

type Rcc<T> = Rc<RefCell<T>>;
type Wkc<T> = Weak<RefCell<T>>;

////////////////////////////////////////////////////////////////////////////////
// Simplest; convenient if the edges matter most; slow traversal.
////////////////////////////////////////////////////////////////////////////////

pub struct EdgesListGraph<E, ID> {
    pub edges: Vec<(E, ID, ID)>,
}

////////////////////////////////////////////////////////////////////////////////
// Convenient for directed graphs.
// Edge data (<E>) is optional.
////////////////////////////////////////////////////////////////////////////////

pub struct PointerGraph<T, E> {
    // If a node is removed from here, the entry in PointerGraphNode#edges will be deallocated.
    //
    pub nodes: Vec<Rcc<PointerGraphNode<T, E>>>,
}

pub struct PointerGraphNode<T, E> {
    pub data: T,
    pub edges: Vec<(E, Wkc<PointerGraphNode<T, E>>)>,
}

////////////////////////////////////////////////////////////////////////////////
// Better performance-wise, when the size of the array is slow to iterate.
// Also, slow if there are many edges per node.
////////////////////////////////////////////////////////////////////////////////

pub struct MapGraph<T, E, ID: Hash + Eq> {
    pub nodes: HashMap<ID, T>,
    pub edges: Vec<(E, ID, ID)>,
}

////////////////////////////////////////////////////////////////////////////////
// Performant for both node and edge search.
////////////////////////////////////////////////////////////////////////////////

pub struct MapPointerGraph<T, E, ID: Hash + Eq> {
    // Node id -> edge ids
    pub nodes: HashMap<ID, (T, Vec<ID>)>,
    // Edge id -> node ids
    pub edges: HashMap<ID, (E, ID, ID)>,
}
