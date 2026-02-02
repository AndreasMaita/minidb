pub mod models;

use crate::models::{BPlusTree, DeleteResult, InternalNode, KeySize, LeafNode, Node};
use rand::random;
use std::vec;

impl<V> BPlusTree<V>
where
    V: std::fmt::Debug + Clone,
{
    pub fn new(min_elements: usize, order: usize, root: Node<V>, arena: Vec<Node<V>>) -> Self {
        BPlusTree {
            order,
            root,
            arena,
            min_elements,
        }
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
        key: KeySize,
        value: V,
        order: usize,
    ) -> Option<(KeySize, Box<Node<V>>)> {
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

    fn delete_recursive(
        node: &mut Node<V>,
        key: KeySize,
        order: usize,
        min_elements: usize,
    ) -> DeleteResult<KeySize> {
        match node {
            Node::Leaf(leaf) => {
                let pos = leaf.keys.iter().position(|&k| k == key).unwrap();
                leaf.keys.remove(pos);
                leaf.values.remove(pos);

                if leaf.keys.is_empty() {
                    // Zwischenstand: kein Merge/Borrow -> Parent macht NICHTS
                    return DeleteResult::Empty;
                }
                if pos == 0 {
                    if leaf.keys.len() < min_elements {
                        // space in keys is less than allowed
                        // should borrow from sibling
                        return DeleteResult::Underflow {
                            new_min_opt: Option::Some(leaf.keys[0]),
                        };
                    }

                    // Min-Key des Leafs hat sich geändert
                    return DeleteResult::MinChanged {
                        // child_index füllt später der Parent
                        new_min: leaf.keys[0],
                    };
                }

                DeleteResult::Ok
            }
            Node::Internal(internal) => {
                let idx = internal
                    .keys
                    .iter()
                    .position(|&k| key < k)
                    .unwrap_or(internal.keys.len());

                let child = internal.children[idx].as_mut();

                let result = Self::delete_recursive(child, key, order, min_elements);

                match result {
                    DeleteResult::Empty => {
                        if idx > 0 {
                            println!("removing the key {}", idx - 1);
                            internal.keys.remove(idx - 1);
                            println!("removing the child {}", idx);
                        }
                        internal.children.remove(idx);

                        // reassign the internal node to the only child it has left
                        if internal.children.len() == 1 && internal.keys.len() == 1 {
                            let only_child = *internal.children.remove(0);
                            *node = only_child;

                            return DeleteResult::Ok;
                        }

                        internal.keys
                        return DeleteResult::Ok;
                    }
                    DeleteResult::MinChanged { new_min } => {
                        if idx > 0 {
                            internal.keys[idx - 1] = new_min;
                        }
                        return DeleteResult::Ok;
                    }
                    DeleteResult::NotFound => DeleteResult::NotFound,
                    DeleteResult::Ok => DeleteResult::Ok,
                    DeleteResult::Underflow { new_min_opt } => match new_min_opt {
                        Option::None => {
                            // Underflow, try to borrow one from the sibling
                            DeleteResult::Ok
                        }
                        Option::Some(new_min) => {
                            // Underflow, and assign new min
                            if idx > 0 {
                                internal.keys[idx - 1] = new_min;
                            }

                            // now try to borrow from sibling
                            DeleteResult::Ok
                        }
                    },
                }
            }
        }
    }

    pub fn update(&mut self, key: KeySize, value: V) {
        let leaf = Self::find_leaf_mut(&mut self.root, key);
        let position = leaf.keys.iter().position(|&k| key == k);

        match position {
            None => None,
            Some(index) => Some(leaf.values[index] = value),
        };
    }

    /// returns the value for a specific key, if found.
    #[allow(dead_code)]
    pub fn get(&self, key: KeySize) -> Option<&V> {
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
    pub fn find_leaf_mut<'a>(node: &'a mut Node<V>, key: KeySize) -> &'a mut LeafNode<V> {
        match node {
            Node::Leaf(leaf) => leaf,
            Node::Internal(internal) => {
                let idx = internal
                    .keys
                    .iter()
                    .position(|&k| key < k)
                    .unwrap_or(internal.keys.len());
                Self::find_leaf_mut(internal.children[idx].as_mut(), key)
            }
        }
    }

    /// Traverses the tree until it reaches the leaf that should contain `key`.
    /// This is mainly useful for debugging and visualisation.    
    pub fn find_leaf<'a>(&self, node: &'a Node<V>, key: KeySize) -> &'a LeafNode<V> {
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

    pub fn delete_value(&mut self, key: KeySize) {
        Self::delete_recursive(&mut self.root, key, self.order, self.min_elements);
    }

    /// Public entry point for inserts with an auto-generated key.
    ///
    /// Starts the recursive insert process at the root. If the root itself is split,
    /// this method creates a new internal root with two children (old root and new sibling).
    pub fn insert_value(&mut self, value: V) {
        let new_key = random::<KeySize>();

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
    pub fn new(keys: Vec<KeySize>) -> Self {
        LeafNode {
            keys,
            values: Vec::new(),
        }
    }

    /// Inserts a key–value pair into this leaf, keeping `keys` and `values` sorted by key.
    pub fn add(&mut self, key: KeySize, value: V) {
        let insertion_idx = self
            .keys
            .iter()
            .position(|&k| key < k)
            .unwrap_or(self.keys.len());

        self.keys.insert(insertion_idx, key);
        self.values.insert(insertion_idx, value);
    }
}
