# B+ Tree Database Engine (Learning Project)

This repository contains a learning implementation of a **B+ tree** in Rust, as a building block for a simple database engine.  
The focus is on understanding data structures, tree balancing, and Rust ownership/borrowing in a non‑trivial setting, ultimately building toward a minimal relational database.

## Goals

- Learn how B+ trees work (internal nodes, leaf nodes, splits, promotion of separator keys).
- Practice Rust concepts:
  - Ownership and borrowing
  - Mutable references across recursive calls
  - Lifetimes for tree traversal
- Build a foundation for a toy database engine with:
  - Primary and secondary indexes
  - Basic query capabilities (insert, select, update, delete)
  - Simple transaction support
  - On-disk persistence (later)

## Current Features

- Generic `BPlusTree<V>` type with:
  - `Node::Internal` and `Node::Leaf` variants
  - Configurable `order` (maximum number of keys per node)
- Insertion with automatic rebalancing:
  - Recursive descent to the correct leaf
  - Split of leaf nodes when they overflow
  - Propagation of splits up the tree via a returned `(promoted_key, new_sibling)`
  - Split of internal nodes when they overflow
  - Creation of a new root when the old root splits
- Keys inside nodes are kept **sorted**, both in internal nodes and leaf nodes.

At the moment, keys are `u8` (randomly generated) and values are generic `V` (with `Debug + Clone` bounds).

## Data Structure Overview

BPlusTree<V>
└── root: Node<V>

Node<V> =
├── Internal(InternalNode<V>)
└── Leaf(LeafNode<V>)

InternalNode<V>
├── keys: Vec<u8>
└── children: Vec<Box<Node<V>>>

LeafNode<V>
├── keys: Vec<u8>
└── values: Vec<V>

- In `InternalNode`:
  - `keys` partition the key space into ranges.
  - For `keys = [k0, k1, k2]` and `children = [C0, C1, C2, C3]`:
    - `C0` contains keys `< k0`
    - `C1` contains keys `>= k0` and `< k1`
    - `C2` contains keys `>= k1` and `< k2`
    - `C3` contains keys `>= k2`
- In `LeafNode`:
  - `keys[i]` is the key for `values[i]`.
  - Both vectors are kept in sorted order by key.

## Insertion Algorithm

The core of the insertion logic is implemented in a recursive helper:
fn insert_recursive(
node: &mut Node<V>,
key: u8,
value: V,
order: usize,
) -> Option<(u8, Box<Node<V>>)>

High‑level behavior:

1. **Descent**:
   - If `node` is an `Internal` node, choose the appropriate child based on the separator keys and recurse.
   - If `node` is a `Leaf`, insert the key/value into the sorted position in that leaf.

2. **Split**:
   - If a leaf overflows (`keys.len() > order`), it is split into two leaves:
     - The left part stays in the existing node.
     - The right part becomes a new sibling node.
     - The first key of the right node is _promoted_ to the parent.
   - If an internal node overflows, it is similarly split:
     - One middle key is promoted to the parent.
     - Left and right siblings each keep their own subset of keys and children.

3. **Propagation**:
   - The result of `insert_recursive` is:
     - `None` if no split occurred.
     - `Some((promoted_key, new_sibling))` if the current node was split.
   - Parents receive this and update their own `keys` and `children` accordingly.
   - If the root reports a split, `insert_value` creates a **new root internal node** with two children: the old root and the new sibling.

Public entry point:
pub fn insert_value(&mut self, value: V)

This generates a random `u8` key and starts the recursive insertion at the root.

## TODO List

### Phase 1: Core B+ Tree Operations ✅ (Partially Complete)

- [x] B+ Tree data structures (`InternalNode`, `LeafNode`)
- [x] Insert with recursive splitting
- [x] Root split handling
- [ ] **Get operation** (`get(&self, key: u8) -> Option<&V>`)
- [ ] **Update operation** (update value for existing key)
- [ ] **Delete operation** (with merge/rebalance logic)
- [ ] Handle duplicate keys (overwrite vs. reject)
- [ ] Configurable key type (beyond `u8`)
- [ ] Unit tests for insert, get, update, delete

### Phase 2: Database Table Layer

- [ ] Define `Table` struct with schema (column names, types)
- [ ] Support multiple columns per row
- [ ] Primary key as B+ Tree index (auto-generated or user-provided)
- [ ] Insert row: `table.insert(row_data) -> Result<RowId, Error>`
- [ ] Get row by primary key: `table.get(id) -> Option<Row>`
- [ ] Update row by primary key
- [ ] Delete row by primary key
- [ ] Basic validation (type checking, null constraints)

### Phase 3: Secondary Indexes

- [ ] Create secondary B+ Tree for non-primary key columns
- [ ] Secondary index stores: `(indexed_column_value → primary_key)`
- [ ] Query by secondary index (e.g., find user by email)
- [ ] Keep secondary indexes in sync with inserts/updates/deletes
- [ ] Support multiple secondary indexes per table

### Phase 4: Query Interface

- [ ] Simple query API:
  - `SELECT * FROM table WHERE column = value`
  - `UPDATE table SET column = value WHERE id = x`
  - `DELETE FROM table WHERE id = x`
- [ ] Range queries (e.g., `WHERE age > 18 AND age < 65`)
- [ ] Iterate over all rows in a table
- [ ] Join support (simple nested loop join between two tables)

### Phase 5: Transaction Support

- [ ] Basic transaction struct (`Transaction`)
- [ ] Begin/commit/rollback semantics
- [ ] In-memory undo log for rollback
- [ ] Isolation: serialize transactions (no concurrency for now)
- [ ] ACID guarantees (simplified version)

### Phase 6: Persistence (Optional)

- [ ] Serialize B+ Tree nodes to disk
- [ ] Page-based storage format
- [ ] Write-ahead log (WAL) for crash recovery
- [ ] Load existing database from disk on startup
- [ ] Flush in-memory changes to disk on commit

### Phase 7: Testing & Documentation

- [ ] Comprehensive unit tests for all operations
- [ ] Integration tests (multi-table scenarios)
- [ ] Performance benchmarks (insert, lookup, range scan)
- [ ] Documentation: architecture overview, API examples
- [ ] Example usage: simple CLI or web server demo

### Phase 8: Polish & Extensions (Stretch Goals)

- [ ] SQL parser (basic subset: SELECT, INSERT, UPDATE, DELETE)
- [ ] Query optimizer (choose best index)
- [ ] Concurrent access (readers-writer locks or MVCC)
- [ ] B+ Tree statistics (depth, node count, fill factor)
- [ ] Visualization tool for tree structure (graphviz or web UI)

## Running

Standard Rust workflow:
git clone <this-repo-url>
cd <project-directory>
cargo build
cargo test # once tests are added
cargo run # if a binary is provided

## Project Structure (Planned)

src/
├── btree/
│ ├── mod.rs # B+ Tree implementation
│ ├── node.rs # Node, InternalNode, LeafNode
│ └── tests.rs # Unit tests for B+ Tree
├── table/
│ ├── mod.rs # Table abstraction
│ ├── row.rs # Row representation
│ └── schema.rs # Column definitions
├── index/
│ ├── mod.rs # Secondary indexes
│ └── index_manager.rs
├── query/
│ ├── mod.rs # Query execution
│ └── parser.rs # (optional) SQL parsing
├── transaction/
│ ├── mod.rs # Transaction logic
│ └── undo_log.rs # Rollback support
├── storage/
│ ├── mod.rs # Disk persistence (optional)
│ └── page.rs # Page-based storage
└── main.rs # CLI or demo

## License

MIT

## Acknowledgments

This project is a learning exercise inspired by:

- _Database Internals_ by Alex Petrov
- SQLite architecture
- Various B+ Tree tutorials and Rust ownership deep-dives

Feedback, issues, and pull requests welcome!
