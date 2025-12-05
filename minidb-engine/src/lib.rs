pub mod models;

use crate::models::{BPlusTree, InternalNode, LeafNode, Node};
use rand::random;
use std::vec;

impl<V> BPlusTree<V>
where
    V: std::fmt::Debug + Clone,
{
    pub fn new(order: usize, root: Node<V>) -> Self {
        BPlusTree { order, root }
    }

    /// Recursive helper that inserts a single key–value pair into the subtree rooted at `node`.
    ///
    /// Traverses down to the appropriate leaf, inserts the key/value there, and then propagates
    /// splits back up the call stack. If the current node overflows `order` keys, it is split
    /// and the middle key is *promoted* to the caller.
    ///
    /// Returns:
    /// - `None` if no split was necessary
    /// - `Some((promoted_key, new_sibling))` if this node was split and the parent must insert
    ///    `promoted_key` and attach `new_sibling` as a new child.
    fn insert_recursive(
        node: &mut Node<V>,
        key: u8,
        value: V,
        order: usize,
    ) -> Option<(u8, Box<Node<V>>)> {
        match node {
            Node::Leaf(leaf) => {
                leaf.add(key, value);

                if leaf.keys.len() > order {
                    let mid = order.div_ceil(2);
                    let right_keys = leaf.keys.split_off(mid);
                    let right_values = leaf.values.split_off(mid);

                    let promoted_key = right_keys[0];
                    let new_sibling = Box::new(Node::Leaf(LeafNode {
                        values: right_values,
                        keys: right_keys,
                    }));

                    return Some((promoted_key, new_sibling));
                }

                None
            }
            Node::Internal(internal) => {
                let idx = internal
                    .keys
                    .iter()
                    .position(|&k| key < k)
                    .unwrap_or(internal.keys.len());

                let child = internal.children[idx].as_mut();
                if let Some((promoted_key, new_child)) =
                    Self::insert_recursive(child, key, value, order)
                {
                    internal.keys.insert(idx, promoted_key);
                    internal.children.insert(idx + 1, new_child);

                    if internal.keys.len() > order {
                        let mid = order.div_ceil(2);

                        let right_keys = internal.keys.split_off(mid + 1);
                        let promoted_key = internal.keys.pop().unwrap();

                        let right_children = internal.children.split_off(mid + 1);

                        let new_sibling = Box::new(Node::Internal(InternalNode {
                            keys: right_keys,
                            children: right_children,
                        }));

                        Some((promoted_key, new_sibling))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }

    /// returns the value for a specific key, if found.
    #[allow(dead_code)]
    pub fn get(&self, key: u8) -> Option<&V> {
        // first find the corresponding leaf node.
        let leaf = self.find_leaf(&self.root, key);
        let position = leaf.keys.iter().position(|&k| key == k);

        match position {
            None => None,
            Some(index) => Some(&leaf.values[index]),
        }
    }

    /// Traverses the tree until it reaches the leaf that should contain `key`.
    /// This is mainly useful for debugging and visualisation.    
    pub fn find_leaf<'a>(&self, node: &'a Node<V>, key: u8) -> &'a LeafNode<V> {
        match node {
            Node::Leaf(leaf) => leaf,
            Node::Internal(internal) => {
                let idx = internal
                    .keys
                    .iter()
                    .position(|&k| key < k)
                    .unwrap_or(internal.keys.len());
                self.find_leaf(internal.children[idx].as_ref(), key)
            }
        }
    }

    /// Public entry point for inserts with an auto-generated key.
    ///
    /// Starts the recursive insert process at the root. If the root itself is split,
    /// this method creates a new internal root with two children (old root and new sibling).
    pub fn insert_value(&mut self, value: V) {
        let new_key = random::<u8>();

        if let Some((promoted_key, new_child)) =
            Self::insert_recursive(&mut self.root, new_key, value, self.order)
        {
            // root wurde gesplittet -> neuen Root anlegen
            let old_root = std::mem::replace(
                &mut self.root,
                Node::Leaf(LeafNode {
                    keys: vec![],
                    values: vec![],
                }),
            );
            self.root = Node::Internal(InternalNode {
                keys: vec![promoted_key],
                children: vec![Box::new(old_root), new_child],
            });
        }
    }
}

impl<V> LeafNode<V> {
    pub fn new(keys: Vec<u8>) -> Self {
        LeafNode {
            keys,
            values: Vec::new(),
        }
    }

    /// Inserts a key–value pair into this leaf, keeping `keys` and `values` sorted by key.
    pub fn add(&mut self, key: u8, value: V) {
        let insertion_idx = self
            .keys
            .iter()
            .position(|&k| key < k)
            .unwrap_or(self.keys.len());

        self.keys.insert(insertion_idx, key);
        self.values.insert(insertion_idx, value);
    }
}
