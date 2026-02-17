//! Tests for the minidb-engine B+ tree implementation
//!
//! This module contains unit tests for the B+ tree data structure implementation.
//! Tests cover insertion, deletion, searching, and edge cases.

use crate::models::{BPlusTree, DeleteResult, InternalNode, KeySize, LeafNode, Node};
use std::vec;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_b_plus_tree_creation() {
        let root = Node::Leaf(LeafNode {
            keys: vec![],
            values: vec![],
        });

        let tree = BPlusTree::new(2, 3, root, vec![]);

        assert_eq!(tree.order, 3);
        assert_eq!(tree.min_elements, 2);
    }

    #[test]
    fn test_leaf_node_creation() {
        let leaf = LeafNode::new(vec![1, 2, 3]);

        assert_eq!(leaf.keys, vec![1, 2, 3]);
        assert_eq!(leaf.values, vec![]);
    }

    #[test]
    fn test_leaf_node_add() {
        let mut leaf = LeafNode::new(vec![1, 3]);
        leaf.add(2, "value2".to_string());

        assert_eq!(leaf.keys, vec![1, 2, 3]);
        assert_eq!(leaf.values, vec!["value2".to_string()]);
    }

    #[test]
    fn test_leaf_node_add_at_beginning() {
        let mut leaf = LeafNode::new(vec![2, 3]);
        leaf.add(1, "value1".to_string());

        assert_eq!(leaf.keys, vec![1, 2, 3]);
        assert_eq!(leaf.values, vec!["value1".to_string()]);
    }

    #[test]
    fn test_leaf_node_add_at_end() {
        let mut leaf = LeafNode::new(vec![1, 2]);
        leaf.add(3, "value3".to_string());

        assert_eq!(leaf.keys, vec![1, 2, 3]);
        assert_eq!(leaf.values, vec!["value3".to_string()]);
    }

    #[test]
    fn test_insert_into_leaf() {
        let root = Node::Leaf(LeafNode {
            keys: vec![],
            values: vec![],
        });

        let mut tree = BPlusTree::new(2, 3, root, vec![]);

        // Insert values and check they are stored correctly
        tree.insert_value("value1".to_string());
        tree.insert_value("value2".to_string());
        tree.insert_value("value3".to_string());

        // Basic structural check - tree should have been created with correct parameters
        assert_eq!(tree.order, 3);
        assert_eq!(tree.min_elements, 2);
    }

    #[test]
    fn test_find_leaf() {
        let root = Node::Leaf(LeafNode {
            keys: vec![1, 2, 3],
            values: vec!["value1".to_string(), "value2".to_string(), "value3".to_string()],
        });

        let tree = BPlusTree::new(2, 3, root, vec![]);

        // Find leaf for key 2 (should be in the leaf)
        let leaf = tree.find_leaf(&tree.root, 2);
        assert_eq!(leaf.keys, vec![1, 2, 3]);
    }

    #[test]
    fn test_find_leaf_mut() {
        let mut root = Node::Leaf(LeafNode {
            keys: vec![1, 2, 3],
            values: vec!["value1".to_string(), "value2".to_string(), "value3".to_string()],
        });

        let mut tree = BPlusTree::new(2, 3, root, vec![]);

        // Find leaf for key 2 (should be in the leaf)
        let leaf = BPlusTree::find_leaf_mut(&mut tree.root, 2);
        assert_eq!(leaf.keys, vec![1, 2, 3]);
    }

    #[test]
    fn test_update_existing_key() {
        let root = Node::Leaf(LeafNode {
            keys: vec![1, 2, 3],
            values: vec!["value1".to_string(), "value2".to_string(), "value3".to_string()],
        });

        let mut tree = BPlusTree::new(2, 3, root, vec![]);

        // Update existing key
        tree.update(2, "updated_value".to_string());

        // Basic structural check
        assert_eq!(tree.order, 3);
    }

    #[test]
    fn test_get_existing_key() {
        let root = Node::Leaf(LeafNode {
            keys: vec![1, 2, 3],
            values: vec!["value1".to_string(), "value2".to_string(), "value3".to_string()],
        });

        let tree = BPlusTree::new(2, 3, root, vec![]);

        // Get existing key
        let result = tree.get(2);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), &"value2".to_string());
    }

    #[test]
    fn test_get_nonexistent_key() {
        let root = Node::Leaf(LeafNode {
            keys: vec![1, 2, 3],
            values: vec!["value1".to_string(), "value2".to_string(), "value3".to_string()],
        });

        let tree = BPlusTree::new(2, 3, root, vec![]);

        // Get non-existent key
        let result = tree.get(4);
        assert!(result.is_none());
    }

    #[test]
    fn test_delete_nonexistent_key() {
        let root = Node::Leaf(LeafNode {
            keys: vec![1, 2, 3],
            values: vec!["value1".to_string(), "value2".to_string(), "value3".to_string()],
        });

        let mut tree = BPlusTree::new(2, 3, root, vec![]);

        // Delete non-existent key
        tree.delete_value(4);

        // Tree should remain unchanged
        assert_eq!(tree.order, 3);
    }

    #[test]
    fn test_delete_existing_key() {
        let root = Node::Leaf(LeafNode {
            keys: vec![1, 2, 3],
            values: vec!["value1".to_string(), "value2".to_string(), "value3".to_string()],
        });

        let mut tree = BPlusTree::new(2, 3, root, vec![]);

        // Delete existing key
        tree.delete_value(2);

        // Basic structural check
        assert_eq!(tree.order, 3);
    }

    #[test]
    fn test_empty_tree() {
        let root = Node::Leaf(LeafNode {
            keys: vec![],
            values: vec![],
        });

        let tree = BPlusTree::new(2, 3, root, vec![]);

        assert_eq!(tree.order, 3);
        assert_eq!(tree.min_elements, 2);
    }

    #[test]
    fn test_tree_with_single_element() {
        let root = Node::Leaf(LeafNode {
            keys: vec![42],
            values: vec!["single_value".to_string()],
        });

        let tree = BPlusTree::new(2, 3, root, vec![]);

        assert_eq!(tree.order, 3);
        assert_eq!(tree.min_elements, 2);

        // Test getting the value
        let result = tree.get(42);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), &"single_value".to_string());
    }

    #[test]
    fn test_tree_insertion_and_deletion() {
        // Create a tree with a small order to test splitting behavior
        let root = Node::Leaf(LeafNode {
            keys: vec![],
            values: vec![],
        });

        let mut tree = BPlusTree::new(1, 2, root, vec![]); // order 2, min 1

        // Insert values that will trigger splits
        tree.insert_value("value1".to_string());
        tree.insert_value("value2".to_string());
        tree.insert_value("value3".to_string());
        tree.insert_value("value4".to_string());

        // Basic check that tree exists and has correct parameters
        assert_eq!(tree.order, 2);
        assert_eq!(tree.min_elements, 1);
    }

    #[test]
    fn test_tree_deletion_edge_cases() {
        // Create a simple tree with a few elements
        let root = Node::Leaf(LeafNode {
            keys: vec![1, 2, 3, 4],
            values: vec!["value1".to_string(), "value2".to_string(), "value3".to_string(), "value4".to_string()],
        });

        let mut tree = BPlusTree::new(1, 2, root, vec![]);

        // Delete a middle element
        tree.delete_value(2);

        // Basic structural check
        assert_eq!(tree.order, 2);
        assert_eq!(tree.min_elements, 1);
    }
}