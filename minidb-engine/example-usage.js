// Example usage of minidb-engine in Next.js/JavaScript
// This file demonstrates how to use the WASM-compiled B+ tree

import init, { BPlusTreeWrapper, createDemoTree } from './pkg/minidb_engine';

// Initialize the WASM module
await init();

// Example 1: Create and use a tree
const tree = new BPlusTreeWrapper(4); // order = 4

// Insert some data (any JSON-serializable values)
tree.insert({ name: "Alice", age: 30, role: "Engineer" });
tree.insert({ name: "Bob", age: 25, role: "Designer" });
tree.insert({ name: "Charlie", age: 35, role: "Manager" });

// Update a value (you need to know the key)
tree.update(someKey, { name: "Alice", age: 31, role: "Senior Engineer" });

// Delete a value
tree.delete(someKey);

// Get the entire tree structure for visualization
const treeStructure = tree.toJS();
console.log("Tree structure:", treeStructure);

// Example 2: Use the demo tree helper
const demoTree = createDemoTree(4, 20);
const demoStructure = demoTree.toJS();
console.log("Demo tree:", demoStructure);

// Example 3: Serialize and deserialize
const serialized = tree.toJS();
// ... save to localStorage, send to server, etc.

// Later, restore the tree
const restoredTree = BPlusTreeWrapper.fromJS(serialized);

// Example 4: Access tree properties
console.log("Order:", tree.order);
console.log("Min elements:", tree.minElements);
