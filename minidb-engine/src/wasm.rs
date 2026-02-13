//! WASM-specific wrapper module for JavaScript interop.
//!
//! This module provides a JavaScript-compatible API by creating concrete
//! type instantiations of the generic BPlusTree with String as the value type.
//! JavaScript values are serialized to JSON strings for storage.

use crate::models::{BPlusTree, LeafNode, Node};
use wasm_bindgen::prelude::*;

/// Concrete type alias for BPlusTree that stores JSON strings
/// This allows JavaScript to store any JSON-serializable data
type WasmBPlusTree = BPlusTree<String>;
type WasmNode = Node<String>;
type WasmLeafNode = LeafNode<String>;

/// JavaScript-compatible wrapper for BPlusTree
#[wasm_bindgen]
pub struct BPlusTreeWrapper {
    inner: WasmBPlusTree,
}

#[wasm_bindgen]
impl BPlusTreeWrapper {
    /// Create a new B+ tree with the specified order
    ///
    /// # Arguments
    /// * `order` - Maximum number of keys per node
    #[wasm_bindgen(constructor)]
    pub fn new(order: usize) -> Self {
        let min_elements = order / 2;
        let root = WasmNode::Leaf(WasmLeafNode {
            keys: vec![],
            values: vec![],
        });

        Self {
            inner: WasmBPlusTree {
                order,
                root,
                arena: vec![],
                min_elements,
            },
        }
    }

    /// Insert a value into the tree with an auto-generated key
    ///
    /// # Arguments
    /// * `value` - Any JavaScript value (will be JSON-serialized)
    #[wasm_bindgen]
    pub fn insert(&mut self, value: JsValue) -> Result<(), JsValue> {
        let json_string = js_sys::JSON::stringify(&value)
            .map_err(|e| JsValue::from_str(&format!("Failed to stringify value: {:?}", e)))?
            .as_string()
            .ok_or_else(|| JsValue::from_str("Stringify returned null"))?;

        self.inner.insert_value(json_string);
        Ok(())
    }

    /// Get a value by key
    ///
    /// # Arguments
    /// * `key` - The key to look up
    ///
    /// # Returns
    /// The value as a JavaScript object, or undefined if not found
    #[wasm_bindgen]
    pub fn get(&self, key: u32) -> Result<JsValue, JsValue> {
        match self.inner.get(key) {
            Some(json_string) => js_sys::JSON::parse(json_string)
                .map_err(|e| JsValue::from_str(&format!("Failed to parse JSON: {:?}", e))),
            None => Ok(JsValue::UNDEFINED),
        }
    }

    /// Update a value at a specific key
    ///
    /// # Arguments
    /// * `key` - The key to update
    /// * `value` - The new value
    #[wasm_bindgen]
    pub fn update(&mut self, key: u32, value: JsValue) -> Result<(), JsValue> {
        let json_string = js_sys::JSON::stringify(&value)
            .map_err(|e| JsValue::from_str(&format!("Failed to stringify value: {:?}", e)))?
            .as_string()
            .ok_or_else(|| JsValue::from_str("Stringify returned null"))?;

        self.inner.update(key, json_string);
        Ok(())
    }

    /// Delete a value at a specific key
    ///
    /// # Arguments
    /// * `key` - The key to delete
    #[wasm_bindgen]
    pub fn delete(&mut self, key: u32) {
        self.inner.delete_value(key);
    }

    /// Serialize the entire tree to a JavaScript object
    /// This allows you to inspect the tree structure from JavaScript
    #[wasm_bindgen(js_name = toJS)]
    pub fn to_js(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.inner)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Create a tree from a JavaScript object
    /// Useful for hydrating a tree from saved state
    #[wasm_bindgen(js_name = fromJS)]
    pub fn from_js(js_value: JsValue) -> Result<BPlusTreeWrapper, JsValue> {
        let inner: WasmBPlusTree = serde_wasm_bindgen::from_value(js_value)
            .map_err(|e| JsValue::from_str(&format!("Deserialization error: {}", e)))?;
        Ok(Self { inner })
    }

    /// Get the order of the tree
    #[wasm_bindgen(getter)]
    pub fn order(&self) -> usize {
        self.inner.order
    }

    /// Get the minimum elements per node
    #[wasm_bindgen(getter, js_name = minElements)]
    pub fn min_elements(&self) -> usize {
        self.inner.min_elements
    }
}

/// Helper function to create a demo tree with some sample data
/// Useful for testing and visualization
#[wasm_bindgen(js_name = createDemoTree)]
pub fn create_demo_tree(order: usize, num_items: usize) -> BPlusTreeWrapper {
    let mut tree = BPlusTreeWrapper::new(order);

    for i in 0..num_items {
        let value = JsValue::from_str(&format!("Item {}", i));
        let _ = tree.insert(value);
    }

    tree
}
