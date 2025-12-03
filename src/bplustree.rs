use std::{ops::Div, vec};

use rand::random;

pub struct BPlusTree<V> {
    pub root: Node<V>,
    order: usize,
}

pub enum Node<V> {
    Internal(InternalNode<V>),
    Leaf(LeafNode<V>),
}

pub struct InternalNode<V> {
    // die Verknüpfung zwischen den werten ist durch die keys und die position in den childrens
    // gegeben.
    pub keys: Vec<u8>,
    pub children: Vec<Box<Node<V>>>,
}

pub struct LeafNode<V> {
    pub values: Vec<V>,
    pub keys: Vec<u8>,
}

impl<V> BPlusTree<V>
where
    V: std::fmt::Debug + Clone,
{
    pub fn new(order: usize, root: Node<V>) -> Self {
        // implelemntation
        BPlusTree { order, root }
    }

    fn insert_recursive(
        node: &mut Node<V>,
        key: u8,
        value: V,
        order: usize,
    ) -> Option<(u8, Box<Node<V>>)> {
        match node {
            Node::Leaf(leaf) => {
                // find the correct node to add the new_key
                leaf.add(key, value);

                if leaf.keys.len() > order {
                    // leaf is full, need to split and promote the key
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

    /// this method is only for debugging purposes or other visual possibilities to show
    pub fn find_leaf<'a>(&mut self, node: &'a mut Node<V>, key: u8) -> &'a mut LeafNode<V> {
        match node {
            Node::Leaf(leaf) => leaf,
            Node::Internal(internal) => {
                let idx = internal
                    .keys
                    .iter()
                    .position(|&k| key < k)
                    .unwrap_or(internal.keys.len());
                self.find_leaf(internal.children[idx].as_mut(), key)
            }
        }
    }

    pub fn insert_value(&mut self, value: V) {
        // First, check if we need to split
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

    // Changed from self to &self - don't consume the tree!
    pub fn print(&self) {
        println!("B+ Tree (order: {})", self.order);
        self.print_node(&self.root, 0);
    }

    // Helper function to recursively print nodes
    fn print_node(&self, node: &Node<V>, level: usize) {
        let indent = "  ".repeat(level);

        match node {
            Node::Internal(internal) => {
                println!("{}Internal Node:", indent);
                println!("{}  Children ({}): ", indent, internal.children.len());
                println!("{}  Keys: {:?}", indent, internal.keys);

                for (i, child) in internal.children.iter().enumerate() {
                    println!("{}  Child {}:", indent, i);
                    self.print_node(child, level + 2);
                }
            }
            Node::Leaf(leaf) => {
                println!("{}Leaf Node:", indent);
                println!("{}  Values: {:?}", indent, leaf.values);
                println!("{}  Keys: {:?}", indent, leaf.keys);
            }
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

impl<V> InternalNode<V> {
    pub fn new(keys: Vec<u8>) -> Self {
        InternalNode {
            keys,
            children: Vec::new(),
        }
    }

    pub fn add(&mut self, key: u8, child: Box<Node<V>>) {
        let insertion_idx = self
            .keys
            .iter()
            .position(|&k| key < k)
            .unwrap_or(self.keys.len());

        self.keys.insert(insertion_idx, key);
        self.children.insert(insertion_idx, child);
    }
}
