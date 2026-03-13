use serde::{Deserialize, Serialize};

/// A single node in the B+ tree, either an internal node or a leaf node.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Node<V> {
    Internal(InternalNode<V>),
    Leaf(LeafNode<V>),
}

/// Internal node of the B+ tree.
///
/// `keys` partition the key space into ranges, each handled by one child:
/// - `children[0]` contains all keys `< keys[0]`
/// - `children[i]` contains all keys `>= keys[i - 1]` and `< keys[i]`
/// - `children[last]` contains all keys `>= keys[last_key]`
///
/// `keys` is always sorted in ascending order.
/// `children` holds boxed nodes so they can be stored on the heap and shared by the tree.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InternalNode<V> {
    pub keys: Vec<KeySize>,
    pub children: Vec<Box<Node<V>>>,
}

/// Leaf node of the B+ tree.
///
/// Stores key–value pairs in two parallel vectors. The index links keys and values:
/// `keys[i]` is the key for `values[i]`.
/// `keys` is kept sorted so lookups and inserts can use binary / positional search.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeafNode<V> {
    pub values: Vec<V>,
    pub keys: Vec<KeySize>,
}

pub type KeySize = u32;

/// Result of a delete operation
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DeleteResult<K> {
    NotFound,
    Ok,
    MinChanged { new_min: K },
    Underflow { new_min_opt: Option<K> },
    Empty, // only happens when no elements are left in the leaf, and no borrowing was possible in
           // earlier iterations
}