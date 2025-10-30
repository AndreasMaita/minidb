
pub struct BPlusTree<K, V> {
    pub root: Node<K, V>,
    order: usize,
}

pub enum Node<K, V> {
    Internal(InternalNode<K, V>),
    Leaf(LeafNode<V>)
}

pub struct InternalNode<K, V> {
    pub keys: Vec<K>,
    pub children: Vec<Box<Node<K, V>>>,
}

pub struct LeafNode<V> {
    pub values: Vec<V>,

}

impl<K, V> BPlusTree<K, V>
where K: std::fmt::Debug, 
V: std::fmt::Debug {
    pub fn new(order: usize, root: Node<K, V>) -> Self {
        // implelemntation
        BPlusTree {order, root}
    }


    pub fn insert(&mut self, key: i32, value: String) {

    }

        // Changed from self to &self - don't consume the tree!
    pub fn print(&self) {
        println!("B+ Tree (order: {})", self.order);
        self.print_node(&self.root, 0);
    }

    // Helper function to recursively print nodes
    fn print_node(&self, node: &Node<K, V>, level: usize) {
        let indent = "  ".repeat(level);
        
        match node {
            Node::Internal(internal) => {
                println!("{}Internal Node:", indent);
                println!("{}  Keys: {:?}", indent, internal.keys);
                println!("{}  Children ({}): ", indent, internal.children.len());
                
                for (i, child) in internal.children.iter().enumerate() {
                    println!("{}  Child {}:", indent, i);
                    self.print_node(child, level + 2);
                }
            }
            Node::Leaf(leaf) => {
                println!("{}Leaf Node:", indent);
                println!("{}  Values: {:?}", indent, leaf.values);
            }
        }
    }
}

impl<K, V> InternalNode<K, V> {
    pub fn new(keys: Vec<K>) -> Self {
        InternalNode { keys, children: Vec::new()}
    }

    pub fn add_child(&mut self, child: Box<Node<K, V>>) {
        self.children.push(child);
    }
}
